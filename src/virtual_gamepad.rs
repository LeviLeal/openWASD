#![allow(dead_code)]
use crate::gamepad_types::VirtualGamepadButton;
use evdev::uinput::VirtualDevice;
use evdev::{
    AbsInfo, AbsoluteAxisCode, AttributeSet, BusType, EventType, InputEvent, InputId, KeyCode,
    SynchronizationCode, UinputAbsSetup,
};
use std::io;

pub struct VirtualGamepad {
    device: Option<VirtualDevice>,

    // Estado analógico Left Stick (WASD)
    ls_up: bool,
    ls_down: bool,
    ls_left: bool,
    ls_right: bool,

    // Estado analógico Right Stick (Câmera)
    rs_up: bool,
    rs_down: bool,
    rs_left: bool,
    rs_right: bool,

    // Estado D-Pad
    dpad_up: bool,
    dpad_down: bool,
    dpad_left: bool,
    dpad_right: bool,
}

impl VirtualGamepad {
    pub fn new() -> Self {
        Self {
            device: None,
            ls_up: false,
            ls_down: false,
            ls_left: false,
            ls_right: false,
            rs_up: false,
            rs_down: false,
            rs_left: false,
            rs_right: false,
            dpad_up: false,
            dpad_down: false,
            dpad_left: false,
            dpad_right: false,
        }
    }

    pub fn is_open(&self) -> bool {
        self.device.is_some()
    }

    pub fn open_device(&mut self, device_name: &str) -> io::Result<()> {
        if self.device.is_some() {
            return Ok(());
        }

        let mut keys = AttributeSet::<KeyCode>::new();

        // Botões de Ação
        keys.insert(KeyCode::BTN_SOUTH);
        keys.insert(KeyCode::BTN_EAST);
        keys.insert(KeyCode::BTN_NORTH);
        keys.insert(KeyCode::BTN_WEST);

        // Bumpers e Gatilhos
        keys.insert(KeyCode::BTN_TL);
        keys.insert(KeyCode::BTN_TR);
        keys.insert(KeyCode::BTN_TL2);
        keys.insert(KeyCode::BTN_TR2);

        // Sticks e Menus
        keys.insert(KeyCode::BTN_SELECT);
        keys.insert(KeyCode::BTN_START);
        keys.insert(KeyCode::BTN_MODE);
        keys.insert(KeyCode::BTN_THUMBL);
        keys.insert(KeyCode::BTN_THUMBR);

        // D-Pad
        keys.insert(KeyCode::BTN_DPAD_UP);
        keys.insert(KeyCode::BTN_DPAD_DOWN);
        keys.insert(KeyCode::BTN_DPAD_LEFT);
        keys.insert(KeyCode::BTN_DPAD_RIGHT);

        // Configuração de Eixos Analógicos
        let stick_info = AbsInfo::new(0, -32768, 32767, 16, 128, 0);
        let trigger_info = AbsInfo::new(0, 0, 255, 0, 0, 0);
        let hat_info = AbsInfo::new(0, -1, 1, 0, 0, 0);

        let abs_x = UinputAbsSetup::new(AbsoluteAxisCode::ABS_X, stick_info);
        let abs_y = UinputAbsSetup::new(AbsoluteAxisCode::ABS_Y, stick_info);
        let abs_rx = UinputAbsSetup::new(AbsoluteAxisCode::ABS_RX, stick_info);
        let abs_ry = UinputAbsSetup::new(AbsoluteAxisCode::ABS_RY, stick_info);
        let abs_z = UinputAbsSetup::new(AbsoluteAxisCode::ABS_Z, trigger_info);
        let abs_rz = UinputAbsSetup::new(AbsoluteAxisCode::ABS_RZ, trigger_info);
        let abs_hat_x = UinputAbsSetup::new(AbsoluteAxisCode::ABS_HAT0X, hat_info);
        let abs_hat_y = UinputAbsSetup::new(AbsoluteAxisCode::ABS_HAT0Y, hat_info);

        // ID de controle oficial Xbox 360 para máxima compatibilidade no Linux/Proton/Steam
        let input_id = InputId::new(BusType::BUS_USB, 0x045e, 0x028e, 1);

        let virtual_dev = VirtualDevice::builder()?
            .name(device_name)
            .input_id(input_id)
            .with_keys(&keys)?
            .with_absolute_axis(&abs_x)?
            .with_absolute_axis(&abs_y)?
            .with_absolute_axis(&abs_rx)?
            .with_absolute_axis(&abs_ry)?
            .with_absolute_axis(&abs_z)?
            .with_absolute_axis(&abs_rz)?
            .with_absolute_axis(&abs_hat_x)?
            .with_absolute_axis(&abs_hat_y)?
            .build()?;

        self.device = Some(virtual_dev);
        self.reset_state();
        Ok(())
    }

    pub fn close_device(&mut self) {
        if self.device.is_some() {
            self.reset_state();
            self.device = None;
        }
    }

    fn emit(&mut self, type_: u16, code: u16, value: i32) {
        if let Some(dev) = &mut self.device {
            let event = InputEvent::new(type_, code, value);
            let _ = dev.emit(&[event]);
        }
    }

    pub fn send_key(&mut self, code: KeyCode, pressed: bool) {
        self.emit(EventType::KEY.0, code.0, if pressed { 1 } else { 0 });
    }

    pub fn send_axis(&mut self, axis: AbsoluteAxisCode, value: i32) {
        self.emit(EventType::ABSOLUTE.0, axis.0, value);
    }

    pub fn sync(&mut self) {
        self.emit(
            EventType::SYNCHRONIZATION.0,
            SynchronizationCode::SYN_REPORT.0,
            0,
        );
    }

    pub fn send_button(&mut self, button: VirtualGamepadButton, pressed: bool) {
        match button {
            VirtualGamepadButton::A => self.send_key(KeyCode::BTN_SOUTH, pressed),
            VirtualGamepadButton::B => self.send_key(KeyCode::BTN_EAST, pressed),
            VirtualGamepadButton::X => self.send_key(KeyCode::BTN_NORTH, pressed),
            VirtualGamepadButton::Y => self.send_key(KeyCode::BTN_WEST, pressed),

            VirtualGamepadButton::LB => self.send_key(KeyCode::BTN_TL, pressed),
            VirtualGamepadButton::RB => self.send_key(KeyCode::BTN_TR, pressed),

            VirtualGamepadButton::LT => {
                self.send_key(KeyCode::BTN_TL2, pressed);
                self.send_axis(AbsoluteAxisCode::ABS_Z, if pressed { 255 } else { 0 });
            }
            VirtualGamepadButton::RT => {
                self.send_key(KeyCode::BTN_TR2, pressed);
                self.send_axis(AbsoluteAxisCode::ABS_RZ, if pressed { 255 } else { 0 });
            }

            VirtualGamepadButton::DpadUp => {
                self.dpad_up = pressed;
                self.send_key(KeyCode::BTN_DPAD_UP, pressed);
                self.update_dpad();
            }
            VirtualGamepadButton::DpadDown => {
                self.dpad_down = pressed;
                self.send_key(KeyCode::BTN_DPAD_DOWN, pressed);
                self.update_dpad();
            }
            VirtualGamepadButton::DpadLeft => {
                self.dpad_left = pressed;
                self.send_key(KeyCode::BTN_DPAD_LEFT, pressed);
                self.update_dpad();
            }
            VirtualGamepadButton::DpadRight => {
                self.dpad_right = pressed;
                self.send_key(KeyCode::BTN_DPAD_RIGHT, pressed);
                self.update_dpad();
            }

            VirtualGamepadButton::L3 => self.send_key(KeyCode::BTN_THUMBL, pressed),
            VirtualGamepadButton::R3 => self.send_key(KeyCode::BTN_THUMBR, pressed),

            VirtualGamepadButton::Start => self.send_key(KeyCode::BTN_START, pressed),
            VirtualGamepadButton::Back => self.send_key(KeyCode::BTN_SELECT, pressed),
            VirtualGamepadButton::Guide => self.send_key(KeyCode::BTN_MODE, pressed),

            VirtualGamepadButton::LeftStickUp => {
                self.ls_up = pressed;
                self.update_left_stick();
            }
            VirtualGamepadButton::LeftStickDown => {
                self.ls_down = pressed;
                self.update_left_stick();
            }
            VirtualGamepadButton::LeftStickLeft => {
                self.ls_left = pressed;
                self.update_left_stick();
            }
            VirtualGamepadButton::LeftStickRight => {
                self.ls_right = pressed;
                self.update_left_stick();
            }

            VirtualGamepadButton::RightStickUp => {
                self.rs_up = pressed;
                self.update_right_stick();
            }
            VirtualGamepadButton::RightStickDown => {
                self.rs_down = pressed;
                self.update_right_stick();
            }
            VirtualGamepadButton::RightStickLeft => {
                self.rs_left = pressed;
                self.update_right_stick();
            }
            VirtualGamepadButton::RightStickRight => {
                self.rs_right = pressed;
                self.update_right_stick();
            }
        }

        self.sync();
    }

    fn update_left_stick(&mut self) {
        let mut y = 0;
        if self.ls_up && !self.ls_down {
            y = -32767;
        } else if self.ls_down && !self.ls_up {
            y = 32767;
        }

        let mut x = 0;
        if self.ls_left && !self.ls_right {
            x = -32767;
        } else if self.ls_right && !self.ls_left {
            x = 32767;
        }

        self.send_axis(AbsoluteAxisCode::ABS_X, x);
        self.send_axis(AbsoluteAxisCode::ABS_Y, y);
    }

    fn update_right_stick(&mut self) {
        let mut y = 0;
        if self.rs_up && !self.rs_down {
            y = -32767;
        } else if self.rs_down && !self.rs_up {
            y = 32767;
        }

        let mut x = 0;
        if self.rs_left && !self.rs_right {
            x = -32767;
        } else if self.rs_right && !self.rs_left {
            x = 32767;
        }

        self.send_axis(AbsoluteAxisCode::ABS_RX, x);
        self.send_axis(AbsoluteAxisCode::ABS_RY, y);
    }

    fn update_dpad(&mut self) {
        let mut hat_y = 0;
        if self.dpad_up && !self.dpad_down {
            hat_y = -1;
        } else if self.dpad_down && !self.dpad_up {
            hat_y = 1;
        }

        let mut hat_x = 0;
        if self.dpad_left && !self.dpad_right {
            hat_x = -1;
        } else if self.dpad_right && !self.dpad_left {
            hat_x = 1;
        }

        self.send_axis(AbsoluteAxisCode::ABS_HAT0X, hat_x);
        self.send_axis(AbsoluteAxisCode::ABS_HAT0Y, hat_y);
    }

    pub fn reset_state(&mut self) {
        self.ls_up = false;
        self.ls_down = false;
        self.ls_left = false;
        self.ls_right = false;

        self.rs_up = false;
        self.rs_down = false;
        self.rs_left = false;
        self.rs_right = false;

        self.dpad_up = false;
        self.dpad_down = false;
        self.dpad_left = false;
        self.dpad_right = false;

        self.send_axis(AbsoluteAxisCode::ABS_X, 0);
        self.send_axis(AbsoluteAxisCode::ABS_Y, 0);
        self.send_axis(AbsoluteAxisCode::ABS_RX, 0);
        self.send_axis(AbsoluteAxisCode::ABS_RY, 0);
        self.send_axis(AbsoluteAxisCode::ABS_Z, 0);
        self.send_axis(AbsoluteAxisCode::ABS_RZ, 0);
        self.send_axis(AbsoluteAxisCode::ABS_HAT0X, 0);
        self.send_axis(AbsoluteAxisCode::ABS_HAT0Y, 0);

        self.sync();
    }
}
