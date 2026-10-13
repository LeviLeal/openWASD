#![allow(dead_code)]
use crate::gamepad_types::VirtualGamepadButton;
use crate::virtual_gamepad::VirtualGamepad;
use crossbeam_channel::{Receiver, Sender};
use evdev::{Device, EventType, KeyCode};
use std::collections::HashMap;
use std::fs;
use std::os::fd::AsRawFd;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputDeviceInfo {
    pub path: String,
    pub name: String,
    pub is_keyboard: bool,
}

#[derive(Debug, Clone)]
pub enum InterceptorEvent {
    Started,
    Stopped,
    PermissionError { path: String, message: String },
    LiveKeyEvent { evdev_code: u16, pressed: bool },
    Log(String),
}

pub struct KeyboardInterceptor {
    running: Arc<AtomicBool>,
    paused: Arc<AtomicBool>,
    mappings: Arc<Mutex<HashMap<u16, Vec<VirtualGamepadButton>>>>,
    thread_handle: Option<JoinHandle<()>>,
    event_receiver: Receiver<InterceptorEvent>,
    event_sender: Sender<InterceptorEvent>,
}

impl KeyboardInterceptor {
    pub fn new() -> Self {
        let (event_sender, event_receiver) = crossbeam_channel::unbounded();

        Self {
            running: Arc::new(AtomicBool::new(false)),
            paused: Arc::new(AtomicBool::new(false)),
            mappings: Arc::new(Mutex::new(HashMap::new())),
            thread_handle: None,
            event_receiver,
            event_sender,
        }
    }

    pub fn event_receiver(&self) -> &Receiver<InterceptorEvent> {
        &self.event_receiver
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    pub fn is_paused(&self) -> bool {
        self.paused.load(Ordering::SeqCst)
    }

    pub fn set_paused(&self, paused: bool) {
        self.paused.store(paused, Ordering::SeqCst);
    }

    pub fn update_mappings(&self, new_mappings: HashMap<u16, Vec<VirtualGamepadButton>>) {
        if let Ok(mut map) = self.mappings.lock() {
            *map = new_mappings;
        }
    }

    pub fn detect_keyboards() -> Vec<InputDeviceInfo> {
        let mut list = Vec::new();

        // 1. Verificar em /dev/input/by-id
        let by_id = Path::new("/dev/input/by-id");
        if by_id.exists() {
            if let Ok(entries) = fs::read_dir(by_id) {
                for entry in entries.flatten() {
                    let file_name = entry.file_name().to_string_lossy().to_string();
                    if file_name.contains("-kbd") || file_name.contains("event-kbd") {
                        let path = entry.path().to_string_lossy().to_string();
                        let display_name = match Device::open(&path) {
                            Ok(dev) => dev.name().unwrap_or(&file_name).to_string(),
                            Err(_) => file_name.clone(),
                        };
                        list.push(InputDeviceInfo {
                            path,
                            name: display_name,
                            is_keyboard: true,
                        });
                    }
                }
            }
        }

        // 2. Se vazio, procurar em /dev/input/event*
        if list.is_empty() {
            let input_dir = Path::new("/dev/input");
            if let Ok(entries) = fs::read_dir(input_dir) {
                for entry in entries.flatten() {
                    let file_name = entry.file_name().to_string_lossy().to_string();
                    if file_name.starts_with("event") {
                        let path = entry.path().to_string_lossy().to_string();
                        if let Ok(dev) = Device::open(&path) {
                            if let Some(keys) = dev.supported_keys() {
                                if keys.contains(KeyCode::KEY_A)
                                    && keys.contains(KeyCode::KEY_SPACE)
                                {
                                    let name = dev.name().unwrap_or(&file_name).to_string();
                                    list.push(InputDeviceInfo {
                                        path,
                                        name,
                                        is_keyboard: true,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        // 3. Fallback
        if list.is_empty() {
            list.push(InputDeviceInfo {
                path: "/dev/input/event3".to_string(),
                name: "Default Keyboard (/dev/input/event3)".to_string(),
                is_keyboard: true,
            });
        }

        list
    }

    pub fn start(
        &mut self,
        device_path: String,
        grab_enabled: bool,
        initial_mappings: HashMap<u16, Vec<VirtualGamepadButton>>,
    ) {
        if self.is_running() {
            return;
        }

        self.update_mappings(initial_mappings);
        self.running.store(true, Ordering::SeqCst);
        self.paused.store(false, Ordering::SeqCst);

        let running = Arc::clone(&self.running);
        let paused = Arc::clone(&self.paused);
        let mappings = Arc::clone(&self.mappings);
        let sender = self.event_sender.clone();

        self.thread_handle = Some(thread::spawn(move || {
            let mut virtual_gamepad = VirtualGamepad::new();
            if let Err(e) = virtual_gamepad.open_device("Microsoft X-Box 360 pad") {
                let _ = sender.send(InterceptorEvent::PermissionError {
                    path: "/dev/uinput".to_string(),
                    message: format!("Failed to create /dev/uinput virtual device: {}", e),
                });
                running.store(false, Ordering::SeqCst);
                let _ = sender.send(InterceptorEvent::Stopped);
                return;
            }

            let mut dev = match Device::open(&device_path) {
                Ok(d) => d,
                Err(e) => {
                    let _ = sender.send(InterceptorEvent::PermissionError {
                        path: device_path.clone(),
                        message: format!("Failed to open keyboard '{}': {}", device_path, e),
                    });
                    running.store(false, Ordering::SeqCst);
                    let _ = sender.send(InterceptorEvent::Stopped);
                    return;
                }
            };

            let fd = dev.as_raw_fd();
            unsafe {
                let flags = libc::fcntl(fd, libc::F_GETFL, 0);
                let _ = libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK);
            }

            let mut currently_grabbed = false;
            if grab_enabled {
                if let Ok(()) = dev.grab() {
                    currently_grabbed = true;
                }
            }

            let _ = sender.send(InterceptorEvent::Started);
            let _ = sender.send(InterceptorEvent::Log(format!(
                "Interceptor started on '{}' (Grab: {})",
                device_path, grab_enabled
            )));

            let mut pfd = libc::pollfd {
                fd,
                events: libc::POLLIN,
                revents: 0,
            };

            while running.load(Ordering::SeqCst) {
                let is_paused = paused.load(Ordering::SeqCst);

                if is_paused {
                    if currently_grabbed {
                        let _ = dev.ungrab();
                        currently_grabbed = false;
                    }
                    thread::sleep(Duration::from_millis(30));
                    continue;
                } else if grab_enabled && !currently_grabbed {
                    if let Ok(()) = dev.grab() {
                        currently_grabbed = true;
                    }
                }

                // Poll com timeout de 40ms para checar running flag responsivamente
                let ret = unsafe { libc::poll(&mut pfd, 1, 40) };
                if ret <= 0 {
                    continue;
                }

                if (pfd.revents & libc::POLLIN) != 0 {
                    match dev.fetch_events() {
                        Ok(events) => {
                            for ev in events {
                                if ev.event_type() == EventType::KEY {
                                    let ev_code = ev.code();
                                    let pressed = ev.value() == 1;
                                    let released = ev.value() == 0;

                                    if pressed || released {
                                        let buttons_to_trigger = {
                                            if let Ok(map) = mappings.lock() {
                                                map.get(&ev_code).cloned()
                                            } else {
                                                None
                                            }
                                        };

                                        if let Some(buttons) = buttons_to_trigger {
                                            for btn in buttons {
                                                virtual_gamepad.send_button(btn, pressed);
                                            }
                                        }

                                        let _ = sender.send(InterceptorEvent::LiveKeyEvent {
                                            evdev_code: ev_code,
                                            pressed,
                                        });
                                    }
                                }
                            }
                        }
                        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                            // Sem mais eventos no buffer não bloqueante
                        }
                        Err(_) => {
                            // Erro de I/O
                            break;
                        }
                    }
                }
            }

            if currently_grabbed {
                let _ = dev.ungrab();
            }

            virtual_gamepad.close_device();
            let _ = sender.send(InterceptorEvent::Stopped);
            let _ = sender.send(InterceptorEvent::Log("Interceptor stopped.".to_string()));
        }));
    }

    pub fn stop(&mut self) {
        if !self.is_running() {
            return;
        }

        self.running.store(false, Ordering::SeqCst);
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
    }
}

impl Drop for KeyboardInterceptor {
    fn drop(&mut self) {
        self.stop();
    }
}
