use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VirtualGamepadButton {
    // Face Buttons
    A,
    B,
    X,
    Y,

    // Shoulders & Triggers
    LB,
    RB,
    LT,
    RT,

    // D-Pad
    DpadUp,
    DpadDown,
    DpadLeft,
    DpadRight,

    // Left Stick (Movement)
    LeftStickUp,
    LeftStickDown,
    LeftStickLeft,
    LeftStickRight,
    L3, // Left stick click

    // Right Stick (Camera)
    RightStickUp,
    RightStickDown,
    RightStickLeft,
    RightStickRight,
    R3, // Right stick click

    // System Buttons
    Start,
    Back,
    Guide,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GamepadButtonInfo {
    pub id: VirtualGamepadButton,
    pub name: &'static str,
    pub category: &'static str,
    pub description: &'static str,
    pub evdev_code: u16,
}

pub fn get_all_gamepad_buttons() -> &'static [GamepadButtonInfo] {
    static BUTTONS: &[GamepadButtonInfo] = &[
        // Face Buttons
        GamepadButtonInfo {
            id: VirtualGamepadButton::A,
            name: "A",
            category: "Action Buttons",
            description: "South Button (Confirm / Jump)",
            evdev_code: 304, // BTN_SOUTH
        },
        GamepadButtonInfo {
            id: VirtualGamepadButton::B,
            name: "B",
            category: "Action Buttons",
            description: "East Button (Back / Cancel)",
            evdev_code: 305, // BTN_EAST
        },
        GamepadButtonInfo {
            id: VirtualGamepadButton::X,
            name: "X",
            category: "Action Buttons",
            description: "North Button (Attack / Interact)",
            evdev_code: 307, // BTN_NORTH
        },
        GamepadButtonInfo {
            id: VirtualGamepadButton::Y,
            name: "Y",
            category: "Action Buttons",
            description: "West Button (Special / Switch)",
            evdev_code: 308, // BTN_WEST
        },

        // Bumpers & Triggers
        GamepadButtonInfo {
            id: VirtualGamepadButton::LB,
            name: "LB",
            category: "Shoulders & Triggers",
            description: "Left Bumper (L1)",
            evdev_code: 310, // BTN_TL
        },
        GamepadButtonInfo {
            id: VirtualGamepadButton::RB,
            name: "RB",
            category: "Shoulders & Triggers",
            description: "Right Bumper (R1)",
            evdev_code: 311, // BTN_TR
        },
        GamepadButtonInfo {
            id: VirtualGamepadButton::LT,
            name: "LT",
            category: "Shoulders & Triggers",
            description: "Left Trigger (L2)",
            evdev_code: 312, // BTN_TL2
        },
        GamepadButtonInfo {
            id: VirtualGamepadButton::RT,
            name: "RT",
            category: "Shoulders & Triggers",
            description: "Right Trigger (R2)",
            evdev_code: 313, // BTN_TR2
        },

        // D-Pad
        GamepadButtonInfo {
            id: VirtualGamepadButton::DpadUp,
            name: "D-Pad Up",
            category: "D-Pad",
            description: "Digital Direction Up",
            evdev_code: 544, // BTN_DPAD_UP
        },
        GamepadButtonInfo {
            id: VirtualGamepadButton::DpadDown,
            name: "D-Pad Down",
            category: "D-Pad",
            description: "Digital Direction Down",
            evdev_code: 545, // BTN_DPAD_DOWN
        },
        GamepadButtonInfo {
            id: VirtualGamepadButton::DpadLeft,
            name: "D-Pad Left",
            category: "D-Pad",
            description: "Digital Direction Left",
            evdev_code: 546, // BTN_DPAD_LEFT
        },
        GamepadButtonInfo {
            id: VirtualGamepadButton::DpadRight,
            name: "D-Pad Right",
            category: "D-Pad",
            description: "Digital Direction Right",
            evdev_code: 547, // BTN_DPAD_RIGHT
        },

        // Left Stick (Movement)
        GamepadButtonInfo {
            id: VirtualGamepadButton::LeftStickUp,
            name: "LS Up",
            category: "Left Stick (LS)",
            description: "Left Stick Y-Axis Negative (Forward)",
            evdev_code: 1, // ABS_Y
        },
        GamepadButtonInfo {
            id: VirtualGamepadButton::LeftStickDown,
            name: "LS Down",
            category: "Left Stick (LS)",
            description: "Left Stick Y-Axis Positive (Backward)",
            evdev_code: 1, // ABS_Y
        },
        GamepadButtonInfo {
            id: VirtualGamepadButton::LeftStickLeft,
            name: "LS Left",
            category: "Left Stick (LS)",
            description: "Left Stick X-Axis Negative (Left)",
            evdev_code: 0, // ABS_X
        },
        GamepadButtonInfo {
            id: VirtualGamepadButton::LeftStickRight,
            name: "LS Right",
            category: "Left Stick (LS)",
            description: "Left Stick X-Axis Positive (Right)",
            evdev_code: 0, // ABS_X
        },
        GamepadButtonInfo {
            id: VirtualGamepadButton::L3,
            name: "L3 (LS Click)",
            category: "Left Stick (LS)",
            description: "Press Left Stick (Sprint)",
            evdev_code: 317, // BTN_THUMBL
        },

        // Right Stick (Camera)
        GamepadButtonInfo {
            id: VirtualGamepadButton::RightStickUp,
            name: "RS Up",
            category: "Right Stick (RS)",
            description: "Right Stick Y-Axis Negative (Look Up)",
            evdev_code: 4, // ABS_RY
        },
        GamepadButtonInfo {
            id: VirtualGamepadButton::RightStickDown,
            name: "RS Down",
            category: "Right Stick (RS)",
            description: "Right Stick Y-Axis Positive (Look Down)",
            evdev_code: 4, // ABS_RY
        },
        GamepadButtonInfo {
            id: VirtualGamepadButton::RightStickLeft,
            name: "RS Left",
            category: "Right Stick (RS)",
            description: "Right Stick X-Axis Negative (Look Left)",
            evdev_code: 3, // ABS_RX
        },
        GamepadButtonInfo {
            id: VirtualGamepadButton::RightStickRight,
            name: "RS Right",
            category: "Right Stick (RS)",
            description: "Right Stick X-Axis Positive (Look Right)",
            evdev_code: 3, // ABS_RX
        },
        GamepadButtonInfo {
            id: VirtualGamepadButton::R3,
            name: "R3 (RS Click)",
            category: "Right Stick (RS)",
            description: "Press Right Stick (Melee / Zoom)",
            evdev_code: 318, // BTN_THUMBR
        },

        // System Buttons
        GamepadButtonInfo {
            id: VirtualGamepadButton::Start,
            name: "Start",
            category: "Menus & System",
            description: "Menu / Pause Button",
            evdev_code: 315, // BTN_START
        },
        GamepadButtonInfo {
            id: VirtualGamepadButton::Back,
            name: "Back / View",
            category: "Menus & System",
            description: "Back / Map Button",
            evdev_code: 314, // BTN_SELECT
        },
        GamepadButtonInfo {
            id: VirtualGamepadButton::Guide,
            name: "Guide / Home",
            category: "Menus & System",
            description: "Xbox / Guide Button",
            evdev_code: 316, // BTN_MODE
        },
    ];

    BUTTONS
}
