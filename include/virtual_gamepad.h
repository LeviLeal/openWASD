#pragma once

#include "gamepad_types.h"
#include <string>

class VirtualGamepad {
public:
    VirtualGamepad();
    ~VirtualGamepad();

    bool openDevice(const std::string &deviceName = "Microsoft X-Box 360 pad");
    void closeDevice();
    bool isOpen() const { return m_fd >= 0; }

    // Enviar comandos
    void sendButton(VirtualGamepadButton button, bool pressed);
    void sendKey(int evdevCode, bool pressed);
    void sendAxis(int axis, int value);
    void sync();

    // Resetar todos os eixos e botões para estado neutro
    void resetState();

private:
    int m_fd = -1;

    // Estado dos eixos analógicos (para combinar múltiplas teclas como W+D)
    int m_leftStickX = 0;
    int m_leftStickY = 0;
    int m_rightStickX = 0;
    int m_rightStickY = 0;

    bool m_lsUp = false;
    bool m_lsDown = false;
    bool m_lsLeft = false;
    bool m_lsRight = false;

    bool m_rsUp = false;
    bool m_rsDown = false;
    bool m_rsLeft = false;
    bool m_rsRight = false;

    void updateLeftStick();
    void updateRightStick();
    void emitEvent(int type, int code, int value);
};
