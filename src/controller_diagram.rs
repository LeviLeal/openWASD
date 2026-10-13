use crate::gamepad_types::VirtualGamepadButton;
use crate::mapping_manager::MappingManager;
use egui::{
    pos2, vec2, Align2, Color32, FontFamily, FontId, Pos2, Rect, Sense, Stroke, StrokeKind, Ui,
};
use std::collections::HashSet;

pub struct ControllerDiagram {
    pub pressed_buttons: HashSet<VirtualGamepadButton>,
    pub listening_button: Option<VirtualGamepadButton>,
}

struct ButtonTarget {
    id: VirtualGamepadButton,
    button_rect: Rect,
    badge_rect: Rect,
    label: &'static str,
    color: Color32,
    is_circle: bool,
}

impl ControllerDiagram {
    pub fn new() -> Self {
        Self {
            pressed_buttons: HashSet::new(),
            listening_button: None,
        }
    }

    pub fn show(
        &mut self,
        ui: &mut Ui,
        mapping_manager: &mut MappingManager,
    ) -> Option<VirtualGamepadButton> {
        let (rect, response) = ui.allocate_exact_size(
            vec2(ui.available_width().max(760.0), 480.0),
            Sense::click(),
        );

        let painter = ui.painter_at(rect);
        let center = rect.center();

        // Escala responsiva em relação à base 840x480
        let scale = (rect.width() / 840.0).min(rect.height() / 480.0).clamp(0.8, 1.25);

        // Fundo sutil do container
        painter.rect_filled(
            rect,
            12.0,
            Color32::from_rgb(15, 23, 42), // Slate 900
        );
        painter.rect_stroke(
            rect,
            12.0,
            Stroke::new(1.0, Color32::from_rgb(30, 41, 59)),
            StrokeKind::Inside,
        );

        // Desenhar corpo estilizado do controle Xbox 360
        let body_w = 480.0 * scale;
        let body_h = 300.0 * scale;
        let body_rect = Rect::from_center_size(center, vec2(body_w, body_h));

        painter.rect_filled(
            body_rect,
            body_h * 0.45,
            Color32::from_rgb(30, 41, 59), // Slate 800
        );
        painter.rect_stroke(
            body_rect,
            body_h * 0.45,
            Stroke::new(2.0, Color32::from_rgb(51, 65, 85)),
            StrokeKind::Inside,
        );

        // Grips laterais (asas inferiores)
        let grip_w = 110.0 * scale;
        let grip_h = 160.0 * scale;
        let left_grip = Rect::from_center_size(
            pos2(center.x - 200.0 * scale, center.y + 110.0 * scale),
            vec2(grip_w, grip_h),
        );
        let right_grip = Rect::from_center_size(
            pos2(center.x + 200.0 * scale, center.y + 110.0 * scale),
            vec2(grip_w, grip_h),
        );
        painter.rect_filled(left_grip, 40.0 * scale, Color32::from_rgb(26, 36, 52));
        painter.rect_filled(right_grip, 40.0 * scale, Color32::from_rgb(26, 36, 52));

        // Calcular alvos interativos
        let targets = self.calculate_targets(center, scale);
        let mut clicked_button = None;

        let mouse_pos = response.hover_pos();
        let clicked = response.clicked();

        for t in &targets {
            let is_pressed = self.pressed_buttons.contains(&t.id);
            let is_listening = self.listening_button == Some(t.id);
            let is_bound = mapping_manager.has_binding(t.id);
            let bound_text = mapping_manager
                .get_binding(t.id)
                .map(|b| b.key_name.as_str())
                .unwrap_or("None");

            let is_hovered = mouse_pos
                .map(|p| t.button_rect.contains(p) || t.badge_rect.contains(p))
                .unwrap_or(false);

            if clicked && is_hovered {
                clicked_button = Some(t.id);
            }

            // Cores do botão
            let base_color = if is_listening {
                Color32::from_rgb(245, 158, 11) // Amber pulsante
            } else if is_pressed {
                t.color
            } else if is_hovered {
                Color32::from_rgb(71, 85, 105)
            } else {
                Color32::from_rgb(30, 41, 59)
            };

            let stroke_color = if is_listening {
                Color32::from_rgb(251, 191, 36)
            } else if is_pressed {
                Color32::WHITE
            } else if is_bound {
                t.color
            } else {
                Color32::from_rgb(71, 85, 105)
            };

            // Desenhar botão
            if t.is_circle {
                painter.circle_filled(t.button_rect.center(), t.button_rect.width() / 2.0, base_color);
                painter.circle_stroke(
                    t.button_rect.center(),
                    t.button_rect.width() / 2.0,
                    Stroke::new(if is_listening || is_pressed { 2.5 } else { 1.5 }, stroke_color),
                );
            } else {
                painter.rect_filled(t.button_rect, 6.0 * scale, base_color);
                painter.rect_stroke(
                    t.button_rect,
                    6.0 * scale,
                    Stroke::new(if is_listening || is_pressed { 2.5 } else { 1.5 }, stroke_color),
                    StrokeKind::Inside,
                );
            }

            // Rótulo do botão
            let text_color = if is_pressed || is_listening {
                Color32::WHITE
            } else {
                Color32::from_rgb(226, 232, 240)
            };

            painter.text(
                t.button_rect.center(),
                Align2::CENTER_CENTER,
                t.label,
                FontId::new(12.0 * scale, FontFamily::Proportional),
                text_color,
            );

            // Badge com a tecla atribuída
            let badge_bg = if is_listening {
                Color32::from_rgb(180, 83, 9)
            } else if is_bound {
                Color32::from_rgb(30, 41, 59)
            } else {
                Color32::from_rgb(15, 23, 42)
            };

            painter.rect_filled(t.badge_rect, 5.0 * scale, badge_bg);
            painter.rect_stroke(
                t.badge_rect,
                5.0 * scale,
                Stroke::new(1.0, stroke_color),
                StrokeKind::Inside,
            );

            let badge_text = if is_listening {
                "Press key..."
            } else if is_bound {
                bound_text
            } else {
                "-"
            };

            painter.text(
                t.badge_rect.center(),
                Align2::CENTER_CENTER,
                badge_text,
                FontId::new(10.5 * scale, FontFamily::Monospace),
                if is_bound || is_listening {
                    Color32::WHITE
                } else {
                    Color32::from_rgb(148, 163, 184)
                },
            );
        }

        clicked_button
    }

    fn calculate_targets(&self, center: Pos2, scale: f32) -> Vec<ButtonTarget> {
        let cx = center.x;
        let cy = center.y;
        let mut list = Vec::new();

        // 1. Shoulders & Triggers
        // Esquerda
        list.push(ButtonTarget {
            id: VirtualGamepadButton::LT,
            button_rect: Rect::from_center_size(
                pos2(cx - 210.0 * scale, cy - 170.0 * scale),
                vec2(58.0 * scale, 24.0 * scale),
            ),
            badge_rect: Rect::from_center_size(
                pos2(cx - 210.0 * scale, cy - 144.0 * scale),
                vec2(58.0 * scale, 18.0 * scale),
            ),
            label: "LT",
            color: Color32::from_rgb(56, 189, 248),
            is_circle: false,
        });

        list.push(ButtonTarget {
            id: VirtualGamepadButton::LB,
            button_rect: Rect::from_center_size(
                pos2(cx - 210.0 * scale, cy - 110.0 * scale),
                vec2(58.0 * scale, 24.0 * scale),
            ),
            badge_rect: Rect::from_center_size(
                pos2(cx - 210.0 * scale, cy - 84.0 * scale),
                vec2(58.0 * scale, 18.0 * scale),
            ),
            label: "LB",
            color: Color32::from_rgb(14, 165, 233),
            is_circle: false,
        });

        // Direita
        list.push(ButtonTarget {
            id: VirtualGamepadButton::RT,
            button_rect: Rect::from_center_size(
                pos2(cx + 210.0 * scale, cy - 170.0 * scale),
                vec2(58.0 * scale, 24.0 * scale),
            ),
            badge_rect: Rect::from_center_size(
                pos2(cx + 210.0 * scale, cy - 144.0 * scale),
                vec2(58.0 * scale, 18.0 * scale),
            ),
            label: "RT",
            color: Color32::from_rgb(56, 189, 248),
            is_circle: false,
        });

        list.push(ButtonTarget {
            id: VirtualGamepadButton::RB,
            button_rect: Rect::from_center_size(
                pos2(cx + 210.0 * scale, cy - 110.0 * scale),
                vec2(58.0 * scale, 24.0 * scale),
            ),
            badge_rect: Rect::from_center_size(
                pos2(cx + 210.0 * scale, cy - 84.0 * scale),
                vec2(58.0 * scale, 18.0 * scale),
            ),
            label: "RB",
            color: Color32::from_rgb(14, 165, 233),
            is_circle: false,
        });

        // 2. Left Stick (LS - WASD)
        let ls_x = cx - 125.0 * scale;
        let ls_y = cy - 25.0 * scale;
        let stick_r = 22.0 * scale;

        list.push(ButtonTarget {
            id: VirtualGamepadButton::L3,
            button_rect: Rect::from_center_size(pos2(ls_x, ls_y), vec2(stick_r * 2.0, stick_r * 2.0)),
            badge_rect: Rect::from_center_size(
                pos2(ls_x, ls_y + stick_r + 14.0 * scale),
                vec2(54.0 * scale, 18.0 * scale),
            ),
            label: "L3",
            color: Color32::from_rgb(6, 182, 212),
            is_circle: true,
        });

        list.push(ButtonTarget {
            id: VirtualGamepadButton::LeftStickUp,
            button_rect: Rect::from_center_size(
                pos2(ls_x, ls_y - stick_r - 20.0 * scale),
                vec2(28.0 * scale, 20.0 * scale),
            ),
            badge_rect: Rect::from_center_size(
                pos2(ls_x, ls_y - stick_r - 42.0 * scale),
                vec2(44.0 * scale, 18.0 * scale),
            ),
            label: "▲",
            color: Color32::from_rgb(34, 211, 238),
            is_circle: false,
        });

        list.push(ButtonTarget {
            id: VirtualGamepadButton::LeftStickDown,
            button_rect: Rect::from_center_size(
                pos2(ls_x, ls_y + stick_r + 34.0 * scale),
                vec2(28.0 * scale, 20.0 * scale),
            ),
            badge_rect: Rect::from_center_size(
                pos2(ls_x, ls_y + stick_r + 56.0 * scale),
                vec2(44.0 * scale, 18.0 * scale),
            ),
            label: "▼",
            color: Color32::from_rgb(34, 211, 238),
            is_circle: false,
        });

        list.push(ButtonTarget {
            id: VirtualGamepadButton::LeftStickLeft,
            button_rect: Rect::from_center_size(
                pos2(ls_x - stick_r - 20.0 * scale, ls_y),
                vec2(28.0 * scale, 20.0 * scale),
            ),
            badge_rect: Rect::from_center_size(
                pos2(ls_x - stick_r - 54.0 * scale, ls_y),
                vec2(44.0 * scale, 18.0 * scale),
            ),
            label: "◀",
            color: Color32::from_rgb(34, 211, 238),
            is_circle: false,
        });

        list.push(ButtonTarget {
            id: VirtualGamepadButton::LeftStickRight,
            button_rect: Rect::from_center_size(
                pos2(ls_x + stick_r + 20.0 * scale, ls_y),
                vec2(28.0 * scale, 20.0 * scale),
            ),
            badge_rect: Rect::from_center_size(
                pos2(ls_x + stick_r + 54.0 * scale, ls_y),
                vec2(44.0 * scale, 18.0 * scale),
            ),
            label: "▶",
            color: Color32::from_rgb(34, 211, 238),
            is_circle: false,
        });

        // 3. D-Pad
        let dp_x = cx - 125.0 * scale;
        let dp_y = cy + 120.0 * scale;
        let dp_s = 24.0 * scale;

        list.push(ButtonTarget {
            id: VirtualGamepadButton::DpadUp,
            button_rect: Rect::from_center_size(
                pos2(dp_x, dp_y - dp_s),
                vec2(dp_s, dp_s),
            ),
            badge_rect: Rect::from_center_size(
                pos2(dp_x, dp_y - dp_s - 18.0 * scale),
                vec2(48.0 * scale, 16.0 * scale),
            ),
            label: "▲",
            color: Color32::from_rgb(168, 85, 247),
            is_circle: false,
        });

        list.push(ButtonTarget {
            id: VirtualGamepadButton::DpadDown,
            button_rect: Rect::from_center_size(
                pos2(dp_x, dp_y + dp_s),
                vec2(dp_s, dp_s),
            ),
            badge_rect: Rect::from_center_size(
                pos2(dp_x, dp_y + dp_s + 18.0 * scale),
                vec2(48.0 * scale, 16.0 * scale),
            ),
            label: "▼",
            color: Color32::from_rgb(168, 85, 247),
            is_circle: false,
        });

        list.push(ButtonTarget {
            id: VirtualGamepadButton::DpadLeft,
            button_rect: Rect::from_center_size(
                pos2(dp_x - dp_s, dp_y),
                vec2(dp_s, dp_s),
            ),
            badge_rect: Rect::from_center_size(
                pos2(dp_x - dp_s - 34.0 * scale, dp_y),
                vec2(48.0 * scale, 16.0 * scale),
            ),
            label: "◀",
            color: Color32::from_rgb(168, 85, 247),
            is_circle: false,
        });

        list.push(ButtonTarget {
            id: VirtualGamepadButton::DpadRight,
            button_rect: Rect::from_center_size(
                pos2(dp_x + dp_s, dp_y),
                vec2(dp_s, dp_s),
            ),
            badge_rect: Rect::from_center_size(
                pos2(dp_x + dp_s + 34.0 * scale, dp_y),
                vec2(48.0 * scale, 16.0 * scale),
            ),
            label: "▶",
            color: Color32::from_rgb(168, 85, 247),
            is_circle: false,
        });

        // 4. Face Buttons (Y, X, B, A)
        let fb_x = cx + 135.0 * scale;
        let fb_y = cy - 25.0 * scale;
        let fb_r = 14.0 * scale;
        let dist = 42.0 * scale;

        list.push(ButtonTarget {
            id: VirtualGamepadButton::Y,
            button_rect: Rect::from_center_size(pos2(fb_x, fb_y - dist), vec2(fb_r * 2.0, fb_r * 2.0)),
            badge_rect: Rect::from_center_size(
                pos2(fb_x, fb_y - dist - 22.0 * scale),
                vec2(48.0 * scale, 16.0 * scale),
            ),
            label: "Y",
            color: Color32::from_rgb(234, 179, 8),
            is_circle: true,
        });

        list.push(ButtonTarget {
            id: VirtualGamepadButton::A,
            button_rect: Rect::from_center_size(pos2(fb_x, fb_y + dist), vec2(fb_r * 2.0, fb_r * 2.0)),
            badge_rect: Rect::from_center_size(
                pos2(fb_x, fb_y + dist + 22.0 * scale),
                vec2(48.0 * scale, 16.0 * scale),
            ),
            label: "A",
            color: Color32::from_rgb(34, 197, 94),
            is_circle: true,
        });

        list.push(ButtonTarget {
            id: VirtualGamepadButton::X,
            button_rect: Rect::from_center_size(pos2(fb_x - dist, fb_y), vec2(fb_r * 2.0, fb_r * 2.0)),
            badge_rect: Rect::from_center_size(
                pos2(fb_x - dist - 34.0 * scale, fb_y),
                vec2(48.0 * scale, 16.0 * scale),
            ),
            label: "X",
            color: Color32::from_rgb(59, 130, 246),
            is_circle: true,
        });

        list.push(ButtonTarget {
            id: VirtualGamepadButton::B,
            button_rect: Rect::from_center_size(pos2(fb_x + dist, fb_y), vec2(fb_r * 2.0, fb_r * 2.0)),
            badge_rect: Rect::from_center_size(
                pos2(fb_x + dist + 34.0 * scale, fb_y),
                vec2(48.0 * scale, 16.0 * scale),
            ),
            label: "B",
            color: Color32::from_rgb(239, 68, 68),
            is_circle: true,
        });

        // 5. Right Stick (RS - Câmera)
        let rs_x = cx + 65.0 * scale;
        let rs_y = cy + 105.0 * scale;

        list.push(ButtonTarget {
            id: VirtualGamepadButton::R3,
            button_rect: Rect::from_center_size(pos2(rs_x, rs_y), vec2(stick_r * 2.0, stick_r * 2.0)),
            badge_rect: Rect::from_center_size(
                pos2(rs_x, rs_y + stick_r + 14.0 * scale),
                vec2(54.0 * scale, 18.0 * scale),
            ),
            label: "R3",
            color: Color32::from_rgb(6, 182, 212),
            is_circle: true,
        });

        list.push(ButtonTarget {
            id: VirtualGamepadButton::RightStickUp,
            button_rect: Rect::from_center_size(
                pos2(rs_x, rs_y - stick_r - 20.0 * scale),
                vec2(28.0 * scale, 20.0 * scale),
            ),
            badge_rect: Rect::from_center_size(
                pos2(rs_x, rs_y - stick_r - 42.0 * scale),
                vec2(44.0 * scale, 18.0 * scale),
            ),
            label: "▲",
            color: Color32::from_rgb(34, 211, 238),
            is_circle: false,
        });

        list.push(ButtonTarget {
            id: VirtualGamepadButton::RightStickDown,
            button_rect: Rect::from_center_size(
                pos2(rs_x, rs_y + stick_r + 34.0 * scale),
                vec2(28.0 * scale, 20.0 * scale),
            ),
            badge_rect: Rect::from_center_size(
                pos2(rs_x, rs_y + stick_r + 56.0 * scale),
                vec2(44.0 * scale, 18.0 * scale),
            ),
            label: "▼",
            color: Color32::from_rgb(34, 211, 238),
            is_circle: false,
        });

        list.push(ButtonTarget {
            id: VirtualGamepadButton::RightStickLeft,
            button_rect: Rect::from_center_size(
                pos2(rs_x - stick_r - 20.0 * scale, rs_y),
                vec2(28.0 * scale, 20.0 * scale),
            ),
            badge_rect: Rect::from_center_size(
                pos2(rs_x - stick_r - 54.0 * scale, rs_y),
                vec2(44.0 * scale, 18.0 * scale),
            ),
            label: "◀",
            color: Color32::from_rgb(34, 211, 238),
            is_circle: false,
        });

        list.push(ButtonTarget {
            id: VirtualGamepadButton::RightStickRight,
            button_rect: Rect::from_center_size(
                pos2(rs_x + stick_r + 20.0 * scale, rs_y),
                vec2(28.0 * scale, 20.0 * scale),
            ),
            badge_rect: Rect::from_center_size(
                pos2(rs_x + stick_r + 54.0 * scale, rs_y),
                vec2(44.0 * scale, 18.0 * scale),
            ),
            label: "▶",
            color: Color32::from_rgb(34, 211, 238),
            is_circle: false,
        });

        // 6. Central Buttons (Back, Guide, Start)
        list.push(ButtonTarget {
            id: VirtualGamepadButton::Back,
            button_rect: Rect::from_center_size(
                pos2(cx - 50.0 * scale, cy - 25.0 * scale),
                vec2(24.0 * scale, 18.0 * scale),
            ),
            badge_rect: Rect::from_center_size(
                pos2(cx - 50.0 * scale, cy - 44.0 * scale),
                vec2(42.0 * scale, 16.0 * scale),
            ),
            label: "◀◀",
            color: Color32::from_rgb(148, 163, 184),
            is_circle: false,
        });

        list.push(ButtonTarget {
            id: VirtualGamepadButton::Guide,
            button_rect: Rect::from_center_size(
                pos2(cx, cy - 45.0 * scale),
                vec2(32.0 * scale, 32.0 * scale),
            ),
            badge_rect: Rect::from_center_size(
                pos2(cx, cy - 72.0 * scale),
                vec2(48.0 * scale, 16.0 * scale),
            ),
            label: "X",
            color: Color32::from_rgb(34, 197, 94),
            is_circle: true,
        });

        list.push(ButtonTarget {
            id: VirtualGamepadButton::Start,
            button_rect: Rect::from_center_size(
                pos2(cx + 50.0 * scale, cy - 25.0 * scale),
                vec2(24.0 * scale, 18.0 * scale),
            ),
            badge_rect: Rect::from_center_size(
                pos2(cx + 50.0 * scale, cy - 44.0 * scale),
                vec2(42.0 * scale, 16.0 * scale),
            ),
            label: "▶▶",
            color: Color32::from_rgb(148, 163, 184),
            is_circle: false,
        });

        list
    }
}
