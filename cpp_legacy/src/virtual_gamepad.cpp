#include "virtual_gamepad.h"

#include <fcntl.h>
#include <unistd.h>
#include <cstring>
#include <iostream>
#include <linux/uinput.h>

static void setupAbs(int fd, int axis, int min, int max, int fuzz = 0, int flat = 0) {
    ioctl(fd, UI_SET_ABSBIT, axis);
    struct uinput_abs_setup absSetup;
    std::memset(&absSetup, 0, sizeof(absSetup));
    absSetup.code = axis;
    absSetup.absinfo.minimum = min;
    absSetup.absinfo.maximum = max;
    absSetup.absinfo.fuzz = fuzz;
    absSetup.absinfo.flat = flat;
    ioctl(fd, UI_ABS_SETUP, &absSetup);
}

VirtualGamepad::VirtualGamepad() {
}

VirtualGamepad::~VirtualGamepad() {
    closeDevice();
}

bool VirtualGamepad::openDevice(const std::string &deviceName) {
    if (m_fd >= 0) return true;

    m_fd = open("/dev/uinput", O_WRONLY | O_NONBLOCK);
    if (m_fd < 0) {
        std::cerr << "[VirtualGamepad] Erro ao abrir /dev/uinput: " << strerror(errno) << std::endl;
        return false;
    }

    // Habilitar tipos de eventos
    ioctl(m_fd, UI_SET_EVBIT, EV_KEY);
    ioctl(m_fd, UI_SET_EVBIT, EV_ABS);
    ioctl(m_fd, UI_SET_EVBIT, EV_SYN);

    // Botões de Ação
    ioctl(m_fd, UI_SET_KEYBIT, BTN_SOUTH);
    ioctl(m_fd, UI_SET_KEYBIT, BTN_EAST);
    ioctl(m_fd, UI_SET_KEYBIT, BTN_NORTH);
    ioctl(m_fd, UI_SET_KEYBIT, BTN_WEST);

    // Bumpers e Gatilhos (digitais e analógicos)
    ioctl(m_fd, UI_SET_KEYBIT, BTN_TL);
    ioctl(m_fd, UI_SET_KEYBIT, BTN_TR);
    ioctl(m_fd, UI_SET_KEYBIT, BTN_TL2);
    ioctl(m_fd, UI_SET_KEYBIT, BTN_TR2);

    // Sticks e Menus
    ioctl(m_fd, UI_SET_KEYBIT, BTN_SELECT);
    ioctl(m_fd, UI_SET_KEYBIT, BTN_START);
    ioctl(m_fd, UI_SET_KEYBIT, BTN_MODE);
    ioctl(m_fd, UI_SET_KEYBIT, BTN_THUMBL);
    ioctl(m_fd, UI_SET_KEYBIT, BTN_THUMBR);

    // D-Pad como botões digitais
    ioctl(m_fd, UI_SET_KEYBIT, BTN_DPAD_UP);
    ioctl(m_fd, UI_SET_KEYBIT, BTN_DPAD_DOWN);
    ioctl(m_fd, UI_SET_KEYBIT, BTN_DPAD_LEFT);
    ioctl(m_fd, UI_SET_KEYBIT, BTN_DPAD_RIGHT);

    // Eixos Analógicos
    setupAbs(m_fd, ABS_X, -32768, 32767, 16, 128);
    setupAbs(m_fd, ABS_Y, -32768, 32767, 16, 128);
    setupAbs(m_fd, ABS_RX, -32768, 32767, 16, 128);
    setupAbs(m_fd, ABS_RY, -32768, 32767, 16, 128);

    // Gatilhos Analógicos (LT / RT)
    setupAbs(m_fd, ABS_Z, 0, 255);
    setupAbs(m_fd, ABS_RZ, 0, 255);

    // D-Pad como Hat Switch
    setupAbs(m_fd, ABS_HAT0X, -1, 1);
    setupAbs(m_fd, ABS_HAT0Y, -1, 1);

    // Configuração do dispositivo (ID Xbox 360 padrão oficial para compatibilidade com Proton/Wine)
    struct uinput_setup us;
    std::memset(&us, 0, sizeof(us));
    us.id.bustype = BUS_USB;
    us.id.vendor = 0x045e;
    us.id.product = 0x028e;
    us.id.version = 1;
    std::strncpy(us.name, deviceName.c_str(), UINPUT_MAX_NAME_SIZE);

    if (ioctl(m_fd, UI_DEV_SETUP, &us) < 0) {
        std::cerr << "[VirtualGamepad] Falha em UI_DEV_SETUP: " << strerror(errno) << std::endl;
        close(m_fd);
        m_fd = -1;
        return false;
    }

    if (ioctl(m_fd, UI_DEV_CREATE) < 0) {
        std::cerr << "[VirtualGamepad] Falha em UI_DEV_CREATE: " << strerror(errno) << std::endl;
        close(m_fd);
        m_fd = -1;
        return false;
    }

    std::cout << "[VirtualGamepad] Controle criado com sucesso: " << deviceName << std::endl;
    resetState();
    return true;
}

void VirtualGamepad::closeDevice() {
    if (m_fd >= 0) {
        resetState();
        ioctl(m_fd, UI_DEV_DESTROY);
        close(m_fd);
        m_fd = -1;
        std::cout << "[VirtualGamepad] Controle destruído." << std::endl;
    }
}

void VirtualGamepad::emitEvent(int type, int code, int value) {
    if (m_fd < 0) return;
    struct input_event ev;
    std::memset(&ev, 0, sizeof(ev));
    ev.type = type;
    ev.code = code;
    ev.value = value;
    write(m_fd, &ev, sizeof(ev));
}

void VirtualGamepad::sendKey(int evdevCode, bool pressed) {
    emitEvent(EV_KEY, evdevCode, pressed ? 1 : 0);
}

void VirtualGamepad::sendAxis(int axis, int value) {
    emitEvent(EV_ABS, axis, value);
}

void VirtualGamepad::sync() {
    emitEvent(EV_SYN, SYN_REPORT, 0);
}

void VirtualGamepad::sendButton(VirtualGamepadButton button, bool pressed) {
    if (m_fd < 0) return;

    switch (button) {
        case VirtualGamepadButton::A: sendKey(BTN_SOUTH, pressed); break;
        case VirtualGamepadButton::B: sendKey(BTN_EAST, pressed); break;
        case VirtualGamepadButton::X: sendKey(BTN_NORTH, pressed); break;
        case VirtualGamepadButton::Y: sendKey(BTN_WEST, pressed); break;

        case VirtualGamepadButton::LB: sendKey(BTN_TL, pressed); break;
        case VirtualGamepadButton::RB: sendKey(BTN_TR, pressed); break;

        case VirtualGamepadButton::LT:
            sendKey(BTN_TL2, pressed);
            sendAxis(ABS_Z, pressed ? 255 : 0);
            break;
        case VirtualGamepadButton::RT:
            sendKey(BTN_TR2, pressed);
            sendAxis(ABS_RZ, pressed ? 255 : 0);
            break;

        case VirtualGamepadButton::DpadUp:
            sendKey(BTN_DPAD_UP, pressed);
            sendAxis(ABS_HAT0Y, pressed ? -1 : 0);
            break;
        case VirtualGamepadButton::DpadDown:
            sendKey(BTN_DPAD_DOWN, pressed);
            sendAxis(ABS_HAT0Y, pressed ? 1 : 0);
            break;
        case VirtualGamepadButton::DpadLeft:
            sendKey(BTN_DPAD_LEFT, pressed);
            sendAxis(ABS_HAT0X, pressed ? -1 : 0);
            break;
        case VirtualGamepadButton::DpadRight:
            sendKey(BTN_DPAD_RIGHT, pressed);
            sendAxis(ABS_HAT0X, pressed ? 1 : 0);
            break;

        case VirtualGamepadButton::L3: sendKey(BTN_THUMBL, pressed); break;
        case VirtualGamepadButton::R3: sendKey(BTN_THUMBR, pressed); break;

        case VirtualGamepadButton::Start: sendKey(BTN_START, pressed); break;
        case VirtualGamepadButton::Back:  sendKey(BTN_SELECT, pressed); break;
        case VirtualGamepadButton::Guide: sendKey(BTN_MODE, pressed); break;

        // Eixos Analógicos Esquerdo (WASD)
        case VirtualGamepadButton::LeftStickUp:
            m_lsUp = pressed;
            updateLeftStick();
            break;
        case VirtualGamepadButton::LeftStickDown:
            m_lsDown = pressed;
            updateLeftStick();
            break;
        case VirtualGamepadButton::LeftStickLeft:
            m_lsLeft = pressed;
            updateLeftStick();
            break;
        case VirtualGamepadButton::LeftStickRight:
            m_lsRight = pressed;
            updateLeftStick();
            break;

        // Eixos Analógicos Direito (Câmera)
        case VirtualGamepadButton::RightStickUp:
            m_rsUp = pressed;
            updateRightStick();
            break;
        case VirtualGamepadButton::RightStickDown:
            m_rsDown = pressed;
            updateRightStick();
            break;
        case VirtualGamepadButton::RightStickLeft:
            m_rsLeft = pressed;
            updateRightStick();
            break;
        case VirtualGamepadButton::RightStickRight:
            m_rsRight = pressed;
            updateRightStick();
            break;
    }

    sync();
}

void VirtualGamepad::updateLeftStick() {
    int y = 0;
    if (m_lsUp && !m_lsDown) y = -32767;
    else if (m_lsDown && !m_lsUp) y = 32767;

    int x = 0;
    if (m_lsLeft && !m_lsRight) x = -32767;
    else if (m_lsRight && !m_lsLeft) x = 32767;

    m_leftStickX = x;
    m_leftStickY = y;

    sendAxis(ABS_X, m_leftStickX);
    sendAxis(ABS_Y, m_leftStickY);
}

void VirtualGamepad::updateRightStick() {
    int y = 0;
    if (m_rsUp && !m_rsDown) y = -32767;
    else if (m_rsDown && !m_rsUp) y = 32767;

    int x = 0;
    if (m_rsLeft && !m_rsRight) x = -32767;
    else if (m_rsRight && !m_rsLeft) x = 32767;

    m_rightStickX = x;
    m_rightStickY = y;

    sendAxis(ABS_RX, m_rightStickX);
    sendAxis(ABS_RY, m_rightStickY);
}

void VirtualGamepad::resetState() {
    m_lsUp = m_lsDown = m_lsLeft = m_lsRight = false;
    m_rsUp = m_rsDown = m_rsLeft = m_rsRight = false;
    m_leftStickX = m_leftStickY = 0;
    m_rightStickX = m_rightStickY = 0;

    sendAxis(ABS_X, 0);
    sendAxis(ABS_Y, 0);
    sendAxis(ABS_RX, 0);
    sendAxis(ABS_RY, 0);
    sendAxis(ABS_Z, 0);
    sendAxis(ABS_RZ, 0);
    sendAxis(ABS_HAT0X, 0);
    sendAxis(ABS_HAT0Y, 0);

    sync();
}
