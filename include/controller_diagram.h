#pragma once

#include "gamepad_types.h"
#include "mapping_manager.h"
#include <QWidget>
#include <QSet>
#include <QMap>
#include <QKeyEvent>
#include <QMouseEvent>

class ControllerDiagram : public QWidget {
    Q_OBJECT

public:
    explicit ControllerDiagram(QWidget *parent = nullptr);

    void setMappingManager(MappingManager *manager);
    void setButtonPressed(VirtualGamepadButton button, bool pressed);
    void clearAllPressed();
    bool isButtonPressed(VirtualGamepadButton button) const;

    bool isListening() const { return m_listeningButton.has_value(); }
    void cancelListening();

signals:
    void bindingChanged(VirtualGamepadButton button, int qtKey, quint32 nativeScanCode, const QString &keyName);
    void bindingCleared(VirtualGamepadButton button);
    void listeningStateChanged(bool isListening);

protected:
    void paintEvent(QPaintEvent *event) override;
    void mousePressEvent(QMouseEvent *event) override;
    void keyPressEvent(QKeyEvent *event) override;
    void focusOutEvent(QFocusEvent *event) override;
    QSize minimumSizeHint() const override { return QSize(750, 480); }
    QSize sizeHint() const override { return QSize(860, 540); }

private:
    struct ButtonTarget {
        VirtualGamepadButton id;
        QRectF buttonRect;
        QRectF badgeRect;
        QString label;
        QColor activeColor;
        bool isCircle = false;
    };

    QList<ButtonTarget> calculateTargets(const QRectF &bounds) const;
    QString formatKeyText(int qtKey, const QString &text) const;

    MappingManager *m_mappingManager = nullptr;
    QSet<VirtualGamepadButton> m_pressedButtons;
    std::optional<VirtualGamepadButton> m_listeningButton;

    // Deslocamento visual dos analógicos
    QPointF m_leftStickOffset;
    QPointF m_rightStickOffset;
    void updateStickOffsets();
};
