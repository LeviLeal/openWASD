#include "controller_diagram.h"
#include <QPainter>
#include <QPainterPath>
#include <QMenu>
#include <QKeySequence>

ControllerDiagram::ControllerDiagram(QWidget *parent)
    : QWidget(parent)
{
    setSizePolicy(QSizePolicy::Expanding, QSizePolicy::Expanding);
    setFocusPolicy(Qt::StrongFocus);
    setCursor(Qt::PointingHandCursor);
    setContextMenuPolicy(Qt::CustomContextMenu);

    connect(this, &QWidget::customContextMenuRequested, this, [this](const QPoint &pos) {
        if (m_listeningButton.has_value()) return;

        auto targets = calculateTargets(rect());
        for (const auto &t : targets) {
            if (t.buttonRect.contains(pos) || t.badgeRect.contains(pos)) {
                QMenu menu(this);
                menu.setStyleSheet(
                    "QMenu { background-color: #182234; color: #f8fafc; border: 1px solid #334155; border-radius: 6px; padding: 4px; font-size: 12px; }"
                    "QMenu::item { padding: 5px 16px; border-radius: 4px; }"
                    "QMenu::item:selected { background-color: #ef4444; color: white; }"
                );
                QAction *clearAct = menu.addAction(QString("Limpar tecla de %1").arg(t.label));
                connect(clearAct, &QAction::triggered, this, [this, id = t.id]() {
                    if (m_mappingManager) {
                        m_mappingManager->clearBinding(id);
                    }
                    emit bindingCleared(id);
                    update();
                });
                menu.exec(mapToGlobal(pos));
                return;
            }
        }
    });
}

void ControllerDiagram::setMappingManager(MappingManager *manager) {
    m_mappingManager = manager;
    if (m_mappingManager) {
        connect(m_mappingManager, &MappingManager::bindingUpdated, this, [this]() { update(); });
        connect(m_mappingManager, &MappingManager::allBindingsCleared, this, [this]() { update(); });
        connect(m_mappingManager, &MappingManager::profileLoaded, this, [this]() { update(); });
    }
    update();
}

void ControllerDiagram::setButtonPressed(VirtualGamepadButton button, bool pressed) {
    if (pressed) {
        m_pressedButtons.insert(button);
    } else {
        m_pressedButtons.remove(button);
    }
    updateStickOffsets();
    update();
}

void ControllerDiagram::clearAllPressed() {
    m_pressedButtons.clear();
    m_leftStickOffset = QPointF(0, 0);
    m_rightStickOffset = QPointF(0, 0);
    update();
}

bool ControllerDiagram::isButtonPressed(VirtualGamepadButton button) const {
    return m_pressedButtons.contains(button);
}

void ControllerDiagram::cancelListening() {
    if (m_listeningButton.has_value()) {
        m_listeningButton = std::nullopt;
        releaseKeyboard();
        emit listeningStateChanged(false);
        update();
    }
}

void ControllerDiagram::updateStickOffsets() {
    qreal lsX = 0, lsY = 0;
    if (m_pressedButtons.contains(VirtualGamepadButton::LeftStickUp)) lsY -= 10;
    if (m_pressedButtons.contains(VirtualGamepadButton::LeftStickDown)) lsY += 10;
    if (m_pressedButtons.contains(VirtualGamepadButton::LeftStickLeft)) lsX -= 10;
    if (m_pressedButtons.contains(VirtualGamepadButton::LeftStickRight)) lsX += 10;
    m_leftStickOffset = QPointF(lsX, lsY);

    qreal rsX = 0, rsY = 0;
    if (m_pressedButtons.contains(VirtualGamepadButton::RightStickUp)) rsY -= 10;
    if (m_pressedButtons.contains(VirtualGamepadButton::RightStickDown)) rsY += 10;
    if (m_pressedButtons.contains(VirtualGamepadButton::RightStickLeft)) rsX -= 10;
    if (m_pressedButtons.contains(VirtualGamepadButton::RightStickRight)) rsX += 10;
    m_rightStickOffset = QPointF(rsX, rsY);
}

QList<ControllerDiagram::ButtonTarget> ControllerDiagram::calculateTargets(const QRectF &bounds) const {
    QList<ButtonTarget> list;
    qreal cx = bounds.center().x();
    qreal cy = bounds.center().y();

    // ==========================================
    // 1. OMBROS E GATILHOS (LT, LB / RT, RB)
    // ==========================================
    // Esquerda
    list.append({ VirtualGamepadButton::LT,
                  QRectF(cx - 310, cy - 220, 68, 28),
                  QRectF(cx - 310, cy - 188, 68, 22),
                  "LT", QColor("#38bdf8") });

    list.append({ VirtualGamepadButton::LB,
                  QRectF(cx - 310, cy - 145, 68, 28),
                  QRectF(cx - 310, cy - 113, 68, 22),
                  "LB", QColor("#0ea5e9") });

    // Direita
    list.append({ VirtualGamepadButton::RT,
                  QRectF(cx + 242, cy - 220, 68, 28),
                  QRectF(cx + 242, cy - 188, 68, 22),
                  "RT", QColor("#38bdf8") });

    list.append({ VirtualGamepadButton::RB,
                  QRectF(cx + 242, cy - 145, 68, 28),
                  QRectF(cx + 242, cy - 113, 68, 22),
                  "RB", QColor("#0ea5e9") });

    // ==========================================
    // 2. ANALÓGICO ESQUERDO (LS - WASD)
    // Centro em (cx - 165, cy - 35)
    // ==========================================
    qreal lsX = cx - 165;
    qreal lsY = cy - 35;
    qreal stickR = 26; // diâmetro 52px

    list.append({ VirtualGamepadButton::L3,
                  QRectF(lsX - stickR, lsY - stickR, stickR*2, stickR*2),
                  QRectF(lsX - 35, lsY + stickR + 6, 70, 22),
                  "L3", QColor("#06b6d4"), true });

    // Direcionais WASD bem espaçados
    list.append({ VirtualGamepadButton::LeftStickUp,
                  QRectF(lsX - 18, lsY - stickR - 38, 36, 24),
                  QRectF(lsX - 25, lsY - stickR - 64, 50, 22),
                  "▲", QColor("#22d3ee") });

    list.append({ VirtualGamepadButton::LeftStickDown,
                  QRectF(lsX - 18, lsY + stickR + 32, 36, 24),
                  QRectF(lsX - 25, lsY + stickR + 60, 50, 22),
                  "▼", QColor("#22d3ee") });

    list.append({ VirtualGamepadButton::LeftStickLeft,
                  QRectF(lsX - stickR - 44, lsY - 12, 36, 24),
                  QRectF(lsX - stickR - 100, lsY - 11, 52, 22),
                  "◀", QColor("#22d3ee") });

    list.append({ VirtualGamepadButton::LeftStickRight,
                  QRectF(lsX + stickR + 8, lsY - 12, 36, 24),
                  QRectF(lsX + stickR + 48, lsY - 11, 52, 22),
                  "▶", QColor("#22d3ee") });

    // ==========================================
    // 3. D-PAD (DIRECIONAL DIGITAL)
    // Centro em (cx - 165, cy + 155) - bem separado do LS
    // ==========================================
    qreal dpX = cx - 165;
    qreal dpY = cy + 155;
    qreal dpS = 28;

    list.append({ VirtualGamepadButton::DpadUp,
                  QRectF(dpX - dpS/2, dpY - dpS - 6, dpS, dpS),
                  QRectF(dpX - 32, dpY - dpS - 34, 64, 22),
                  "▲", QColor("#a855f7") });

    list.append({ VirtualGamepadButton::DpadDown,
                  QRectF(dpX - dpS/2, dpY + 6, dpS, dpS),
                  QRectF(dpX - 32, dpY + dpS + 12, 64, 22),
                  "▼", QColor("#a855f7") });

    list.append({ VirtualGamepadButton::DpadLeft,
                  QRectF(dpX - dpS - 6, dpY - dpS/2, dpS, dpS),
                  QRectF(dpX - dpS - 74, dpY - 11, 64, 22),
                  "◀", QColor("#a855f7") });

    list.append({ VirtualGamepadButton::DpadRight,
                  QRectF(dpX + 6, dpY - dpS/2, dpS, dpS),
                  QRectF(dpX + dpS + 12, dpY - 11, 64, 22),
                  "▶", QColor("#a855f7") });

    // ==========================================
    // 4. FACE BUTTONS (Y, X, B, A)
    // Centro em (cx + 175, cy - 35)
    // ==========================================
    qreal fbX = cx + 175;
    qreal fbY = cy - 35;
    qreal fbR = 17; // diâmetro 34px
    qreal dist = 52;

    // Y (Topo - Amarelo)
    list.append({ VirtualGamepadButton::Y,
                  QRectF(fbX - fbR, fbY - dist - fbR, fbR*2, fbR*2),
                  QRectF(fbX - 32, fbY - dist - fbR - 26, 64, 22),
                  "Y", QColor("#eab308"), true });

    // A (Baixo - Verde)
    list.append({ VirtualGamepadButton::A,
                  QRectF(fbX - fbR, fbY + dist - fbR, fbR*2, fbR*2),
                  QRectF(fbX - 32, fbY + dist + fbR + 6, 64, 22),
                  "A", QColor("#22c55e"), true });

    // X (Esquerda - Azul)
    list.append({ VirtualGamepadButton::X,
                  QRectF(fbX - dist - fbR, fbY - fbR, fbR*2, fbR*2),
                  QRectF(fbX - dist - fbR - 70, fbY - 11, 66, 22),
                  "X", QColor("#3b82f6"), true });

    // B (Direita - Vermelho)
    list.append({ VirtualGamepadButton::B,
                  QRectF(fbX + dist - fbR, fbY - fbR, fbR*2, fbR*2),
                  QRectF(fbX + dist + fbR + 8, fbY - 11, 66, 22),
                  "B", QColor("#ef4444"), true });

    // ==========================================
    // 5. ANALÓGICO DIREITO (RS - CÂMERA)
    // Centro em (cx + 90, cy + 135)
    // ==========================================
    qreal rsX = cx + 90;
    qreal rsY = cy + 135;

    list.append({ VirtualGamepadButton::R3,
                  QRectF(rsX - stickR, rsY - stickR, stickR*2, stickR*2),
                  QRectF(rsX - 35, rsY + stickR + 6, 70, 22),
                  "R3", QColor("#06b6d4"), true });

    list.append({ VirtualGamepadButton::RightStickUp,
                  QRectF(rsX - 18, rsY - stickR - 38, 36, 24),
                  QRectF(rsX - 25, rsY - stickR - 64, 50, 22),
                  "▲", QColor("#22d3ee") });

    list.append({ VirtualGamepadButton::RightStickDown,
                  QRectF(rsX - 18, rsY + stickR + 32, 36, 24),
                  QRectF(rsX - 25, rsY + stickR + 60, 50, 22),
                  "▼", QColor("#22d3ee") });

    list.append({ VirtualGamepadButton::RightStickLeft,
                  QRectF(rsX - stickR - 44, rsY - 12, 36, 24),
                  QRectF(rsX - stickR - 100, rsY - 11, 52, 22),
                  "◀", QColor("#22d3ee") });

    list.append({ VirtualGamepadButton::RightStickRight,
                  QRectF(rsX + stickR + 8, rsY - 12, 36, 24),
                  QRectF(rsX + stickR + 48, rsY - 11, 52, 22),
                  "▶", QColor("#22d3ee") });

    // ==========================================
    // 6. MENUS CENTRAIS (BACK, GUIDE, START)
    // ==========================================
    list.append({ VirtualGamepadButton::Back,
                  QRectF(cx - 75, cy - 45, 46, 24),
                  QRectF(cx - 75, cy - 75, 46, 22),
                  "Back", QColor("#94a3b8") });

    list.append({ VirtualGamepadButton::Guide,
                  QRectF(cx - 18, cy - 58, 36, 36),
                  QRectF(cx - 28, cy - 90, 56, 22),
                  "Xbox", QColor("#10b981"), true });

    list.append({ VirtualGamepadButton::Start,
                  QRectF(cx + 29, cy - 45, 46, 24),
                  QRectF(cx + 29, cy - 75, 46, 22),
                  "Start", QColor("#94a3b8") });

    return list;
}

void ControllerDiagram::paintEvent(QPaintEvent *) {
    QPainter p(this);
    p.setRenderHint(QPainter::Antialiasing, true);

    QRectF bounds = rect();
    qreal cx = bounds.center().x();
    qreal cy = bounds.center().y();

    // 1. Fundo Escuro Minimalista
    p.fillRect(bounds, QColor("#080d1a"));

    // 2. Chassi Silhueta do Gamepad (Limpo e Espaçoso)
    qreal padW = 590;
    qreal padH = 340;
    QRectF padRect(cx - padW/2, cy - padH/2, padW, padH);

    QPainterPath bodyPath;
    bodyPath.addRoundedRect(padRect, 70, 70);

    QLinearGradient bodyGrad(padRect.topLeft(), padRect.bottomRight());
    bodyGrad.setColorAt(0.0, QColor("#162032"));
    bodyGrad.setColorAt(1.0, QColor("#0f172a"));
    p.setBrush(bodyGrad);
    p.setPen(QPen(QColor("#27354f"), 2));
    p.drawPath(bodyPath);

    // Grips laterais sutis
    p.setPen(Qt::NoPen);
    p.setBrush(QColor("#0b1220"));
    p.drawRoundedRect(QRectF(cx - padW/2 + 10, cy - 40, 45, 160), 22, 22);
    p.drawRoundedRect(QRectF(cx + padW/2 - 55, cy - 40, 45, 160), 22, 22);

    // Anéis base dos analógicos
    p.setPen(QPen(QColor("#27354f"), 1.5));
    p.setBrush(QColor("#0d1525"));
    p.drawEllipse(QPointF(cx - 165, cy - 35), 36, 36);
    p.drawEllipse(QPointF(cx + 90, cy + 135), 36, 36);

    // 3. Renderização dos Botões e Badges
    auto targets = calculateTargets(bounds);

    for (const auto &t : targets) {
        bool isPressed = m_pressedButtons.contains(t.id);
        bool isListeningThis = (m_listeningButton.has_value() && m_listeningButton.value() == t.id);

        QString boundKeyText = "—";
        bool hasBinding = false;
        if (m_mappingManager && m_mappingManager->hasBinding(t.id)) {
            boundKeyText = m_mappingManager->getBinding(t.id).keyName;
            hasBinding = true;
        }

        // Deslocamento físico suave do joystick
        QRectF renderBtnRect = t.buttonRect;
        if (t.id == VirtualGamepadButton::L3) {
            renderBtnRect.translate(m_leftStickOffset);
        } else if (t.id == VirtualGamepadButton::R3) {
            renderBtnRect.translate(m_rightStickOffset);
        }

        // Linha de conexão sutil e elegante
        if (!t.badgeRect.isEmpty()) {
            p.setPen(QPen(hasBinding ? QColor(t.activeColor.red(), t.activeColor.green(), t.activeColor.blue(), 60) : QColor("#1e293b"), 1, Qt::DotLine));
            p.drawLine(renderBtnRect.center(), t.badgeRect.center());
        }

        // Iluminação / Glow
        if (isListeningThis) {
            p.setPen(Qt::NoPen);
            p.setBrush(QColor(245, 158, 11, 80));
            if (t.isCircle) p.drawEllipse(renderBtnRect.adjusted(-5, -5, 5, 5));
            else p.drawRoundedRect(renderBtnRect.adjusted(-5, -5, 5, 5), 8, 8);

            p.setBrush(QColor("#f59e0b"));
            p.setPen(QPen(Qt::white, 2));
        } else if (isPressed) {
            p.setPen(Qt::NoPen);
            p.setBrush(QColor(t.activeColor.red(), t.activeColor.green(), t.activeColor.blue(), 100));
            if (t.isCircle) p.drawEllipse(renderBtnRect.adjusted(-5, -5, 5, 5));
            else p.drawRoundedRect(renderBtnRect.adjusted(-5, -5, 5, 5), 8, 8);

            p.setBrush(t.activeColor);
            p.setPen(QPen(Qt::white, 2));
        } else {
            p.setBrush(hasBinding ? QColor("#192336") : QColor("#131c2d"));
            p.setPen(QPen(hasBinding ? t.activeColor.darker(130) : QColor("#334155"), 1.2));
        }

        // Desenhar botão
        if (t.isCircle) {
            p.drawEllipse(renderBtnRect);
        } else {
            p.drawRoundedRect(renderBtnRect, 6, 6);
        }

        // Texto do Botão (simples e conciso)
        p.setPen(isPressed ? Qt::white : QColor("#f1f5f9"));
        QFont f = p.font();
        f.setBold(true);
        f.setPointSize(renderBtnRect.width() > 50 ? 9 : 8);
        p.setFont(f);
        p.drawText(renderBtnRect, Qt::AlignCenter, t.label);

        // 4. Badge da Tecla (Limpa, Direta, Sem Brackets)
        if (!t.badgeRect.isEmpty()) {
            if (isListeningThis) {
                p.setBrush(QColor("#b45309"));
                p.setPen(QPen(QColor("#fbbf24"), 1.5));
                p.drawRoundedRect(t.badgeRect, 4, 4);

                p.setPen(Qt::white);
                QFont bf = p.font();
                bf.setPointSize(8);
                bf.setBold(true);
                p.setFont(bf);
                p.drawText(t.badgeRect, Qt::AlignCenter, "Aperte...");
            } else {
                if (hasBinding) {
                    p.setBrush(isPressed ? t.activeColor.darker(180) : QColor("#0d1525"));
                    p.setPen(QPen(isPressed ? t.activeColor : QColor("#38bdf8"), 1.2));
                } else {
                    p.setBrush(QColor("#0a0f1d"));
                    p.setPen(QPen(QColor("#1e293b"), 1.0));
                }
                p.drawRoundedRect(t.badgeRect, 4, 4);

                // Texto limpo sem bracket nenhum!
                p.setPen(hasBinding ? (isPressed ? Qt::white : QColor("#38bdf8")) : QColor("#475569"));
                QFont bf = p.font();
                bf.setPointSize(8);
                bf.setBold(hasBinding);
                p.setFont(bf);
                p.drawText(t.badgeRect, Qt::AlignCenter, hasBinding ? boundKeyText : "—");
            }
        }
    }

    // Instrução simples no rodapé
    p.setPen(QColor("#64748b"));
    QFont hintFont = p.font();
    hintFont.setPointSize(9);
    p.setFont(hintFont);
    p.drawText(QRectF(cx - 250, padRect.bottom() + 15, 500, 20), Qt::AlignCenter,
        m_listeningButton.has_value()
            ? "Pressione a tecla no teclado (ESC para cancelar)"
            : "Clique para associar • Botão direito para limpar");
}

void ControllerDiagram::mousePressEvent(QMouseEvent *event) {
    if (event->button() == Qt::LeftButton) {
        auto targets = calculateTargets(rect());
        for (const auto &t : targets) {
            if (t.buttonRect.contains(event->pos()) || t.badgeRect.contains(event->pos())) {
                m_listeningButton = t.id;
                setFocus();
                grabKeyboard();
                emit listeningStateChanged(true);
                update();
                event->accept();
                return;
            }
        }

        if (m_listeningButton.has_value()) {
            cancelListening();
            event->accept();
            return;
        }
    }
    QWidget::mousePressEvent(event);
}

void ControllerDiagram::keyPressEvent(QKeyEvent *event) {
    if (!m_listeningButton.has_value()) {
        QWidget::keyPressEvent(event);
        return;
    }

    int key = event->key();
    if (key == Qt::Key_Escape) {
        cancelListening();
        event->accept();
        return;
    }

    VirtualGamepadButton btn = m_listeningButton.value();
    quint32 scanCode = event->nativeScanCode();
    QString keyText = formatKeyText(key, event->text());

    m_listeningButton = std::nullopt;
    releaseKeyboard();

    if (m_mappingManager) {
        m_mappingManager->setBinding(btn, key, scanCode, keyText);
    }

    emit bindingChanged(btn, key, scanCode, keyText);
    emit listeningStateChanged(false);

    update();
    event->accept();
}

void ControllerDiagram::focusOutEvent(QFocusEvent *event) {
    if (m_listeningButton.has_value()) {
        cancelListening();
    }
    QWidget::focusOutEvent(event);
}

QString ControllerDiagram::formatKeyText(int qtKey, const QString &text) const {
    switch (qtKey) {
        case Qt::Key_Space: return "Espaço";
        case Qt::Key_Return:
        case Qt::Key_Enter: return "Enter";
        case Qt::Key_Tab: return "Tab";
        case Qt::Key_Shift: return "Shift";
        case Qt::Key_Control: return "Ctrl";
        case Qt::Key_Alt: return "Alt";
        case Qt::Key_AltGr: return "AltGr";
        case Qt::Key_Meta: return "Super";
        case Qt::Key_CapsLock: return "Caps";
        case Qt::Key_Backspace: return "Backsp";
        case Qt::Key_Delete: return "Del";
        case Qt::Key_Up: return "Cima";
        case Qt::Key_Down: return "Baixo";
        case Qt::Key_Left: return "Esq";
        case Qt::Key_Right: return "Dir";
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
