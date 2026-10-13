#include "key_binding_button.h"
#include <QKeySequence>
#include <QMenu>

KeyBindingButton::KeyBindingButton(VirtualGamepadButton buttonId, QWidget *parent)
    : QPushButton(parent)
    , m_buttonId(buttonId)
{
    setFocusPolicy(Qt::StrongFocus);
    setCursor(Qt::PointingHandCursor);
    setContextMenuPolicy(Qt::CustomContextMenu);

    connect(this, &QWidget::customContextMenuRequested, this, [this](const QPoint &pos) {
        if (m_isListening) return;
        QMenu menu(this);
        QAction *clearAction = menu.addAction("Unbind Key (Clear)");
        connect(clearAction, &QAction::triggered, this, &KeyBindingButton::clearBinding);
        menu.exec(mapToGlobal(pos));
    });

    updateAppearance();
}

void KeyBindingButton::setBinding(int qtKey, quint32 nativeScanCode, const QString &keyName) {
    m_qtKey = qtKey;
    m_nativeScanCode = nativeScanCode;
    m_keyName = keyName;
    m_isBound = true;
    m_isListening = false;
    updateAppearance();
    emit bindingChanged(m_buttonId, m_qtKey, m_nativeScanCode, m_keyName);
}

void KeyBindingButton::clearBinding() {
    m_qtKey = 0;
    m_nativeScanCode = 0;
    m_keyName.clear();
    m_isBound = false;
    m_isListening = false;
    updateAppearance();
    emit bindingCleared(m_buttonId);
}

void KeyBindingButton::startListening() {
    m_isListening = true;
    setFocus();
    grabKeyboard();
    updateAppearance();
    emit listeningStateChanged(m_buttonId, true);
}

void KeyBindingButton::stopListening() {
    if (m_isListening) {
        m_isListening = false;
        releaseKeyboard();
        updateAppearance();
        emit listeningStateChanged(m_buttonId, false);
    }
}

void KeyBindingButton::mousePressEvent(QMouseEvent *event) {
    if (event->button() == Qt::LeftButton) {
        if (!m_isListening) {
            startListening();
        } else {
            stopListening();
        }
        event->accept();
        return;
    }
    QPushButton::mousePressEvent(event);
}

void KeyBindingButton::keyPressEvent(QKeyEvent *event) {
    if (!m_isListening) {
        QPushButton::keyPressEvent(event);
        return;
    }

    int key = event->key();

    if (key == Qt::Key_Escape) {
        stopListening();
        event->accept();
        return;
    }

    QString keyText = formatKeyText(key, event->text());
    quint32 scanCode = event->nativeScanCode();

    stopListening();
    setBinding(key, scanCode, keyText);
    event->accept();
}

void KeyBindingButton::focusOutEvent(QFocusEvent *event) {
    if (m_isListening) {
        stopListening();
    }
    QPushButton::focusOutEvent(event);
}

QString KeyBindingButton::formatKeyText(int qtKey, const QString &text) const {
    switch (qtKey) {
        case Qt::Key_Space: return "Space";
        case Qt::Key_Return:
        case Qt::Key_Enter: return "Enter";
        case Qt::Key_Tab: return "Tab";
        case Qt::Key_Shift: return "Shift";
        case Qt::Key_Control: return "Ctrl";
        case Qt::Key_Alt: return "Alt";
        case Qt::Key_AltGr: return "AltGr";
        case Qt::Key_Meta: return "Super";
        case Qt::Key_CapsLock: return "Caps Lock";
        case Qt::Key_Backspace: return "Backspace";
        case Qt::Key_Delete: return "Delete";
        case Qt::Key_Insert: return "Insert";
        case Qt::Key_Home: return "Home";
        case Qt::Key_End: return "End";
        case Qt::Key_PageUp: return "Page Up";
        case Qt::Key_PageDown: return "Page Down";
        case Qt::Key_Up: return "Up Arrow";
        case Qt::Key_Down: return "Down Arrow";
        case Qt::Key_Left: return "Left Arrow";
        case Qt::Key_Right: return "Right Arrow";
        default: break;
    }

    if (qtKey >= Qt::Key_F1 && qtKey <= Qt::Key_F12) {
        return QString("F%1").arg(qtKey - Qt::Key_F1 + 1);
    }

    if (!text.isEmpty() && text[0].isPrint()) {
        return text.toUpper();
    }

    return QKeySequence(qtKey).toString();
}

void KeyBindingButton::updateAppearance() {
    QPalette pal = palette();
    QColor highlight = pal.color(QPalette::Highlight);
    QColor disabledText = pal.color(QPalette::Disabled, QPalette::ButtonText);

    if (m_isListening) {
        setText("Press a key...");
        setStyleSheet(QString(
            "QPushButton {"
            "   font-weight: bold;"
            "   border: 2px solid %1;"
            "   border-radius: 4px;"
            "   padding: 5px 12px;"
            "}"
        ).arg(highlight.name()));
    } else if (m_isBound) {
        setText(m_keyName);
        setStyleSheet(QString(
            "QPushButton {"
            "   font-weight: bold;"
            "   color: %1;"
            "   border: 1px solid %2;"
            "   border-radius: 4px;"
            "   padding: 5px 12px;"
            "}"
        ).arg(highlight.name(), highlight.name()));
    } else {
        setText("—");
        setStyleSheet(QString(
            "QPushButton {"
            "   color: %1;"
            "   border: 1px dashed %2;"
            "   border-radius: 4px;"
            "   padding: 5px 12px;"
            "}"
        ).arg(disabledText.name(), disabledText.name()));
    }
}
