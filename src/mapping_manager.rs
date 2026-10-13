#![allow(dead_code)]
use crate::gamepad_types::{get_all_gamepad_buttons, VirtualGamepadButton};
use crate::key_codes::{evdev_to_key_name, qt_key_to_linux_evdev};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyBinding {
    pub evdev_code: u16,
    pub key_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ModernProfileFile {
    pub profile_name: String,
    pub bindings: HashMap<String, KeyBinding>,
}

#[derive(Debug, Clone, Deserialize)]
struct LegacyBindingItem {
    pub qt_key: Option<i32>,
    pub scan_code: Option<u64>,
    pub key_name: Option<String>,
    pub evdev_code: Option<u16>,
}

#[derive(Debug, Clone, Deserialize)]
struct LegacyProfileFile {
    pub profile_name: Option<String>,
    pub bindings: HashMap<String, LegacyBindingItem>,
}

pub struct MappingManager {
    bindings: HashMap<VirtualGamepadButton, KeyBinding>,
    current_profile_name: String,
    profiles_dir: PathBuf,
}

impl MappingManager {
    pub fn new() -> Self {
        let profiles_dir = match directories::BaseDirs::new() {
            Some(base) => {
                let new_dir = base.config_dir().join("openwasd").join("profiles");
                let old_dir = base.config_dir().join("wasd-anywhere").join("profiles");
                if !new_dir.exists() && old_dir.exists() {
                    let _ = fs::create_dir_all(&new_dir);
                    if let Ok(entries) = fs::read_dir(&old_dir) {
                        for entry in entries.flatten() {
                            let dest = new_dir.join(entry.file_name());
                            let _ = fs::copy(entry.path(), dest);
                        }
                    }
                }
                new_dir
            }
            None => PathBuf::from(".config/openwasd/profiles"),
        };

        let _ = fs::create_dir_all(&profiles_dir);

        let mut manager = Self {
            bindings: HashMap::new(),
            current_profile_name: "Default (WASD)".to_string(),
            profiles_dir,
        };

        manager.apply_default_fps_profile();
        manager
    }

    pub fn current_profile_name(&self) -> &str {
        &self.current_profile_name
    }

    pub fn set_current_profile_name(&mut self, name: String) {
        self.current_profile_name = name;
    }

    pub fn set_binding(&mut self, button: VirtualGamepadButton, evdev_code: u16, key_name: &str) {
        self.bindings.insert(
            button,
            KeyBinding {
                evdev_code,
                key_name: key_name.to_string(),
            },
        );
    }

    pub fn clear_binding(&mut self, button: VirtualGamepadButton) {
        self.bindings.remove(&button);
    }

    pub fn clear_all_bindings(&mut self) {
        self.bindings.clear();
    }

    pub fn has_binding(&self, button: VirtualGamepadButton) -> bool {
        self.bindings.contains_key(&button)
    }

    pub fn get_binding(&self, button: VirtualGamepadButton) -> Option<&KeyBinding> {
        self.bindings.get(&button)
    }

    pub fn get_all_bindings(&self) -> &HashMap<VirtualGamepadButton, KeyBinding> {
        &self.bindings
    }

    pub fn find_buttons_for_key(&self, evdev_code: u16) -> Vec<VirtualGamepadButton> {
        let mut result = Vec::new();
        for (button, binding) in &self.bindings {
            if binding.evdev_code == evdev_code {
                result.push(*button);
            }
        }
        result
    }

    pub fn apply_default_fps_profile(&mut self) {
        self.clear_all_bindings();
        self.current_profile_name = "Default (WASD)".to_string();

        // Left Stick -> WASD
        self.set_binding(VirtualGamepadButton::LeftStickUp, 17, "W");
        self.set_binding(VirtualGamepadButton::LeftStickDown, 31, "S");
        self.set_binding(VirtualGamepadButton::LeftStickLeft, 30, "A");
        self.set_binding(VirtualGamepadButton::LeftStickRight, 32, "D");

        // Action Buttons
        self.set_binding(VirtualGamepadButton::A, 57, "Space");
        self.set_binding(VirtualGamepadButton::B, 46, "C");
        self.set_binding(VirtualGamepadButton::X, 18, "E");
        self.set_binding(VirtualGamepadButton::Y, 16, "Q");

        // Bumpers
        self.set_binding(VirtualGamepadButton::LB, 33, "F");
        self.set_binding(VirtualGamepadButton::RB, 19, "R");

        // L3 / Sprint
        self.set_binding(VirtualGamepadButton::L3, 42, "Left Shift");

        // Menus
        self.set_binding(VirtualGamepadButton::Start, 15, "Tab");
        self.set_binding(VirtualGamepadButton::Back, 50, "M");
    }

    fn button_from_ordinal(idx: usize) -> Option<VirtualGamepadButton> {
        get_all_gamepad_buttons().get(idx).map(|info| info.id)
    }

    fn button_to_ordinal(button: VirtualGamepadButton) -> usize {
        get_all_gamepad_buttons()
            .iter()
            .position(|info| info.id == button)
            .unwrap_or(0)
    }

    pub fn save_profile(&mut self, profile_name: &str) -> io::Result<()> {
        let name = profile_name.trim();
        if name.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Profile name cannot be empty",
            ));
        }

        let _ = fs::create_dir_all(&self.profiles_dir);

        let mut bindings_map = HashMap::new();
        for (button, binding) in &self.bindings {
            let key = format!("{:?}", button);
            bindings_map.insert(key, binding.clone());
        }

        let modern = ModernProfileFile {
            profile_name: name.to_string(),
            bindings: bindings_map,
        };

        let json = serde_json::to_string_pretty(&modern)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let file_path = self.profiles_dir.join(format!("{}.json", name));
        fs::write(file_path, json)?;
        self.current_profile_name = name.to_string();

        Ok(())
    }

    pub fn load_profile(&mut self, profile_name: &str) -> io::Result<()> {
        let name = profile_name.trim();
        if name == "Default (WASD)" {
            self.apply_default_fps_profile();
            return Ok(());
        }

        let file_path = self.profiles_dir.join(format!("{}.json", name));
        let content = fs::read_to_string(&file_path)?;

        self.clear_all_bindings();

        // 1. Tentar ler no formato moderno do Rust
        if let Ok(modern) = serde_json::from_str::<ModernProfileFile>(&content) {
            for (key_str, binding) in modern.bindings {
                // Tenta casar com o nome do enum (ex: "LeftStickUp")
                let mut matched_button = None;
                for info in get_all_gamepad_buttons() {
                    if format!("{:?}", info.id) == key_str {
                        matched_button = Some(info.id);
                        break;
                    }
                }
                // Ou se for ordinal numérico
                if matched_button.is_none() {
                    if let Ok(idx) = key_str.parse::<usize>() {
                        matched_button = Self::button_from_ordinal(idx);
                    }
                }

                if let Some(button) = matched_button {
                    self.bindings.insert(button, binding);
                }
            }

            self.current_profile_name = name.to_string();
            return Ok(());
        }

        // 2. Se falhar, tentar ler formato legado do C++/Qt
        if let Ok(legacy) = serde_json::from_str::<LegacyProfileFile>(&content) {
            for (key_str, item) in legacy.bindings {
                let button = if let Ok(idx) = key_str.parse::<usize>() {
                    Self::button_from_ordinal(idx)
                } else {
                    None
                };

                if let Some(btn) = button {
                    let mut ev_code = item.evdev_code.unwrap_or(0);
                    if ev_code == 0 {
                        if let Some(scan) = item.scan_code {
                            if scan >= 8 {
                                ev_code = (scan - 8) as u16;
                            } else {
                                ev_code = scan as u16;
                            }
                        }
                    }
                    if ev_code == 0 {
                        if let Some(qt_k) = item.qt_key {
                            ev_code = qt_key_to_linux_evdev(qt_k);
                        }
                    }

                    if ev_code > 0 {
                        let name = item.key_name.unwrap_or_else(|| {
                            evdev_to_key_name(ev_code).to_string()
                        });
                        self.set_binding(btn, ev_code, &name);
                    }
                }
            }

            self.current_profile_name = name.to_string();
            return Ok(());
        }

        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Could not parse profile JSON file",
        ))
    }

    pub fn list_saved_profiles(&self) -> Vec<String> {
        let mut list = Vec::new();
        list.push("Default (WASD)".to_string());

        if let Ok(entries) = fs::read_dir(&self.profiles_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("json") {
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        if stem != "Default (WASD)" && !list.contains(&stem.to_string()) {
                            list.push(stem.to_string());
                        }
                    }
                }
            }
        }

        list
    }

    pub fn delete_profile(&mut self, profile_name: &str) -> io::Result<()> {
        let name = profile_name.trim();
        if name == "Default (WASD)" {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Cannot delete default profile",
            ));
        }

        let file_path = self.profiles_dir.join(format!("{}.json", name));
        if file_path.exists() {
            fs::remove_file(file_path)?;
        }

        if self.current_profile_name == name {
            self.apply_default_fps_profile();
        }

        Ok(())
    }
}
