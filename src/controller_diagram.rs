use crate::gamepad_types::VirtualGamepadButton;
use crate::mapping_manager::MappingManager;
use egui::{
    pos2, vec2, Align2, Color32, FontFamily, FontId, Rect, Sense, Stroke, StrokeKind, Ui,
};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagramAction {
    Remap(VirtualGamepadButton),
    Unbind(VirtualGamepadButton),
}

pub struct ControllerDiagram {
    pub pressed_buttons: HashSet<VirtualGamepadButton>,
    pub listening_button: Option<VirtualGamepadButton>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TargetKind {
    ActionCircle, // Y, B, A, X com círculo colorido clássico
    Text,         // Up, Down, Left, Right, Home, Back, Start, LT, LB, etc.
    StickClick,   // L3, R3
}

struct ButtonTarget {
    id: VirtualGamepadButton,
    rect: Rect,
    btn_name: &'static str,
    desc: &'static str,
    color: Color32,
    kind: TargetKind,
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
    ) -> Option<DiagramAction> {
        let available_size = ui.available_size();
        let target_w = available_size.x.max(880.0);
        let target_h = (available_size.y - 96.0).clamp(510.0, 680.0);

        let (rect, response) = ui.allocate_exact_size(
            vec2(target_w, target_h),
            Sense::click(),
        );

        let painter = ui.painter_at(rect);
        let center = rect.center();

        // Escala responsiva suave baseada em 940x520
        let scale = (rect.width() / 940.0).min(rect.height() / 520.0).clamp(0.85, 1.35);

        // Fundo do container geral
        painter.rect_filled(
            rect,
            12.0,
            Color32::from_rgb(13, 19, 33), // Slate 950/900
        );
        painter.rect_stroke(
            rect,
            12.0,
            Stroke::new(1.0, Color32::from_rgb(30, 41, 59)),
            StrokeKind::Inside,
        );

        // Centro do controle Xbox 360 no canvas
        let cx = center.x;
        let cy = center.y + 16.0 * scale;

        // Calcular alvos interativos
        let targets = self.calculate_targets(cx, cy, scale);

        let mouse_pos = response.hover_pos();
        let primary_clicked = response.clicked();
        let secondary_clicked = response.secondary_clicked();

        let mut clicked_action = None;
        let mut hovered_target: Option<&ButtonTarget> = None;

        for t in &targets {
            let is_inside = mouse_pos
                .map(|p| match t.kind {
                    TargetKind::StickClick => t.rect.center().distance(p) <= t.rect.width() * 0.5,
                    _ => t.rect.contains(p),
                })
                .unwrap_or(false);

            if is_inside {
                hovered_target = Some(t);
                if primary_clicked {
                    clicked_action = Some(DiagramAction::Remap(t.id));
                } else if secondary_clicked {
                    clicked_action = Some(DiagramAction::Unbind(t.id));
                }
            }
        }

        // =====================================================================
        // 1. BANNER SUPERIOR INFORMATIVO & DINÂMICO
        // =====================================================================
        let banner_h = 36.0 * scale;
        let banner_rect = Rect::from_min_size(rect.min, vec2(rect.width(), banner_h));

        painter.rect_filled(
            banner_rect,
            12.0,
            Color32::from_rgb(17, 24, 39),
        );
        painter.line_segment(
            [
                pos2(rect.left(), rect.top() + banner_h),
                pos2(rect.right(), rect.top() + banner_h),
            ],
            Stroke::new(1.0, Color32::from_rgb(31, 41, 55)),
        );

        // Logo / Título no canto esquerdo do banner
        painter.text(
            pos2(rect.left() + 18.0 * scale, rect.top() + banner_h * 0.5),
            Align2::LEFT_CENTER,
            "🎮 XBOX 360 CONTROLLER",
            FontId::new(12.0 * scale, FontFamily::Proportional),
            Color32::from_rgb(203, 213, 225),
        );

        // Status interativo dinâmico no centro / direita do banner
        let (info_text, info_color) = if let Some(listening_btn) = self.listening_button {
            (
                format!("⚡ Listening for key assignment on {:?}... Press any key (Esc to cancel)", listening_btn),
                Color32::from_rgb(251, 191, 36),
            )
        } else if let Some(h) = hovered_target {
            let key_str = mapping_manager
                .get_binding(h.id)
                .map(|b| format!("Mapped to [ {} ]", b.key_name))
                .unwrap_or_else(|| "Unbound".to_string());
            (
                format!("🎯 {} — {}  •  Left-click: Rebind  |  Right-click: Unbind", h.desc, key_str),
                Color32::from_rgb(56, 189, 248),
            )
        } else {
            (
                "💡 Left-click any button to rebind  •  Right-click to unbind  •  Esc to cancel".to_string(),
                Color32::from_rgb(148, 163, 184),
            )
        };

        painter.text(
            pos2(rect.right() - 18.0 * scale, rect.top() + banner_h * 0.5),
            Align2::RIGHT_CENTER,
            info_text,
            FontId::new(12.0 * scale, FontFamily::Proportional),
            info_color,
        );

        // =====================================================================
        // 2. SILHUETA VETORIAL DO CONTROLE XBOX 360 (CHASSIS)
        // =====================================================================
        // Ponte superior dos bumpers
        let bridge_rect = Rect::from_center_size(
            pos2(cx, cy - 145.0 * scale),
            vec2(360.0 * scale, 52.0 * scale),
        );
        painter.rect_filled(bridge_rect, 20.0 * scale, Color32::from_rgb(20, 28, 42));
        painter.rect_stroke(
            bridge_rect,
            20.0 * scale,
            Stroke::new(1.5, Color32::from_rgb(41, 56, 80)),
            StrokeKind::Inside,
        );

        // Grips / Asas ergonômicas inferiores
        let grip_w = 115.0 * scale;
        let grip_h = 220.0 * scale;
        let left_grip = Rect::from_center_size(
            pos2(cx - 225.0 * scale, cy + 90.0 * scale),
            vec2(grip_w, grip_h),
        );
        let right_grip = Rect::from_center_size(
            pos2(cx + 225.0 * scale, cy + 90.0 * scale),
            vec2(grip_w, grip_h),
        );
        painter.rect_filled(left_grip, 55.0 * scale, Color32::from_rgb(17, 24, 38));
        painter.rect_stroke(
            left_grip,
            55.0 * scale,
            Stroke::new(1.5, Color32::from_rgb(34, 48, 70)),
            StrokeKind::Inside,
        );
        painter.rect_filled(right_grip, 55.0 * scale, Color32::from_rgb(17, 24, 38));
        painter.rect_stroke(
            right_grip,
            55.0 * scale,
            Stroke::new(1.5, Color32::from_rgb(34, 48, 70)),
            StrokeKind::Inside,
        );

        // Corpo central
        let body_rect = Rect::from_center_size(
            pos2(cx, cy + 10.0 * scale),
            vec2(540.0 * scale, 330.0 * scale),
        );
        painter.rect_filled(body_rect, 55.0 * scale, Color32::from_rgb(23, 32, 48));
        painter.rect_stroke(
            body_rect,
            55.0 * scale,
            Stroke::new(2.0, Color32::from_rgb(41, 56, 80)),
            StrokeKind::Inside,
        );

        // =====================================================================
        // 3. BACKPLATES & TRILHAS VISUAIS DOS CLUSTERS (SEM PARÊNTESES)
        // =====================================================================
        let ls_cx = cx - 225.0 * scale;
        let ls_cy = cy - 35.0 * scale;
        let dp_cx = cx - 110.0 * scale;
        let dp_cy = cy + 115.0 * scale;
        let abxy_cx = cx + 225.0 * scale;
        let abxy_cy = cy - 35.0 * scale;
        let rs_cx = cx + 110.0 * scale;
        let rs_cy = cy + 115.0 * scale;

        // --- Cluster Left Stick ---
        painter.circle_filled(
            pos2(ls_cx, ls_cy),
            96.0 * scale,
            Color32::from_rgba_unmultiplied(12, 18, 30, 190),
        );
        painter.circle_stroke(
            pos2(ls_cx, ls_cy),
            96.0 * scale,
            Stroke::new(1.0, Color32::from_rgba_unmultiplied(56, 189, 248, 50)),
        );
        painter.text(
            pos2(ls_cx, ls_cy - 88.0 * scale),
            Align2::CENTER_CENTER,
            "LEFT STICK",
            FontId::new(11.0 * scale, FontFamily::Proportional),
            Color32::from_rgb(56, 189, 248),
        );
        // Trilhas direcionais em cruz
        painter.line_segment(
            [pos2(ls_cx - 72.0 * scale, ls_cy), pos2(ls_cx + 72.0 * scale, ls_cy)],
            Stroke::new(1.5, Color32::from_rgba_unmultiplied(56, 189, 248, 50)),
        );
        painter.line_segment(
            [pos2(ls_cx, ls_cy - 68.0 * scale), pos2(ls_cx, ls_cy + 68.0 * scale)],
            Stroke::new(1.5, Color32::from_rgba_unmultiplied(56, 189, 248, 50)),
        );

        // --- Cluster D-Pad ---
        painter.circle_filled(
            pos2(dp_cx, dp_cy),
            86.0 * scale,
            Color32::from_rgba_unmultiplied(12, 18, 30, 190),
        );
        painter.circle_stroke(
            pos2(dp_cx, dp_cy),
            86.0 * scale,
            Stroke::new(1.0, Color32::from_rgba_unmultiplied(168, 85, 247, 50)),
        );
        painter.text(
            pos2(dp_cx, dp_cy - 76.0 * scale),
            Align2::CENTER_CENTER,
            "D-PAD",
            FontId::new(10.5 * scale, FontFamily::Proportional),
            Color32::from_rgb(192, 132, 252),
        );
        // Cruz central decorativa do D-Pad
        let cross_w = 46.0 * scale;
        let cross_t = 16.0 * scale;
        painter.rect_filled(
            Rect::from_center_size(pos2(dp_cx, dp_cy), vec2(cross_w, cross_t)),
            3.0 * scale,
            Color32::from_rgb(26, 36, 52),
        );
        painter.rect_filled(
            Rect::from_center_size(pos2(dp_cx, dp_cy), vec2(cross_t, cross_w)),
            3.0 * scale,
            Color32::from_rgb(26, 36, 52),
        );

        // --- Cluster Actions (ABXY) ---
        painter.circle_filled(
            pos2(abxy_cx, abxy_cy),
            96.0 * scale,
            Color32::from_rgba_unmultiplied(12, 18, 30, 190),
        );
        painter.circle_stroke(
            pos2(abxy_cx, abxy_cy),
            96.0 * scale,
            Stroke::new(1.0, Color32::from_rgba_unmultiplied(234, 179, 8, 50)),
        );
        painter.text(
            pos2(abxy_cx, abxy_cy - 88.0 * scale),
            Align2::CENTER_CENTER,
            "ACTIONS",
            FontId::new(10.5 * scale, FontFamily::Proportional),
            Color32::from_rgb(250, 204, 21),
        );
        // Linhas de conexão em diamante ABXY
        let diamond_pts = [
            pos2(abxy_cx, abxy_cy - 68.0 * scale),
            pos2(abxy_cx + 74.0 * scale, abxy_cy),
            pos2(abxy_cx, abxy_cy + 68.0 * scale),
            pos2(abxy_cx - 74.0 * scale, abxy_cy),
            pos2(abxy_cx, abxy_cy - 68.0 * scale),
        ];
        for i in 0..4 {
            painter.line_segment(
                [diamond_pts[i], diamond_pts[i + 1]],
                Stroke::new(1.2, Color32::from_rgba_unmultiplied(234, 179, 8, 40)),
            );
        }

        // --- Cluster Right Stick ---
        painter.circle_filled(
            pos2(rs_cx, rs_cy),
            96.0 * scale,
            Color32::from_rgba_unmultiplied(12, 18, 30, 190),
        );
        painter.circle_stroke(
            pos2(rs_cx, rs_cy),
            96.0 * scale,
            Stroke::new(1.0, Color32::from_rgba_unmultiplied(56, 189, 248, 50)),
        );
        painter.text(
            pos2(rs_cx, rs_cy - 88.0 * scale),
            Align2::CENTER_CENTER,
            "RIGHT STICK",
            FontId::new(11.0 * scale, FontFamily::Proportional),
            Color32::from_rgb(56, 189, 248),
        );
        // Trilhas direcionais em cruz
        painter.line_segment(
            [pos2(rs_cx - 72.0 * scale, rs_cy), pos2(rs_cx + 72.0 * scale, rs_cy)],
            Stroke::new(1.5, Color32::from_rgba_unmultiplied(56, 189, 248, 50)),
        );
        painter.line_segment(
            [pos2(rs_cx, rs_cy - 68.0 * scale), pos2(rs_cx, rs_cy + 68.0 * scale)],
            Stroke::new(1.5, Color32::from_rgba_unmultiplied(56, 189, 248, 50)),
        );

        // Título de Triggers & Bumpers no topo central
        painter.text(
            pos2(cx, cy - 212.0 * scale),
            Align2::CENTER_CENTER,
            "TRIGGERS & BUMPERS",
            FontId::new(11.0 * scale, FontFamily::Proportional),
            Color32::from_rgb(148, 163, 184),
        );

        // =====================================================================
        // 4. DESENHO DOS 21 CHIPS INTERATIVOS UNIFICADOS
        // =====================================================================
        for t in &targets {
            let is_pressed = self.pressed_buttons.contains(&t.id);
            let is_listening = self.listening_button == Some(t.id);
            let is_bound = mapping_manager.has_binding(t.id);
            let bound_key = mapping_manager.get_binding(t.id).map(|b| b.key_name.as_str());

            let is_hovered = hovered_target.map(|h| h.id == t.id).unwrap_or(false);

            // Cores de estado
            let bg_color = if is_listening {
                Color32::from_rgb(180, 83, 9) // Amber pulsante
            } else if is_pressed {
                Color32::from_rgb(30, 58, 95) // Realce de tecla pressionada
            } else if is_hovered {
                Color32::from_rgb(33, 47, 69)
            } else {
                Color32::from_rgb(18, 25, 38)
            };

            let stroke_color = if is_listening {
                Color32::from_rgb(251, 191, 36)
            } else if is_pressed {
                Color32::WHITE
            } else if is_bound {
                t.color
            } else {
                Color32::from_rgb(51, 65, 85)
            };

            let stroke_width = if is_listening || is_pressed {
                2.2
            } else if is_hovered || is_bound {
                1.5
            } else {
                1.0
            };

            match t.kind {
                TargetKind::StickClick => {
                    // Botão circular de clique da alavanca analógica (L3 / R3)
                    let center_pos = t.rect.center();
                    let radius = t.rect.width() * 0.5;

                    // Anel externo de borracha com textura
                    painter.circle_filled(center_pos, radius, bg_color);
                    painter.circle_stroke(
                        center_pos,
                        radius,
                        Stroke::new(stroke_width, stroke_color),
                    );

                    // Círculo interno concêntrico
                    painter.circle_stroke(
                        center_pos,
                        radius * 0.72,
                        Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 35)),
                    );

                    // Rótulo L3 / R3
                    let label_color = if is_listening || is_pressed {
                        Color32::WHITE
                    } else {
                        t.color
                    };

                    painter.text(
                        pos2(center_pos.x, center_pos.y - 7.0 * scale),
                        Align2::CENTER_CENTER,
                        t.btn_name,
                        FontId::new(12.0 * scale, FontFamily::Proportional),
                        label_color,
                    );

                    // Tecla mapeada abaixo
                    let key_text = if is_listening {
                        "Press..."
                    } else if let Some(k) = bound_key {
                        k
                    } else {
                        "—"
                    };

                    let key_color = if is_listening || is_bound || is_pressed {
                        Color32::WHITE
                    } else {
                        Color32::from_rgb(100, 116, 139)
                    };

                    painter.text(
                        pos2(center_pos.x, center_pos.y + 7.5 * scale),
                        Align2::CENTER_CENTER,
                        key_text,
                        FontId::new(11.5 * scale, FontFamily::Proportional),
                        key_color,
                    );
                }

                _ => {
                    // Chip unificado padrão (Card arredondado)
                    let corner_radius = 7.0 * scale;
                    painter.rect_filled(t.rect, corner_radius, bg_color);
                    painter.rect_stroke(
                        t.rect,
                        corner_radius,
                        Stroke::new(stroke_width, stroke_color),
                        StrokeKind::Inside,
                    );

                    // Posição vertical central do conteúdo
                    let mid_y = t.rect.center().y;

                    // Posição do divisor vertical baseada no tamanho do nome do botão
                    let split_x = match t.kind {
                        TargetKind::ActionCircle => t.rect.left() + 28.0 * scale,
                        TargetKind::Text => {
                            if t.btn_name.len() >= 4 {
                                t.rect.left() + 40.0 * scale // Home, Back, Start, Left, Down
                            } else {
                                t.rect.left() + 32.0 * scale // Up, LT, LB, RT, RB
                            }
                        }
                        TargetKind::StickClick => t.rect.left(),
                    };

                    painter.line_segment(
                        [
                            pos2(split_x, t.rect.top() + 5.0 * scale),
                            pos2(split_x, t.rect.bottom() - 5.0 * scale),
                        ],
                        Stroke::new(1.0, Color32::from_rgb(38, 50, 70)),
                    );

                    // Rótulo à esquerda do chip
                    match t.kind {
                        TargetKind::ActionCircle => {
                            let circle_pos = pos2(t.rect.left() + 14.5 * scale, mid_y);
                            let circle_r = 10.0 * scale;

                            painter.circle_filled(circle_pos, circle_r, t.color);
                            painter.circle_stroke(
                                circle_pos,
                                circle_r,
                                Stroke::new(1.0, Color32::WHITE),
                            );

                            let letter_color = if t.btn_name == "Y" {
                                Color32::from_rgb(17, 24, 39) // Amarelo com texto escuro para contraste impecável
                            } else {
                                Color32::WHITE
                            };

                            painter.text(
                                circle_pos,
                                Align2::CENTER_CENTER,
                                t.btn_name,
                                FontId::new(12.5 * scale, FontFamily::Proportional),
                                letter_color,
                            );
                        }

                        TargetKind::Text => {
                            let left_center_x = (t.rect.left() + split_x) * 0.5;
                            painter.text(
                                pos2(left_center_x, mid_y),
                                Align2::CENTER_CENTER,
                                t.btn_name,
                                FontId::new(12.5 * scale, FontFamily::Proportional),
                                if is_listening || is_pressed {
                                    Color32::WHITE
                                } else {
                                    t.color
                                },
                            );
                        }

                        TargetKind::StickClick => {}
                    }

                    // Tecla mapeada à direita
                    let key_text = if is_listening {
                        "Press key..."
                    } else if let Some(k) = bound_key {
                        k
                    } else {
                        "—"
                    };

                    let key_color = if is_listening || is_pressed {
                        Color32::WHITE
                    } else if is_bound {
                        Color32::from_rgb(241, 245, 249)
                    } else {
                        Color32::from_rgb(100, 116, 139)
                    };

                    let right_center_x = (split_x + t.rect.right()) * 0.5;
                    let font_size = if key_text.len() > 7 {
                        11.0 * scale
                    } else {
                        12.5 * scale
                    };

                    painter.text(
                        pos2(right_center_x, mid_y),
                        Align2::CENTER_CENTER,
                        key_text,
                        FontId::new(font_size, FontFamily::Proportional),
                        key_color,
                    );
                }
            }
        }

        clicked_action
    }

    fn calculate_targets(&self, cx: f32, cy: f32, scale: f32) -> Vec<ButtonTarget> {
        let mut list = Vec::new();

        // =====================================================================
        // 1. OMBROS & GATILHOS (LT, LB, RT, RB)
        // =====================================================================
        // Gatilho Esquerdo (LT)
        list.push(ButtonTarget {
            id: VirtualGamepadButton::LT,
            rect: Rect::from_center_size(
                pos2(cx - 240.0 * scale, cy - 180.0 * scale),
                vec2(98.0 * scale, 34.0 * scale),
            ),
            btn_name: "LT",
            desc: "Left Trigger (LT)",
            color: Color32::from_rgb(56, 189, 248),
            kind: TargetKind::Text,
        });

        // Bumper Esquerdo (LB)
        list.push(ButtonTarget {
            id: VirtualGamepadButton::LB,
            rect: Rect::from_center_size(
                pos2(cx - 130.0 * scale, cy - 180.0 * scale),
                vec2(94.0 * scale, 34.0 * scale),
            ),
            btn_name: "LB",
            desc: "Left Bumper (LB)",
            color: Color32::from_rgb(14, 165, 233),
            kind: TargetKind::Text,
        });

        // Bumper Direito (RB)
        list.push(ButtonTarget {
            id: VirtualGamepadButton::RB,
            rect: Rect::from_center_size(
                pos2(cx + 130.0 * scale, cy - 180.0 * scale),
                vec2(94.0 * scale, 34.0 * scale),
            ),
            btn_name: "RB",
            desc: "Right Bumper (RB)",
            color: Color32::from_rgb(14, 165, 233),
            kind: TargetKind::Text,
        });

        // Gatilho Direito (RT)
        list.push(ButtonTarget {
            id: VirtualGamepadButton::RT,
            rect: Rect::from_center_size(
                pos2(cx + 240.0 * scale, cy - 180.0 * scale),
                vec2(98.0 * scale, 34.0 * scale),
            ),
            btn_name: "RT",
            desc: "Right Trigger (RT)",
            color: Color32::from_rgb(56, 189, 248),
            kind: TargetKind::Text,
        });

        // =====================================================================
        // 2. BOTÕES CENTRAIS (Back, Home, Start)
        // =====================================================================
        list.push(ButtonTarget {
            id: VirtualGamepadButton::Back,
            rect: Rect::from_center_size(
                pos2(cx - 95.0 * scale, cy - 85.0 * scale),
                vec2(84.0 * scale, 34.0 * scale),
            ),
            btn_name: "Back",
            desc: "Back Button",
            color: Color32::from_rgb(148, 163, 184),
            kind: TargetKind::Text,
        });

        list.push(ButtonTarget {
            id: VirtualGamepadButton::Guide,
            rect: Rect::from_center_size(
                pos2(cx, cy - 90.0 * scale),
                vec2(86.0 * scale, 36.0 * scale),
            ),
            btn_name: "Home",
            desc: "Home Button",
            color: Color32::from_rgb(34, 197, 94),
            kind: TargetKind::Text,
        });

        list.push(ButtonTarget {
            id: VirtualGamepadButton::Start,
            rect: Rect::from_center_size(
                pos2(cx + 95.0 * scale, cy - 85.0 * scale),
                vec2(84.0 * scale, 34.0 * scale),
            ),
            btn_name: "Start",
            desc: "Start Button",
            color: Color32::from_rgb(148, 163, 184),
            kind: TargetKind::Text,
        });

        // =====================================================================
        // 3. LEFT STICK (Up, Down, Left, Right, L3)
        // =====================================================================
        let ls_cx = cx - 225.0 * scale;
        let ls_cy = cy - 35.0 * scale;
        let dir_w = 78.0 * scale;
        let dir_h = 32.0 * scale;

        // Clique do analógico L3 no centro
        list.push(ButtonTarget {
            id: VirtualGamepadButton::L3,
            rect: Rect::from_center_size(
                pos2(ls_cx, ls_cy),
                vec2(56.0 * scale, 56.0 * scale),
            ),
            btn_name: "L3",
            desc: "Left Stick — L3",
            color: Color32::from_rgb(6, 182, 212),
            kind: TargetKind::StickClick,
        });

        list.push(ButtonTarget {
            id: VirtualGamepadButton::LeftStickUp,
            rect: Rect::from_center_size(
                pos2(ls_cx, ls_cy - 68.0 * scale),
                vec2(dir_w, dir_h),
            ),
            btn_name: "Up",
            desc: "Left Stick — Up",
            color: Color32::from_rgb(34, 211, 238),
            kind: TargetKind::Text,
        });

        list.push(ButtonTarget {
            id: VirtualGamepadButton::LeftStickDown,
            rect: Rect::from_center_size(
                pos2(ls_cx, ls_cy + 68.0 * scale),
                vec2(dir_w, dir_h),
            ),
            btn_name: "Down",
            desc: "Left Stick — Down",
            color: Color32::from_rgb(34, 211, 238),
            kind: TargetKind::Text,
        });

        list.push(ButtonTarget {
            id: VirtualGamepadButton::LeftStickLeft,
            rect: Rect::from_center_size(
                pos2(ls_cx - 76.0 * scale, ls_cy),
                vec2(dir_w, dir_h),
            ),
            btn_name: "Left",
            desc: "Left Stick — Left",
            color: Color32::from_rgb(34, 211, 238),
            kind: TargetKind::Text,
        });

        list.push(ButtonTarget {
            id: VirtualGamepadButton::LeftStickRight,
            rect: Rect::from_center_size(
                pos2(ls_cx + 76.0 * scale, ls_cy),
                vec2(dir_w, dir_h),
            ),
            btn_name: "Right",
            desc: "Left Stick — Right",
            color: Color32::from_rgb(34, 211, 238),
            kind: TargetKind::Text,
        });

        // =====================================================================
        // 4. D-PAD (Up, Down, Left, Right)
        // =====================================================================
        let dp_cx = cx - 110.0 * scale;
        let dp_cy = cy + 115.0 * scale;
        let dp_w = 74.0 * scale;
        let dp_h = 30.0 * scale;
        let dp_dist = 56.0 * scale;

        list.push(ButtonTarget {
            id: VirtualGamepadButton::DpadUp,
            rect: Rect::from_center_size(
                pos2(dp_cx, dp_cy - dp_dist),
                vec2(dp_w, dp_h),
            ),
            btn_name: "Up",
            desc: "D-Pad — Up",
            color: Color32::from_rgb(168, 85, 247),
            kind: TargetKind::Text,
        });

        list.push(ButtonTarget {
            id: VirtualGamepadButton::DpadDown,
            rect: Rect::from_center_size(
                pos2(dp_cx, dp_cy + dp_dist),
                vec2(dp_w, dp_h),
            ),
            btn_name: "Down",
            desc: "D-Pad — Down",
            color: Color32::from_rgb(168, 85, 247),
            kind: TargetKind::Text,
        });

        list.push(ButtonTarget {
            id: VirtualGamepadButton::DpadLeft,
            rect: Rect::from_center_size(
                pos2(dp_cx - 68.0 * scale, dp_cy),
                vec2(dp_w, dp_h),
            ),
            btn_name: "Left",
            desc: "D-Pad — Left",
            color: Color32::from_rgb(168, 85, 247),
            kind: TargetKind::Text,
        });

        list.push(ButtonTarget {
            id: VirtualGamepadButton::DpadRight,
            rect: Rect::from_center_size(
                pos2(dp_cx + 68.0 * scale, dp_cy),
                vec2(dp_w, dp_h),
            ),
            btn_name: "Right",
            desc: "D-Pad — Right",
            color: Color32::from_rgb(168, 85, 247),
            kind: TargetKind::Text,
        });

        // =====================================================================
        // 5. BOTÕES DE AÇÃO (Y, X, B, A)
        // =====================================================================
        let abxy_cx = cx + 225.0 * scale;
        let abxy_cy = cy - 35.0 * scale;
        let act_w = 70.0 * scale;
        let act_h = 32.0 * scale;

        list.push(ButtonTarget {
            id: VirtualGamepadButton::Y,
            rect: Rect::from_center_size(
                pos2(abxy_cx, abxy_cy - 68.0 * scale),
                vec2(act_w, act_h),
            ),
            btn_name: "Y",
            desc: "Button Y",
            color: Color32::from_rgb(234, 179, 8),
            kind: TargetKind::ActionCircle,
        });

        list.push(ButtonTarget {
            id: VirtualGamepadButton::A,
            rect: Rect::from_center_size(
                pos2(abxy_cx, abxy_cy + 68.0 * scale),
                vec2(act_w, act_h),
            ),
            btn_name: "A",
            desc: "Button A",
            color: Color32::from_rgb(34, 197, 94),
            kind: TargetKind::ActionCircle,
        });

        list.push(ButtonTarget {
            id: VirtualGamepadButton::X,
            rect: Rect::from_center_size(
                pos2(abxy_cx - 74.0 * scale, abxy_cy),
                vec2(act_w, act_h),
            ),
            btn_name: "X",
            desc: "Button X",
            color: Color32::from_rgb(59, 130, 246),
            kind: TargetKind::ActionCircle,
        });

        list.push(ButtonTarget {
            id: VirtualGamepadButton::B,
            rect: Rect::from_center_size(
                pos2(abxy_cx + 74.0 * scale, abxy_cy),
                vec2(act_w, act_h),
            ),
            btn_name: "B",
            desc: "Button B",
            color: Color32::from_rgb(239, 68, 68),
            kind: TargetKind::ActionCircle,
        });

        // =====================================================================
        // 6. RIGHT STICK (Up, Down, Left, Right, R3)
        // =====================================================================
        let rs_cx = cx + 110.0 * scale;
        let rs_cy = cy + 115.0 * scale;

        // Clique do analógico R3 no centro
        list.push(ButtonTarget {
            id: VirtualGamepadButton::R3,
            rect: Rect::from_center_size(
                pos2(rs_cx, rs_cy),
                vec2(56.0 * scale, 56.0 * scale),
            ),
            btn_name: "R3",
            desc: "Right Stick — R3",
            color: Color32::from_rgb(6, 182, 212),
            kind: TargetKind::StickClick,
        });

        list.push(ButtonTarget {
            id: VirtualGamepadButton::RightStickUp,
            rect: Rect::from_center_size(
                pos2(rs_cx, rs_cy - 68.0 * scale),
                vec2(dir_w, dir_h),
            ),
            btn_name: "Up",
            desc: "Right Stick — Up",
            color: Color32::from_rgb(34, 211, 238),
            kind: TargetKind::Text,
        });

        list.push(ButtonTarget {
            id: VirtualGamepadButton::RightStickDown,
            rect: Rect::from_center_size(
                pos2(rs_cx, rs_cy + 68.0 * scale),
                vec2(dir_w, dir_h),
            ),
            btn_name: "Down",
            desc: "Right Stick — Down",
            color: Color32::from_rgb(34, 211, 238),
            kind: TargetKind::Text,
        });

        list.push(ButtonTarget {
            id: VirtualGamepadButton::RightStickLeft,
            rect: Rect::from_center_size(
                pos2(rs_cx - 76.0 * scale, rs_cy),
                vec2(dir_w, dir_h),
            ),
            btn_name: "Left",
            desc: "Right Stick — Left",
            color: Color32::from_rgb(34, 211, 238),
            kind: TargetKind::Text,
        });

        list.push(ButtonTarget {
            id: VirtualGamepadButton::RightStickRight,
            rect: Rect::from_center_size(
                pos2(rs_cx + 76.0 * scale, rs_cy),
                vec2(dir_w, dir_h),
            ),
            btn_name: "Right",
            desc: "Right Stick — Right",
            color: Color32::from_rgb(34, 211, 238),
            kind: TargetKind::Text,
        });

        list
    }
}
