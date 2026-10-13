#include "keyboard_interceptor.h"
#include "key_codes.h"

#include <libevdev/libevdev.h>
#include <fcntl.h>
#include <unistd.h>
#include <poll.h>
#include <iostream>
#include <QDir>
#include <QFileInfo>

KeyboardInterceptor::KeyboardInterceptor(VirtualGamepad *gamepad, QObject *parent)
    : QThread(parent)
    , m_gamepad(gamepad)
{
}

KeyboardInterceptor::~KeyboardInterceptor() {
    stopInterceptor();
}

QList<InputDeviceInfo> KeyboardInterceptor::detectKeyboards() {
    QList<InputDeviceInfo> list;

    // Primeiro verificar em /dev/input/by-id
    QDir byIdDir("/dev/input/by-id");
    if (byIdDir.exists()) {
        QStringList entries = byIdDir.entryList(QDir::Files | QDir::System);
        for (const QString &entry : entries) {
            if (entry.contains("-kbd") || entry.contains("event-kbd")) {
                QString fullPath = byIdDir.absoluteFilePath(entry);
                InputDeviceInfo info;
                info.path = fullPath;
                info.name = entry;
                info.isKeyboard = true;
                list.append(info);
            }
        }
    }

    // Se a lista estiver vazia, procurar em /dev/input/event*
    if (list.isEmpty()) {
        QDir inputDir("/dev/input");
        QStringList entries = inputDir.entryList(QStringList() << "event*", QDir::System);
        for (const QString &entry : entries) {
            QString path = inputDir.absoluteFilePath(entry);
            int fd = open(path.toLocal8Bit().constData(), O_RDONLY | O_NONBLOCK);
            if (fd >= 0) {
                struct libevdev *dev = nullptr;
                if (libevdev_new_from_fd(fd, &dev) >= 0) {
                    if (libevdev_has_event_code(dev, EV_KEY, KEY_A) &&
                        libevdev_has_event_code(dev, EV_KEY, KEY_SPACE)) {
                        InputDeviceInfo info;
                        info.path = path;
                        info.name = QString::fromUtf8(libevdev_get_name(dev));
                        info.isKeyboard = true;
                        list.append(info);
                    }
                    libevdev_free(dev);
                }
                close(fd);
            }
        }
    }

    return list;
}

void KeyboardInterceptor::setDevicePath(const QString &devicePath) {
    m_devicePath = devicePath;
}

void KeyboardInterceptor::setGrabEnabled(bool grab) {
    m_grabEnabled = grab;
}

void KeyboardInterceptor::updateMappings(const QMap<VirtualGamepadButton, KeyBinding> &bindings) {
    QMutexLocker locker(&m_mappingMutex);
    m_evdevToButtons.clear();

    for (auto it = bindings.begin(); it != bindings.end(); ++it) {
        if (!it.value().isValid) continue;

        int evCode = 0;
        // Se temos o scancode gravado
        if (it.value().nativeScanCode > 0) {
            // No Linux xkbcommon/Wayland, nativeScanCode costuma ser evdev_keycode + 8
            if (it.value().nativeScanCode >= 8) {
                evCode = it.value().nativeScanCode - 8;
            } else {
                evCode = it.value().nativeScanCode;
            }
        }

        // Se falhou ou zero, tenta pelo Qt keycode
        if (evCode <= 0) {
            evCode = qtKeyToLinuxEvdev(it.value().qtKey);
        }

        if (evCode > 0) {
            m_evdevToButtons[evCode].append(it.key());
        }
    }
}

void KeyboardInterceptor::startInterceptor() {
    if (m_running) return;
    m_running = true;
    start();
}

void KeyboardInterceptor::stopInterceptor() {
    if (!m_running) return;
    m_running = false;
    wait(1000);
}

void KeyboardInterceptor::setPaused(bool paused) {
    m_paused = paused;
}

void KeyboardInterceptor::run() {
    QString targetPath = m_devicePath;
    if (targetPath.isEmpty()) {
        auto keyboards = detectKeyboards();
        if (!keyboards.isEmpty()) {
            targetPath = keyboards.first().path;
        } else {
            targetPath = "/dev/input/event3"; // Fallback comum
        }
    }

    int fd = open(targetPath.toLocal8Bit().constData(), O_RDONLY | O_NONBLOCK);
    if (fd < 0) {
        emit permissionError(targetPath);
        m_running = false;
        return;
    }

    struct libevdev *dev = nullptr;
    int rc = libevdev_new_from_fd(fd, &dev);
    if (rc < 0) {
        close(fd);
        emit permissionError(targetPath);
        m_running = false;
        return;
    }

    if (m_grabEnabled) {
        libevdev_grab(dev, LIBEVDEV_GRAB);
    }

    emit interceptorStarted();

    struct pollfd pfd;
    pfd.fd = fd;
    pfd.events = POLLIN;

    bool currentlyGrabbed = m_grabEnabled;

    while (m_running) {
        if (m_paused) {
            if (currentlyGrabbed) {
                libevdev_grab(dev, LIBEVDEV_UNGRAB);
                currentlyGrabbed = false;
            }
            usleep(30000); // 30ms
            continue;
        } else {
            if (m_grabEnabled && !currentlyGrabbed) {
                libevdev_grab(dev, LIBEVDEV_GRAB);
                currentlyGrabbed = true;
            }
        }

        int pollRes = poll(&pfd, 1, 50); // 50ms timeout
        if (pollRes <= 0) continue;

        if (pfd.revents & POLLIN) {
            struct input_event ev;
            while (libevdev_next_event(dev, LIBEVDEV_READ_FLAG_NORMAL, &ev) == LIBEVDEV_READ_STATUS_SUCCESS) {
                if (ev.type == EV_KEY) {
                    bool pressed = (ev.value == 1);
                    bool released = (ev.value == 0);

                    if (pressed || released) {
                        QList<VirtualGamepadButton> buttons;
                        {
                            QMutexLocker locker(&m_mappingMutex);
                            buttons = m_evdevToButtons.value(ev.code);
                        }

                        if (!buttons.isEmpty()) {
                            for (auto btn : buttons) {
                                m_gamepad->sendButton(btn, pressed);
                            }
                            emit keyEventProcessed(ev.code, pressed);
                        }
                    }
                }
            }
        }
    }

    if (m_grabEnabled) {
        libevdev_grab(dev, LIBEVDEV_UNGRAB);
    }

    libevdev_free(dev);
    close(fd);

    // Limpar estados residuais do controle
    m_gamepad->resetState();
    emit interceptorStopped();
}
