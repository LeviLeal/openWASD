#pragma once

#include "gamepad_types.h"
#include <QPushButton>
#include <QKeyEvent>
#include <QMouseEvent>
#include <QString>

class KeyBindingButton : public QPushButton {
    Q_OBJECT

public:
    explicit KeyBindingButton(VirtualGamepadButton buttonId, QWidget *parent = nullptr);

    VirtualGamepadButton buttonId() const { return m_buttonId; }
    int qtKey() const { return m_qtKey; }
    quint32 nativeScanCode() const { return m_nativeScanCode; }
    QString keyName() const { return m_keyName; }
    bool isBound() const { return m_isBound; }

    void setBinding(int qtKey, quint32 nativeScanCode, const QString &keyName);
    void clearBinding();
    void startListening();
    void stopListening();

signals:
    void bindingChanged(VirtualGamepadButton buttonId, int qtKey, quint32 nativeScanCode, const QString &keyName);
    void bindingCleared(VirtualGamepadButton buttonId);
    void listeningStateChanged(VirtualGamepadButton buttonId, bool isListening);

protected:
    void mousePressEvent(QMouseEvent *event) override;
    void keyPressEvent(QKeyEvent *event) override;
    void focusOutEvent(QFocusEvent *event) override;

private:
    void updateAppearance();
    QString formatKeyText(int qtKey, const QString &text) const;

    VirtualGamepadButton m_buttonId;
    int m_qtKey = 0;
    quint32 m_nativeScanCode = 0;
    QString m_keyName;
    bool m_isBound = false;
    bool m_isListening = false;
};
