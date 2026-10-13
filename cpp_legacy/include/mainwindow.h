#pragma once

#include "gamepad_types.h"
#include "mapping_manager.h"
#include "key_binding_button.h"
#include "virtual_gamepad.h"
#include "keyboard_interceptor.h"

#include <QMainWindow>
#include <QMap>
#include <QComboBox>
#include <QLabel>
#include <QTextEdit>
#include <QCheckBox>
#include <QPushButton>

class MainWindow : public QMainWindow {
    Q_OBJECT

public:
    explicit MainWindow(QWidget *parent = nullptr);
    ~MainWindow() override;

protected:
    void keyPressEvent(QKeyEvent *event) override;
    void keyReleaseEvent(QKeyEvent *event) override;

private slots:
    void onBindingChanged(VirtualGamepadButton buttonId, int qtKey, quint32 nativeScanCode, const QString &keyName);
    void onBindingCleared(VirtualGamepadButton buttonId);
    void onProfileSelected(int index);
    void onSaveProfileClicked();
    void onNewProfileClicked();
    void onResetDefaultClicked();
    void onClearAllClicked();

    // Emulação em Tempo Real
    void onToggleEmulationClicked();
    void onInterceptorStarted();
    void onInterceptorStopped();
    void onPermissionError(const QString &devicePath);
    void onKeyEventProcessed(int evdevCode, bool pressed);

private:
    void setupUi();
    void refreshProfileList();
    void refreshKeyboardList();
    void syncUiFromMappingManager();
    void logEvent(const QString &text);

    QWidget* createColumnGroup(const QString &title, const QList<GamepadButtonInfo> &buttons);

    MappingManager *m_mappingManager = nullptr;
    QMap<VirtualGamepadButton, KeyBindingButton*> m_buttonWidgets;

    // Emulação de Controle e Interceptação
    VirtualGamepad m_virtualGamepad;
    KeyboardInterceptor *m_interceptor = nullptr;

    QComboBox *m_profileCombo = nullptr;
    QComboBox *m_keyboardCombo = nullptr;
    QCheckBox *m_exclusiveGrabCheck = nullptr;
    QPushButton *m_btnToggleEmulation = nullptr;
    QLabel *m_statusLabel = nullptr;
    QLabel *m_gamepadStatusBadge = nullptr;
    QTextEdit *m_logConsole = nullptr;
};
