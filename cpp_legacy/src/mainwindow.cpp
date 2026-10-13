#include "mainwindow.h"
#include "key_codes.h"

#include <QVBoxLayout>
#include <QHBoxLayout>
#include <QScrollArea>
#include <QGroupBox>
#include <QPushButton>
#include <QInputDialog>
#include <QMessageBox>
#include <QDateTime>
#include <QStatusBar>
#include <QApplication>
#include <QIcon>

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
    , m_mappingManager(new MappingManager(this))
    , m_interceptor(new KeyboardInterceptor(&m_virtualGamepad, this))
{
    setupUi();
    refreshProfileList();
    refreshKeyboardList();
    syncUiFromMappingManager();

    // Background Interceptor Connections
    connect(m_interceptor, &KeyboardInterceptor::interceptorStarted, this, &MainWindow::onInterceptorStarted);
    connect(m_interceptor, &KeyboardInterceptor::interceptorStopped, this, &MainWindow::onInterceptorStopped);
    connect(m_interceptor, &KeyboardInterceptor::permissionError, this, &MainWindow::onPermissionError);
    connect(m_interceptor, &KeyboardInterceptor::keyEventProcessed, this, &MainWindow::onKeyEventProcessed);

    // Profile Loaded Connection
    connect(m_mappingManager, &MappingManager::profileLoaded, this, [this](const QString &) {
        syncUiFromMappingManager();
        if (m_interceptor->isRunningInterceptor()) {
            m_interceptor->updateMappings(m_mappingManager->getAllBindings());
        }
    });
}

MainWindow::~MainWindow() {
    if (m_interceptor) {
        m_interceptor->stopInterceptor();
    }
    m_virtualGamepad.closeDevice();
}

QWidget* MainWindow::createColumnGroup(const QString &title, const QList<GamepadButtonInfo> &buttons) {
    QGroupBox *box = new QGroupBox(title, this);
    QVBoxLayout *layout = new QVBoxLayout(box);
    layout->setContentsMargins(12, 16, 12, 12);
    layout->setSpacing(8);

    for (const auto &info : buttons) {
        QWidget *row = new QWidget(box);
        QHBoxLayout *rowLayout = new QHBoxLayout(row);
        rowLayout->setContentsMargins(0, 0, 0, 0);
        rowLayout->setSpacing(8);

        QLabel *lbl = new QLabel(info.name, row);
        lbl->setMinimumWidth(110);
        lbl->setToolTip(info.description);

        KeyBindingButton *btn = new KeyBindingButton(info.id, row);
        btn->setMinimumWidth(120);
        btn->setSizePolicy(QSizePolicy::Expanding, QSizePolicy::Fixed);
        m_buttonWidgets[info.id] = btn;

        connect(btn, &KeyBindingButton::bindingChanged, this, &MainWindow::onBindingChanged);
        connect(btn, &KeyBindingButton::bindingCleared, this, &MainWindow::onBindingCleared);
        connect(btn, &KeyBindingButton::listeningStateChanged, this, [this](VirtualGamepadButton, bool isListening) {
            if (m_interceptor && m_interceptor->isRunningInterceptor()) {
                m_interceptor->setPaused(isListening);
            }
        });

        QPushButton *btnClear = new QPushButton(row);
        btnClear->setIcon(QIcon::fromTheme("edit-clear", QIcon::fromTheme("window-close")));
        btnClear->setFixedSize(28, 28);
        btnClear->setToolTip("Unbind");
        connect(btnClear, &QPushButton::clicked, btn, &KeyBindingButton::clearBinding);

        rowLayout->addWidget(lbl);
        rowLayout->addWidget(btn, 1);
        rowLayout->addWidget(btnClear);

        layout->addWidget(row);
    }

    layout->addStretch();
    return box;
}

void MainWindow::setupUi() {
    setWindowTitle("WASD Anywhere — Virtual Gamepad Mapper");
    QIcon appIcon(":/icon.png");
    if (appIcon.isNull()) {
        appIcon = QIcon::fromTheme("input-gaming", QIcon::fromTheme("input-gamepad"));
    }
    setWindowIcon(appIcon);
    resize(1200, 780);
    setMinimumSize(950, 650);

    QWidget *centralWidget = new QWidget(this);
    setCentralWidget(centralWidget);

    QVBoxLayout *mainLayout = new QVBoxLayout(centralWidget);
    mainLayout->setContentsMargins(14, 14, 14, 14);
    mainLayout->setSpacing(12);

    // ==========================================
    // 1. Top Bar (Header & Profiles & Live Switch)
    // ==========================================
    QGroupBox *headerGroup = new QGroupBox(this);
    QHBoxLayout *headerLayout = new QHBoxLayout(headerGroup);
    headerLayout->setContentsMargins(12, 8, 12, 8);

    QLabel *appIconLabel = new QLabel(this);
    appIconLabel->setPixmap(appIcon.pixmap(24, 24));

    QLabel *appTitle = new QLabel("WASD Anywhere", this);
    QFont titleFont = appTitle->font();
    titleFont.setBold(true);
    titleFont.setPointSize(titleFont.pointSize() + 2);
    appTitle->setFont(titleFont);

    QLabel *profileLabel = new QLabel("Profile:", this);

    m_profileCombo = new QComboBox(this);
    m_profileCombo->setMinimumWidth(160);
    connect(m_profileCombo, QOverload<int>::of(&QComboBox::currentIndexChanged), this, &MainWindow::onProfileSelected);

    QPushButton *btnSave = new QPushButton("Save", this);
    btnSave->setIcon(QIcon::fromTheme("document-save"));

    QPushButton *btnNew = new QPushButton("New", this);
    btnNew->setIcon(QIcon::fromTheme("document-new", QIcon::fromTheme("list-add")));

    QPushButton *btnReset = new QPushButton("Default", this);
    btnReset->setIcon(QIcon::fromTheme("edit-undo", QIcon::fromTheme("view-refresh")));

    QPushButton *btnClear = new QPushButton("Clear All", this);
    btnClear->setIcon(QIcon::fromTheme("edit-clear", QIcon::fromTheme("user-trash")));

    connect(btnSave, &QPushButton::clicked, this, &MainWindow::onSaveProfileClicked);
    connect(btnNew, &QPushButton::clicked, this, &MainWindow::onNewProfileClicked);
    connect(btnReset, &QPushButton::clicked, this, &MainWindow::onResetDefaultClicked);
    connect(btnClear, &QPushButton::clicked, this, &MainWindow::onClearAllClicked);

    m_btnToggleEmulation = new QPushButton("Start Emulation", this);
    m_btnToggleEmulation->setIcon(QIcon::fromTheme("media-playback-start"));
    m_btnToggleEmulation->setMinimumHeight(34);
    connect(m_btnToggleEmulation, &QPushButton::clicked, this, &MainWindow::onToggleEmulationClicked);

    m_gamepadStatusBadge = new QLabel("Inactive", this);
    QPalette pal = palette();
    QColor disabledColor = pal.color(QPalette::Disabled, QPalette::WindowText);
    m_gamepadStatusBadge->setStyleSheet(QString("color: %1; font-weight: bold; margin-left: 8px;").arg(disabledColor.name()));

    headerLayout->addWidget(appIconLabel);
    headerLayout->addWidget(appTitle);
    headerLayout->addSpacing(14);
    headerLayout->addWidget(profileLabel);
    headerLayout->addWidget(m_profileCombo);
    headerLayout->addWidget(btnSave);
    headerLayout->addWidget(btnNew);
    headerLayout->addWidget(btnReset);
    headerLayout->addWidget(btnClear);
    headerLayout->addStretch();
    headerLayout->addWidget(m_btnToggleEmulation);
    headerLayout->addWidget(m_gamepadStatusBadge);

    mainLayout->addWidget(headerGroup);

    // ==========================================
    // 2. Center Stage: ALL BUTTONS IN 4 COLUMNS
    // ==========================================
    QList<GamepadButtonInfo> movButtons;
    QList<GamepadButtonInfo> actionButtons;
    QList<GamepadButtonInfo> dpadMenuButtons;
    QList<GamepadButtonInfo> cameraButtons;

    for (const auto &info : getAllGamepadButtons()) {
        if (info.category.contains("Left Stick")) {
            movButtons.append(info);
        } else if (info.category.contains("Action") || info.category.contains("Shoulders")) {
            actionButtons.append(info);
        } else if (info.category.contains("D-Pad") || info.category.contains("Menus")) {
            dpadMenuButtons.append(info);
        } else if (info.category.contains("Right Stick")) {
            cameraButtons.append(info);
        }
    }

    QWidget *columnsContainer = new QWidget(this);
    QHBoxLayout *columnsLayout = new QHBoxLayout(columnsContainer);
    columnsLayout->setContentsMargins(0, 0, 0, 0);
    columnsLayout->setSpacing(12);

    columnsLayout->addWidget(createColumnGroup("Left Stick", movButtons), 1);
    columnsLayout->addWidget(createColumnGroup("Buttons & Shoulders", actionButtons), 1);
    columnsLayout->addWidget(createColumnGroup("Right Stick", cameraButtons), 1);
    columnsLayout->addWidget(createColumnGroup("D-Pad & Menus", dpadMenuButtons), 1);

    QScrollArea *scrollArea = new QScrollArea(this);
    scrollArea->setWidgetResizable(true);
    scrollArea->setWidget(columnsContainer);
    scrollArea->setFrameShape(QFrame::NoFrame);

    mainLayout->addWidget(scrollArea, 1);

    // ==========================================
    // 3. Bottom Bar (Device & Settings & Console)
    // ==========================================
    QGroupBox *bottomGroup = new QGroupBox("Device & Settings", this);
    QHBoxLayout *bottomLayout = new QHBoxLayout(bottomGroup);
    bottomLayout->setContentsMargins(12, 8, 12, 8);

    QLabel *kbdLabel = new QLabel("Keyboard:", this);

    m_keyboardCombo = new QComboBox(this);
    m_keyboardCombo->setMinimumWidth(260);

    QPushButton *btnRefreshKbd = new QPushButton(this);
    btnRefreshKbd->setIcon(QIcon::fromTheme("view-refresh"));
    btnRefreshKbd->setFixedSize(28, 28);
    btnRefreshKbd->setToolTip("Refresh detected keyboards");
    connect(btnRefreshKbd, &QPushButton::clicked, this, &MainWindow::refreshKeyboardList);

    m_exclusiveGrabCheck = new QCheckBox("Exclusive Mode (block typing in system)", this);
    m_exclusiveGrabCheck->setToolTip("Prevents the system from receiving keyboard characters, sending only gamepad inputs.");

    m_logConsole = new QTextEdit(this);
    m_logConsole->setReadOnly(true);
    m_logConsole->setFixedHeight(48);
    m_logConsole->setPlaceholderText("Live events and logs...");

    bottomLayout->addWidget(kbdLabel);
    bottomLayout->addWidget(m_keyboardCombo);
    bottomLayout->addWidget(btnRefreshKbd);
    bottomLayout->addSpacing(10);
    bottomLayout->addWidget(m_exclusiveGrabCheck);
    bottomLayout->addSpacing(14);
    bottomLayout->addWidget(m_logConsole, 1);

    mainLayout->addWidget(bottomGroup);

    // ==========================================
    // 4. Status Bar
    // ==========================================
    m_statusLabel = new QLabel("Ready. Click on any button to assign a key.", this);
    statusBar()->addWidget(m_statusLabel);
}

void MainWindow::refreshProfileList() {
    m_profileCombo->blockSignals(true);
    m_profileCombo->clear();

    QStringList profiles = m_mappingManager->listSavedProfiles();
    if (!profiles.contains("Default (WASD)")) {
        profiles.prepend("Default (WASD)");
    }

    m_profileCombo->addItems(profiles);
    int idx = m_profileCombo->findText(m_mappingManager->currentProfileName());
    if (idx >= 0) {
        m_profileCombo->setCurrentIndex(idx);
    }
    m_profileCombo->blockSignals(false);
}

void MainWindow::refreshKeyboardList() {
    m_keyboardCombo->clear();
    auto list = KeyboardInterceptor::detectKeyboards();

    for (const auto &item : list) {
        m_keyboardCombo->addItem(QString("%1 (%2)").arg(item.name, item.path), item.path);
    }

    if (m_keyboardCombo->count() == 0) {
        m_keyboardCombo->addItem("/dev/input/event3 (Default Keyboard)", "/dev/input/event3");
    }
}

void MainWindow::syncUiFromMappingManager() {
    for (const auto &info : getAllGamepadButtons()) {
        if (m_buttonWidgets.contains(info.id)) {
            KeyBindingButton *btnWidget = m_buttonWidgets[info.id];
            if (m_mappingManager->hasBinding(info.id)) {
                KeyBinding b = m_mappingManager->getBinding(info.id);
                btnWidget->blockSignals(true);
                btnWidget->setBinding(b.qtKey, b.nativeScanCode, b.keyName);
                btnWidget->blockSignals(false);
            } else {
                btnWidget->blockSignals(true);
                btnWidget->clearBinding();
                btnWidget->blockSignals(false);
            }
        }
    }
}

void MainWindow::onToggleEmulationClicked() {
    if (m_interceptor->isRunningInterceptor()) {
        m_interceptor->stopInterceptor();
        m_virtualGamepad.closeDevice();

        m_btnToggleEmulation->setText("Start Emulation");
        m_btnToggleEmulation->setIcon(QIcon::fromTheme("media-playback-start"));

        QColor disabledColor = palette().color(QPalette::Disabled, QPalette::WindowText);
        m_gamepadStatusBadge->setText("Inactive");
        m_gamepadStatusBadge->setStyleSheet(QString("color: %1; font-weight: bold; margin-left: 8px;").arg(disabledColor.name()));
        m_statusLabel->setText("Emulation stopped.");
        logEvent("Gamepad emulation deactivated.");
    } else {
        if (!m_virtualGamepad.openDevice("Microsoft X-Box 360 pad")) {
            QMessageBox::critical(this, "Error /dev/uinput",
                "Could not create virtual gamepad at /dev/uinput.\n\n"
                "Please check access permissions for /dev/uinput.");
            return;
        }

        QString selectedKbd = m_keyboardCombo->currentData().toString();
        m_interceptor->setDevicePath(selectedKbd);
        m_interceptor->setGrabEnabled(m_exclusiveGrabCheck->isChecked());
        m_interceptor->updateMappings(m_mappingManager->getAllBindings());
        m_interceptor->startInterceptor();
    }
}

void MainWindow::onInterceptorStarted() {
    m_btnToggleEmulation->setText("Stop Emulation");
    m_btnToggleEmulation->setIcon(QIcon::fromTheme("media-playback-stop"));

    QColor activeColor = palette().color(QPalette::Highlight);
    m_gamepadStatusBadge->setText("Emulating: Xbox 360 pad");
    m_gamepadStatusBadge->setStyleSheet(QString("color: %1; font-weight: bold; margin-left: 8px;").arg(activeColor.name()));
    m_statusLabel->setText("Emulation active in background! Enjoy your game.");
    logEvent("Emulation started successfully! Xbox 360 controller connected.");
}

void MainWindow::onInterceptorStopped() {
    // Reset states
}

void MainWindow::onPermissionError(const QString &devicePath) {
    m_virtualGamepad.closeDevice();
    m_btnToggleEmulation->setText("Start Emulation");
    m_btnToggleEmulation->setIcon(QIcon::fromTheme("media-playback-start"));

    m_gamepadStatusBadge->setText("Permission Error");
    m_gamepadStatusBadge->setStyleSheet("color: #ef4444; font-weight: bold; margin-left: 8px;");

    QString msg = QString(
        "Permission denied reading keyboard at '%1'.\n\n"
        "Run in terminal: ./setup_permissions.sh"
    ).arg(devicePath);

    QMessageBox::warning(this, "Permission Required", msg);
    logEvent(QString("Permission denied on %1.").arg(devicePath));
}

void MainWindow::onKeyEventProcessed(int evdevCode, bool pressed) {
    for (const auto &info : getAllGamepadButtons()) {
        KeyBinding b = m_mappingManager->getBinding(info.id);
        if (b.isValid) {
            int code = (b.nativeScanCode >= 8) ? (b.nativeScanCode - 8) : b.nativeScanCode;
            if (code <= 0) code = qtKeyToLinuxEvdev(b.qtKey);

            if (code == evdevCode && pressed) {
                logEvent(QString("Live: key (%1) -> [%2]").arg(evdevCode).arg(info.name));
            }
        }
    }
}

void MainWindow::onBindingChanged(VirtualGamepadButton buttonId, int, quint32, const QString &keyName) {
    if (m_interceptor->isRunningInterceptor()) {
        m_interceptor->updateMappings(m_mappingManager->getAllBindings());
    }

    for (const auto &info : getAllGamepadButtons()) {
        if (info.id == buttonId) {
            logEvent(QString("Mapped: [%1] -> %2").arg(info.name, keyName));
            break;
        }
    }
}

void MainWindow::onBindingCleared(VirtualGamepadButton buttonId) {
    if (m_interceptor->isRunningInterceptor()) {
        m_interceptor->updateMappings(m_mappingManager->getAllBindings());
    }
    for (const auto &info : getAllGamepadButtons()) {
        if (info.id == buttonId) {
            logEvent(QString("Unbound [%1]").arg(info.name));
            break;
        }
    }
}

void MainWindow::onProfileSelected(int index) {
    if (index < 0) return;
    QString profileName = m_profileCombo->itemText(index);
    if (profileName == "Default (WASD)") {
        m_mappingManager->applyDefaultFpsProfile();
    } else {
        m_mappingManager->loadProfile(profileName);
    }
    logEvent(QString("Profile: %1").arg(profileName));
}

void MainWindow::onSaveProfileClicked() {
    QString current = m_profileCombo->currentText();
    bool ok = false;
    QString name = QInputDialog::getText(this, "Save Profile", "Profile Name:", QLineEdit::Normal, current, &ok);
    if (ok && !name.trimmed().isEmpty()) {
        if (m_mappingManager->saveProfile(name.trimmed())) {
            refreshProfileList();
            m_profileCombo->setCurrentText(name.trimmed());
            logEvent(QString("Saved: '%1'").arg(name.trimmed()));
            QMessageBox::information(this, "Success", "Profile saved!");
        } else {
            QMessageBox::warning(this, "Error", "Could not save profile.");
        }
    }
}

void MainWindow::onNewProfileClicked() {
    bool ok = false;
    QString name = QInputDialog::getText(this, "New Profile", "Profile Name:", QLineEdit::Normal, "My Game", &ok);
    if (ok && !name.trimmed().isEmpty()) {
        m_mappingManager->clearAllBindings();
        m_mappingManager->saveProfile(name.trimmed());
        refreshProfileList();
        m_profileCombo->setCurrentText(name.trimmed());
        syncUiFromMappingManager();
        logEvent(QString("New profile: %1").arg(name.trimmed()));
    }
}

void MainWindow::onResetDefaultClicked() {
    m_mappingManager->applyDefaultFpsProfile();
    refreshProfileList();
    m_profileCombo->setCurrentText("Default (WASD)");
    syncUiFromMappingManager();
    logEvent("Default profile restored.");
}

void MainWindow::onClearAllClicked() {
    auto res = QMessageBox::question(this, "Clear All", "Do you want to unbind all keys?");
    if (res == QMessageBox::Yes) {
        m_mappingManager->clearAllBindings();
        syncUiFromMappingManager();
        logEvent("All mappings unbound.");
    }
}

void MainWindow::keyPressEvent(QKeyEvent *event) {
    QMainWindow::keyPressEvent(event);
}

void MainWindow::keyReleaseEvent(QKeyEvent *event) {
    QMainWindow::keyReleaseEvent(event);
}

void MainWindow::logEvent(const QString &text) {
    QString timeStr = QDateTime::currentDateTime().toString("hh:mm:ss");
    m_logConsole->append(QString("[%1] %2").arg(timeStr, text));
}
