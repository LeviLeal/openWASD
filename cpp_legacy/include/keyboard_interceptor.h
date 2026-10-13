#pragma once

#include "virtual_gamepad.h"
#include "mapping_manager.h"
#include <QThread>
#include <QMutex>
#include <QStringList>
#include <QMap>

struct InputDeviceInfo {
    QString path;
    QString name;
    bool isKeyboard = false;
};

class KeyboardInterceptor : public QThread {
    Q_OBJECT

public:
    explicit KeyboardInterceptor(VirtualGamepad *gamepad, QObject *parent = nullptr);
    ~KeyboardInterceptor() override;

    static QList<InputDeviceInfo> detectKeyboards();

    void setDevicePath(const QString &devicePath);
    void setGrabEnabled(bool grab);
    void updateMappings(const QMap<VirtualGamepadButton, KeyBinding> &bindings);

    bool isRunningInterceptor() const { return m_running; }
    void startInterceptor();
    void stopInterceptor();
    void setPaused(bool paused);
    bool isPaused() const { return m_paused; }

signals:
    void interceptorStarted();
    void interceptorStopped();
    void permissionError(const QString &devicePath);
    void keyEventProcessed(int evdevCode, bool pressed);

protected:
    void run() override;

private:
    VirtualGamepad *m_gamepad = nullptr;
    QString m_devicePath;
    bool m_grabEnabled = false;
    std::atomic<bool> m_running{false};
    std::atomic<bool> m_paused{false};

    QMutex m_mappingMutex;
    // Mapeamento: evdev keycode -> lista de botões virtuais
    QMap<int, QList<VirtualGamepadButton>> m_evdevToButtons;

    int m_deviceFd = -1;
};
