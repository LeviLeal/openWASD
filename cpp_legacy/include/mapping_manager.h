#pragma once

#include "gamepad_types.h"
#include <QObject>
#include <QMap>
#include <QString>

struct KeyBinding {
    int qtKey = 0;
    quint32 nativeScanCode = 0;
    QString keyName;
    bool isValid = false;
};

class MappingManager : public QObject {
    Q_OBJECT

public:
    explicit MappingManager(QObject *parent = nullptr);

    void setBinding(VirtualGamepadButton button, int qtKey, quint32 scanCode, const QString &keyName);
    void clearBinding(VirtualGamepadButton button);
    void clearAllBindings();

    bool hasBinding(VirtualGamepadButton button) const;
    KeyBinding getBinding(VirtualGamepadButton button) const;
    const QMap<VirtualGamepadButton, KeyBinding>& getAllBindings() const { return m_bindings; }

    // Encontrar qual botão do controle está associado a uma tecla
    std::vector<VirtualGamepadButton> findButtonsForKey(int qtKey) const;

    // Perfis
    bool saveProfile(const QString &profileName);
    bool loadProfile(const QString &profileName);
    QStringList listSavedProfiles() const;
    void applyDefaultFpsProfile();

    QString currentProfileName() const { return m_currentProfileName; }

signals:
    void bindingUpdated(VirtualGamepadButton button, const KeyBinding &binding);
    void allBindingsCleared();
    void profileLoaded(const QString &profileName);

private:
    QString getProfilesDir() const;

    QMap<VirtualGamepadButton, KeyBinding> m_bindings;
    QString m_currentProfileName = "Default (WASD)";
};
