#pragma once

#include <QString>
#include <vector>

// Enum with all supported buttons and analog directions
enum class VirtualGamepadButton {
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

    // Left Stick (WASD Movement)
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
    Guide
};

struct GamepadButtonInfo {
    VirtualGamepadButton id;
    QString name;
    QString category;
    QString description;
    int evdevCode; // Linux kernel evdev code
};

inline const std::vector<GamepadButtonInfo>& getAllGamepadButtons() {
    static const std::vector<GamepadButtonInfo> buttons = {
        // Face Buttons
        { VirtualGamepadButton::A, "A", "Action Buttons", "South Button (Confirm / Jump)", 304 /* BTN_SOUTH */ },
        { VirtualGamepadButton::B, "B", "Action Buttons", "East Button (Back / Cancel)", 305 /* BTN_EAST */ },
        { VirtualGamepadButton::X, "X", "Action Buttons", "North Button (Attack / Interact)", 307 /* BTN_NORTH */ },
        { VirtualGamepadButton::Y, "Y", "Action Buttons", "West Button (Special / Switch)", 308 /* BTN_WEST */ },

        // Bumpers & Triggers
        { VirtualGamepadButton::LB, "LB", "Shoulders & Triggers", "Left Bumper (L1)", 310 /* BTN_TL */ },
        { VirtualGamepadButton::RB, "RB", "Shoulders & Triggers", "Right Bumper (R1)", 311 /* BTN_TR */ },
        { VirtualGamepadButton::LT, "LT", "Shoulders & Triggers", "Left Trigger (L2)", 312 /* BTN_TL2 */ },
        { VirtualGamepadButton::RT, "RT", "Shoulders & Triggers", "Right Trigger (R2)", 313 /* BTN_TR2 */ },

        // D-Pad
        { VirtualGamepadButton::DpadUp, "D-Pad Up", "D-Pad", "Digital Direction Up", 544 /* BTN_DPAD_UP */ },
        { VirtualGamepadButton::DpadDown, "D-Pad Down", "D-Pad", "Digital Direction Down", 545 /* BTN_DPAD_DOWN */ },
        { VirtualGamepadButton::DpadLeft, "D-Pad Left", "D-Pad", "Digital Direction Left", 546 /* BTN_DPAD_LEFT */ },
        { VirtualGamepadButton::DpadRight, "D-Pad Right", "D-Pad", "Digital Direction Right", 547 /* BTN_DPAD_RIGHT */ },

        // Left Stick (WASD)
        { VirtualGamepadButton::LeftStickUp, "LS Up", "Left Stick (LS)", "Left Stick Y-Axis Negative", 1 },
        { VirtualGamepadButton::LeftStickDown, "LS Down", "Left Stick (LS)", "Left Stick Y-Axis Positive", 1 },
        { VirtualGamepadButton::LeftStickLeft, "LS Left", "Left Stick (LS)", "Left Stick X-Axis Negative", 0 },
        { VirtualGamepadButton::LeftStickRight, "LS Right", "Left Stick (LS)", "Left Stick X-Axis Positive", 0 },
        { VirtualGamepadButton::L3, "L3 (LS Click)", "Left Stick (LS)", "Press Left Stick (Sprint)", 317 /* BTN_THUMBL */ },

        // Right Stick (Camera)
        { VirtualGamepadButton::RightStickUp, "RS Up", "Right Stick (RS)", "Right Stick Y-Axis Negative", 4 },
        { VirtualGamepadButton::RightStickDown, "RS Down", "Right Stick (RS)", "Right Stick Y-Axis Positive", 4 },
        { VirtualGamepadButton::RightStickLeft, "RS Left", "Right Stick (RS)", "Right Stick X-Axis Negative", 3 },
        { VirtualGamepadButton::RightStickRight, "RS Right", "Right Stick (RS)", "Right Stick X-Axis Positive", 3 },
        { VirtualGamepadButton::R3, "R3 (RS Click)", "Right Stick (RS)", "Press Right Stick", 318 /* BTN_THUMBR */ },

        // System
        { VirtualGamepadButton::Start, "Start", "Menus & System", "Menu / Pause Button", 315 /* BTN_START */ },
        { VirtualGamepadButton::Back, "Back / View", "Menus & System", "Back / Map Button", 314 /* BTN_SELECT */ },
        { VirtualGamepadButton::Guide, "Guide / Home", "Menus & System", "Xbox / Guide Button", 316 /* BTN_MODE */ }
    };
    return buttons;
}
