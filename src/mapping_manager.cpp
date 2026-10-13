#include "mapping_manager.h"
#include <QStandardPaths>
#include <QDir>
#include <QFile>
#include <QJsonDocument>
#include <QJsonObject>
#include <QJsonArray>

MappingManager::MappingManager(QObject *parent)
    : QObject(parent)
{
    // Garante que o diretório de perfis exista
    QDir().mkpath(getProfilesDir());
    applyDefaultFpsProfile();
}

void MappingManager::setBinding(VirtualGamepadButton button, int qtKey, quint32 scanCode, const QString &keyName) {
    KeyBinding binding;
    binding.qtKey = qtKey;
    binding.nativeScanCode = scanCode;
    binding.keyName = keyName;
    binding.isValid = true;

    m_bindings[button] = binding;
    emit bindingUpdated(button, binding);
}

void MappingManager::clearBinding(VirtualGamepadButton button) {
    if (m_bindings.contains(button)) {
        m_bindings.remove(button);
        KeyBinding empty;
        emit bindingUpdated(button, empty);
    }
}

void MappingManager::clearAllBindings() {
    m_bindings.clear();
    emit allBindingsCleared();
}

bool MappingManager::hasBinding(VirtualGamepadButton button) const {
    return m_bindings.contains(button) && m_bindings[button].isValid;
}

KeyBinding MappingManager::getBinding(VirtualGamepadButton button) const {
    return m_bindings.value(button, KeyBinding{});
}

std::vector<VirtualGamepadButton> MappingManager::findButtonsForKey(int qtKey) const {
    std::vector<VirtualGamepadButton> result;
    for (auto it = m_bindings.begin(); it != m_bindings.end(); ++it) {
        if (it.value().isValid && it.value().qtKey == qtKey) {
            result.push_back(it.key());
        }
    }
    return result;
}

QString MappingManager::getProfilesDir() const {
    QString configDir = QStandardPaths::writableLocation(QStandardPaths::AppConfigLocation);
    if (configDir.isEmpty()) {
        configDir = QDir::homePath() + "/.config/wasd-anywhere";
    }
    return configDir + "/profiles";
}

void MappingManager::applyDefaultFpsProfile() {
    clearAllBindings();
    m_currentProfileName = "Default (WASD)";

    // Left Stick to classic WASD
    setBinding(VirtualGamepadButton::LeftStickUp, Qt::Key_W, 0, "W");
    setBinding(VirtualGamepadButton::LeftStickDown, Qt::Key_S, 0, "S");
    setBinding(VirtualGamepadButton::LeftStickLeft, Qt::Key_A, 0, "A");
    setBinding(VirtualGamepadButton::LeftStickRight, Qt::Key_D, 0, "D");

    // Face Buttons
    setBinding(VirtualGamepadButton::A, Qt::Key_Space, 0, "Space");
    setBinding(VirtualGamepadButton::B, Qt::Key_C, 0, "C");
    setBinding(VirtualGamepadButton::X, Qt::Key_E, 0, "E");
    setBinding(VirtualGamepadButton::Y, Qt::Key_Q, 0, "Q");

    // Ombros
    setBinding(VirtualGamepadButton::LB, Qt::Key_F, 0, "F");
    setBinding(VirtualGamepadButton::RB, Qt::Key_R, 0, "R");

    // L3 / Correr
    setBinding(VirtualGamepadButton::L3, Qt::Key_Shift, 0, "Shift");

    // Menus
    setBinding(VirtualGamepadButton::Start, Qt::Key_Tab, 0, "Tab");
    setBinding(VirtualGamepadButton::Back, Qt::Key_M, 0, "M");

    emit profileLoaded(m_currentProfileName);
}

bool MappingManager::saveProfile(const QString &profileName) {
    if (profileName.trimmed().isEmpty()) return false;

    QJsonObject rootObj;
    rootObj["profile_name"] = profileName;

    QJsonObject bindingsObj;
    for (const auto &info : getAllGamepadButtons()) {
        if (hasBinding(info.id)) {
            KeyBinding b = getBinding(info.id);
            QJsonObject item;
            item["qt_key"] = b.qtKey;
            item["scan_code"] = static_cast<qint64>(b.nativeScanCode);
            item["key_name"] = b.keyName;
            item["evdev_code"] = info.evdevCode;
            bindingsObj[QString::number(static_cast<int>(info.id))] = item;
        }
    }
    rootObj["bindings"] = bindingsObj;

    QString filePath = getProfilesDir() + "/" + profileName.trimmed() + ".json";
    QFile file(filePath);
    if (!file.open(QIODevice::WriteOnly | QIODevice::Truncate)) {
        return false;
    }

    QJsonDocument doc(rootObj);
    file.write(doc.toJson(QJsonDocument::Indented));
    file.close();

    m_currentProfileName = profileName;
    return true;
}

bool MappingManager::loadProfile(const QString &profileName) {
    QString filePath = getProfilesDir() + "/" + profileName.trimmed() + ".json";
    QFile file(filePath);
    if (!file.open(QIODevice::ReadOnly)) {
        return false;
    }

    QJsonDocument doc = QJsonDocument::fromJson(file.readAll());
    file.close();

    if (!doc.isObject()) return false;

    clearAllBindings();

    QJsonObject root = doc.object();
    QJsonObject bindingsObj = root["bindings"].toObject();

    for (const auto &info : getAllGamepadButtons()) {
        QString keyStr = QString::number(static_cast<int>(info.id));
        if (bindingsObj.contains(keyStr)) {
            QJsonObject item = bindingsObj[keyStr].toObject();
            int qtKey = item["qt_key"].toInt();
            quint32 scanCode = static_cast<quint32>(item["scan_code"].toVariant().toULongLong());
            QString keyName = item["key_name"].toString();
            setBinding(info.id, qtKey, scanCode, keyName);
        }
    }

    m_currentProfileName = profileName;
    emit profileLoaded(profileName);
    return true;
}

QStringList MappingManager::listSavedProfiles() const {
    QDir dir(getProfilesDir());
    QStringList files = dir.entryList(QStringList() << "*.json", QDir::Files);
    QStringList result;
    for (const QString &f : files) {
        result << f.chopped(5); // Remove .json
    }
    return result;
}
