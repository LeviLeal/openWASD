use crate::controller_diagram::ControllerDiagram;
use crate::gamepad_types::{get_all_gamepad_buttons, GamepadButtonInfo, VirtualGamepadButton};
use crate::key_codes::{egui_key_to_evdev, evdev_to_key_name};
use crate::keyboard_interceptor::{InputDeviceInfo, InterceptorEvent, KeyboardInterceptor};
use crate::mapping_manager::MappingManager;
use egui::{
    pos2, vec2, Align, Align2, Color32, CornerRadius, CursorIcon, FontId, Layout, Margin,
    PointerButton, Rect, ResizeDirection, RichText, Sense, Stroke, StrokeKind, Ui, ViewportCommand,
    Visuals, Window,
};
use std::collections::{HashMap, VecDeque};
use std::time::SystemTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewTab {
    Columns,
    Diagram,
}

pub struct WasdApp {
    mapping_manager: MappingManager,
    interceptor: KeyboardInterceptor,
    diagram: ControllerDiagram,

    keyboards: Vec<InputDeviceInfo>,
    selected_keyboard_idx: usize,
    exclusive_grab: bool,

    active_tab: ViewTab,
    listening_button: Option<VirtualGamepadButton>,

    status_message: String,
    emulation_active: bool,
    permission_error: Option<(String, String)>,

    // Modais
    modal_new_profile: bool,
    new_profile_input: String,
    modal_save_profile: bool,
    save_profile_input: String,
    modal_clear_all: bool,

    // Console de logs
    logs: VecDeque<(String, String)>,
}

impl WasdApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::configure_style(&cc.egui_ctx);

        let mapping_manager = MappingManager::new();
        let interceptor = KeyboardInterceptor::new();
        let diagram = ControllerDiagram::new();

        let keyboards = KeyboardInterceptor::detect_keyboards();
        let selected_keyboard_idx = 0;

        let mut app = Self {
            mapping_manager,
            interceptor,
            diagram,
            keyboards,
            selected_keyboard_idx,
            exclusive_grab: false,
            active_tab: ViewTab::Columns,
            listening_button: None,
            status_message: "Ready. Click on any button to assign a key.".to_string(),
            emulation_active: false,
            permission_error: None,
            modal_new_profile: false,
            new_profile_input: String::new(),
            modal_save_profile: false,
            save_profile_input: String::new(),
            modal_clear_all: false,
            logs: VecDeque::new(),
        };

        app.log("WASD Anywhere initialized. Default FPS profile active.");
        app
    }

    fn configure_style(ctx: &egui::Context) {
        let mut visuals = Visuals::dark();

        visuals.override_text_color = Some(Color32::from_rgb(241, 245, 249)); // Slate 100
        visuals.panel_fill = Color32::from_rgb(15, 23, 42); // Slate 900
        visuals.window_fill = Color32::from_rgb(30, 41, 59); // Slate 800
        visuals.extreme_bg_color = Color32::from_rgb(10, 15, 29);

        // Widgets inativos
        visuals.widgets.inactive.bg_fill = Color32::from_rgb(30, 41, 59);
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(51, 65, 85));
        visuals.widgets.inactive.corner_radius = CornerRadius::same(6);

        // Widgets com hover
        visuals.widgets.hovered.bg_fill = Color32::from_rgb(51, 65, 85);
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, Color32::from_rgb(56, 189, 248)); // Cyan
        visuals.widgets.hovered.corner_radius = CornerRadius::same(6);

        // Widgets ativos
        visuals.widgets.active.bg_fill = Color32::from_rgb(14, 165, 233);
        visuals.widgets.active.bg_stroke = Stroke::new(1.5, Color32::WHITE);
        visuals.widgets.active.corner_radius = CornerRadius::same(6);

        // Seleção
        visuals.selection.bg_fill = Color32::from_rgb(2, 132, 199);
        visuals.selection.stroke = Stroke::new(1.0, Color32::WHITE);

        ctx.set_visuals(visuals);
    }

    fn log(&mut self, text: impl Into<String>) {
        let now = chrono_time_string();
        self.logs.push_back((now, text.into()));
        if self.logs.len() > 100 {
            self.logs.pop_front();
        }
    }

    fn current_mappings_for_interceptor(&self) -> HashMap<u16, Vec<VirtualGamepadButton>> {
        let mut map: HashMap<u16, Vec<VirtualGamepadButton>> = HashMap::new();
        for (btn, binding) in self.mapping_manager.get_all_bindings() {
            map.entry(binding.evdev_code).or_default().push(*btn);
        }
        map
    }

    fn toggle_emulation(&mut self) {
        if self.emulation_active {
            self.interceptor.stop();
            self.emulation_active = false;
            self.status_message = "Emulation stopped.".to_string();
            self.diagram.pressed_buttons.clear();
            self.log("Gamepad emulation stopped.");
        } else {
            let device_path = self
                .keyboards
                .get(self.selected_keyboard_idx)
                .map(|k| k.path.clone())
                .unwrap_or_else(|| "/dev/input/event3".to_string());

            let mappings = self.current_mappings_for_interceptor();
            self.interceptor
                .start(device_path, self.exclusive_grab, mappings);
        }
    }

    fn process_interceptor_events(&mut self) {
        let rx = self.interceptor.event_receiver().clone();
        while let Ok(event) = rx.try_recv() {
            match event {
                InterceptorEvent::Started => {
                    self.emulation_active = true;
                    self.permission_error = None;
                    self.status_message =
                        "Emulation active in background! Enjoy your game.".to_string();
                    self.log("Virtual Xbox 360 controller connected.");
                }
                InterceptorEvent::Stopped => {
                    self.emulation_active = false;
                    self.diagram.pressed_buttons.clear();
                }
                InterceptorEvent::PermissionError { path, message } => {
                    self.emulation_active = false;
                    self.permission_error = Some((path.clone(), message.clone()));
                    self.status_message = format!("Permission error on '{}'", path);
                    self.log(format!("Permission Error on {}: {}", path, message));
                }
                InterceptorEvent::LiveKeyEvent { evdev_code, pressed } => {
                    // Se o usuário está esperando para atribuir uma tecla
                    if let Some(listening_btn) = self.listening_button {
                        if pressed {
                            let key_name = evdev_to_key_name(evdev_code);
                            self.mapping_manager.set_binding(
                                listening_btn,
                                evdev_code,
                                key_name,
                            );
                            self.listening_button = None;
                            self.diagram.listening_button = None;
                            self.interceptor.set_paused(false);
                            self.interceptor
                                .update_mappings(self.current_mappings_for_interceptor());
                            self.log(format!(
                                "Bound: [{:?}] -> {} (code {})",
                                listening_btn, key_name, evdev_code
                            ));
                        }
                    }

                    // Atualizar diagrama visual com feedback dos botões em tempo real
                    let btns = self.mapping_manager.find_buttons_for_key(evdev_code);
                    for b in &btns {
                        if pressed {
                            self.diagram.pressed_buttons.insert(*b);
                        } else {
                            self.diagram.pressed_buttons.remove(b);
                        }
                    }

                    if pressed && !btns.is_empty() {
                        let btn_names: Vec<String> =
                            btns.iter().map(|b| format!("{:?}", b)).collect();
                        self.log(format!(
                            "Live key: {} (evdev {}) -> [{}]",
                            evdev_to_key_name(evdev_code),
                            evdev_code,
                            btn_names.join(", ")
                        ));
                    }
                }
                InterceptorEvent::Log(msg) => {
                    self.log(msg);
                }
            }
        }
    }

    fn check_window_key_presses(&mut self, ctx: &egui::Context) {
        if let Some(listening_btn) = self.listening_button {
            if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                self.listening_button = None;
                self.diagram.listening_button = None;
                self.interceptor.set_paused(false);
                self.log("Key assignment cancelled.");
                return;
            }

            // Checar se alguma tecla foi pressionada na janela do egui
            let mut captured_code = None;
            ctx.input(|i| {
                for event in &i.events {
                    if let egui::Event::Key {
                        key,
                        pressed: true,
                        repeat: false,
                        ..
                    } = event
                    {
                        if *key != egui::Key::Escape {
                            captured_code = egui_key_to_evdev(*key);
                        }
                    }
                }
            });

            if let Some(code) = captured_code {
                let key_name = evdev_to_key_name(code);
                self.mapping_manager
                    .set_binding(listening_btn, code, key_name);
                self.listening_button = None;
                self.diagram.listening_button = None;
                self.interceptor.set_paused(false);
                self.interceptor
                    .update_mappings(self.current_mappings_for_interceptor());
                self.log(format!(
                    "Bound via GUI: [{:?}] -> {} (code {})",
                    listening_btn, key_name, code
                ));
            }
        }
    }
}

impl eframe::App for WasdApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        self.process_interceptor_events();
        self.check_window_key_presses(&ctx);

        // Se estiver escutando tecla ou emulando, solicita redesenho contínuo
        if self.listening_button.is_some() || self.emulation_active {
            ctx.request_repaint();
        }

        // ==========================================
        // 0. CSD Custom Titlebar (Título no meio, Fechar, Maximizar, Minimizar)
        // ==========================================
        self.render_csd_titlebar(ui, &ctx);

        // Moldura externa da janela para bordas nítidas (quando não maximizado)
        let is_maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
        if !is_maximized {
            ui.painter().rect_stroke(
                ui.max_rect(),
                CornerRadius::ZERO,
                Stroke::new(1.0, Color32::from_rgb(37, 51, 74)),
                StrokeKind::Inside,
            );
        }

        // Conteúdo com espaçamento e padding
        egui::Frame::new()
            .inner_margin(Margin::symmetric(14, 10))
            .show(ui, |ui| {
                // ==========================================
                // 1. Top Header (Profiles & Emulation Switch)
                // ==========================================
                self.render_top_header(ui);

                ui.add_space(10.0);

                // ==========================================
                // 2. Banner de Erro de Permissão (se houver)
                // ==========================================
                if let Some((path, _)) = &self.permission_error {
                    let path_clone = path.clone();
                    self.render_permission_banner(ui, &path_clone);
                    ui.add_space(10.0);
                }

                // ==========================================
                // 3. Tab Bar (Columns vs Diagram View)
                // ==========================================
                self.render_tab_bar(ui);

                ui.add_space(8.0);

                // ==========================================
                // 4. Center Stage
                // ==========================================
                match self.active_tab {
                    ViewTab::Columns => {
                        self.render_columns_view(ui);
                    }
                    ViewTab::Diagram => {
                        self.render_diagram_view(ui);
                    }
                }

                ui.add_space(10.0);

                // ==========================================
                // 5. Bottom Panel (Device Settings & Console)
                // ==========================================
                self.render_bottom_panel(ui);
            });

        // ==========================================
        // Modais / Dialogs
        // ==========================================
        self.render_modals(&ctx);

        // ==========================================
        // Redimensionamento de bordas e cantos da janela (CSD)
        // ==========================================
        self.handle_window_edge_resize(&ctx, ui.max_rect());
    }
}

impl WasdApp {
    fn render_csd_titlebar(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        let titlebar_height = 38.0;
        let available_width = ui.available_width();
        let (titlebar_rect, titlebar_response) = ui.allocate_exact_size(
            vec2(available_width, titlebar_height),
            Sense::click_and_drag(),
        );

        let is_maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));

        // Background do CSD
        let painter = ui.painter_at(titlebar_rect);
        painter.rect_filled(titlebar_rect, 0.0, Color32::from_rgb(11, 17, 33));
        painter.line_segment(
            [titlebar_rect.left_bottom(), titlebar_rect.right_bottom()],
            Stroke::new(1.0, Color32::from_rgb(30, 41, 59)),
        );

        // 1. TÍTULO CENTRALIZADO NO MEIO DA JANELA
        let center_pos = titlebar_rect.center();
        painter.text(
            center_pos,
            Align2::CENTER_CENTER,
            "WASD Anywhere — Virtual Gamepad Mapper",
            FontId::proportional(13.0),
            Color32::from_rgb(226, 232, 240),
        );

        // 2. LADO ESQUERDO: Ícone e Status
        let mut left_ui = ui.new_child(
            egui::UiBuilder::default()
                .max_rect(titlebar_rect)
                .layout(Layout::left_to_right(Align::Center)),
        );
        left_ui.add_space(12.0);
        left_ui.label(RichText::new("🎮").size(15.0));
        left_ui.add_space(4.0);
        left_ui.label(
            RichText::new("WASD")
                .strong()
                .size(11.5)
                .color(Color32::from_rgb(56, 189, 248)),
        );
        left_ui.add_space(6.0);
        if self.emulation_active {
            left_ui.label(
                RichText::new("● ACTIVE")
                    .size(9.5)
                    .color(Color32::from_rgb(34, 197, 94))
                    .strong(),
            );
        } else {
            left_ui.label(
                RichText::new("○ IDLE")
                    .size(9.5)
                    .color(Color32::from_rgb(148, 163, 184)),
            );
        }

        // 3. LADO DIREITO: Botões de Controle (Fechar, Maximizar, Minimizar)
        let btn_height = if is_maximized { titlebar_height } else { 26.0 };
        let close_width = if is_maximized { 46.0 } else { 40.0 };
        let other_btn_width = if is_maximized { 44.0 } else { 36.0 };
        let btn_radius = if is_maximized {
            CornerRadius::ZERO
        } else {
            CornerRadius::same(5)
        };

        let mut right_ui = ui.new_child(
            egui::UiBuilder::default()
                .max_rect(titlebar_rect)
                .layout(Layout::right_to_left(if is_maximized {
                    Align::Min
                } else {
                    Align::Center
                })),
        );

        if !is_maximized {
            right_ui.add_space(6.0);
        }

        // 1. Botão Fechar (Close) - Vector "X"
        let (close_rect, close_resp) =
            right_ui.allocate_exact_size(vec2(close_width, btn_height), Sense::click());

        let close_color = if close_resp.hovered() {
            right_ui.painter().rect_filled(
                close_rect,
                btn_radius,
                Color32::from_rgb(239, 68, 68),
            );
            Color32::WHITE
        } else {
            Color32::from_rgb(203, 213, 225)
        };

        let c = close_rect.center();
        let s = 4.5;
        right_ui.painter().line_segment(
            [pos2(c.x - s, c.y - s), pos2(c.x + s, c.y + s)],
            Stroke::new(1.6, close_color),
        );
        right_ui.painter().line_segment(
            [pos2(c.x + s, c.y - s), pos2(c.x - s, c.y + s)],
            Stroke::new(1.6, close_color),
        );

        if close_resp.clicked() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        if !is_maximized {
            right_ui.add_space(2.0);
        }

        // 2. Botão Maximizar / Restaurar - Vector Box
        let (max_rect, max_resp) =
            right_ui.allocate_exact_size(vec2(other_btn_width, btn_height), Sense::click());

        let max_color = if max_resp.hovered() {
            right_ui.painter().rect_filled(
                max_rect,
                btn_radius,
                Color32::from_rgb(51, 65, 85),
            );
            Color32::WHITE
        } else {
            Color32::from_rgb(203, 213, 225)
        };

        let c = max_rect.center();
        if is_maximized {
            // Ícone restaurar: dois quadrados sobrepostos
            let back_rect = Rect::from_center_size(pos2(c.x + 2.0, c.y - 2.0), vec2(7.5, 7.5));
            right_ui.painter().rect_stroke(
                back_rect,
                CornerRadius::ZERO,
                Stroke::new(1.3, max_color),
                StrokeKind::Inside,
            );
            let front_rect = Rect::from_center_size(pos2(c.x - 2.0, c.y + 2.0), vec2(7.5, 7.5));
            right_ui.painter().rect_filled(
                front_rect,
                CornerRadius::ZERO,
                Color32::from_rgb(11, 17, 33),
            );
            right_ui.painter().rect_stroke(
                front_rect,
                CornerRadius::ZERO,
                Stroke::new(1.3, max_color),
                StrokeKind::Inside,
            );
        } else {
            // Ícone maximizar: um quadrado único limpo
            let box_rect = Rect::from_center_size(c, vec2(10.0, 10.0));
            right_ui.painter().rect_stroke(
                box_rect,
                CornerRadius::ZERO,
                Stroke::new(1.4, max_color),
                StrokeKind::Inside,
            );
        }

        if max_resp.clicked() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(!is_maximized));
        }

        if !is_maximized {
            right_ui.add_space(2.0);
        }

        // 3. Botão Minimizar - Vector Traço Horizontal
        let (min_rect, min_resp) =
            right_ui.allocate_exact_size(vec2(other_btn_width, btn_height), Sense::click());

        let min_color = if min_resp.hovered() {
            right_ui.painter().rect_filled(
                min_rect,
                btn_radius,
                Color32::from_rgb(51, 65, 85),
            );
            Color32::WHITE
        } else {
            Color32::from_rgb(203, 213, 225)
        };

        let c = min_rect.center();
        let w = 5.0;
        right_ui.painter().line_segment(
            [pos2(c.x - w, c.y), pos2(c.x + w, c.y)],
            Stroke::new(1.6, min_color),
        );

        if min_resp.clicked() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
        }

        // Arrastar e duplo-clique no titlebar (exceto na região dos botões e da borda superior)
        let buttons_width = close_width + other_btn_width * 2.0 + (if is_maximized { 0.0 } else { 10.0 });
        let is_over_buttons = ctx
            .input(|i| i.pointer.hover_pos())
            .map(|pos| pos.x >= titlebar_rect.right() - buttons_width)
            .unwrap_or(false);

        let is_on_resize_border = ctx
            .input(|i| i.pointer.hover_pos())
            .map(|pos| pos.y <= titlebar_rect.top() + 7.0 || pos.x <= titlebar_rect.left() + 7.0)
            .unwrap_or(false);

        if !is_over_buttons && !is_on_resize_border {
            if titlebar_response.drag_started() {
                ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
            }
            if titlebar_response.double_clicked() {
                ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(!is_maximized));
            }
        }
    }

    fn handle_window_edge_resize(&self, ctx: &egui::Context, rect: Rect) {
        let is_maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
        if is_maximized {
            return;
        }

        let Some(pos) = ctx.input(|i| i.pointer.hover_pos()) else {
            return;
        };

        let border_thickness = 8.0;
        let corner_size = 18.0;

        let on_left = pos.x <= rect.left() + border_thickness && pos.x >= rect.left() - 4.0;
        let on_right = pos.x >= rect.right() - border_thickness && pos.x <= rect.right() + 4.0;
        let on_top = pos.y <= rect.top() + border_thickness && pos.y >= rect.top() - 4.0;
        let on_bottom = pos.y >= rect.bottom() - border_thickness && pos.y <= rect.bottom() + 4.0;

        let dir_and_cursor = if on_top && on_left {
            Some((ResizeDirection::NorthWest, CursorIcon::ResizeNorthWest))
        } else if on_top && on_right {
            Some((ResizeDirection::NorthEast, CursorIcon::ResizeNorthEast))
        } else if on_bottom && on_left {
            Some((ResizeDirection::SouthWest, CursorIcon::ResizeSouthWest))
        } else if on_bottom && on_right {
            Some((ResizeDirection::SouthEast, CursorIcon::ResizeSouthEast))
        } else if on_top && pos.x > rect.left() + corner_size && pos.x < rect.right() - corner_size {
            Some((ResizeDirection::North, CursorIcon::ResizeNorth))
        } else if on_bottom && pos.x > rect.left() + corner_size && pos.x < rect.right() - corner_size {
            Some((ResizeDirection::South, CursorIcon::ResizeSouth))
        } else if on_left && pos.y > rect.top() + corner_size && pos.y < rect.bottom() - corner_size {
            Some((ResizeDirection::West, CursorIcon::ResizeWest))
        } else if on_right && pos.y > rect.top() + corner_size && pos.y < rect.bottom() - corner_size {
            Some((ResizeDirection::East, CursorIcon::ResizeEast))
        } else {
            None
        };

        if let Some((direction, cursor)) = dir_and_cursor {
            ctx.set_cursor_icon(cursor);
            if ctx.input(|i| i.pointer.button_pressed(PointerButton::Primary)) {
                ctx.send_viewport_cmd(ViewportCommand::BeginResize(direction));
            }
        }
    }

    fn render_top_header(&mut self, ui: &mut Ui) {
        egui::Frame::new()
            .fill(Color32::from_rgb(26, 36, 52))
            .stroke(Stroke::new(1.0, Color32::from_rgb(51, 65, 85)))
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::symmetric(14, 10))
            .show(ui, |ui| {
                ui.horizontal(|ui| {

                    // Perfil Selector
                    ui.label(RichText::new("Profile:").strong());

                    let current_name = self.mapping_manager.current_profile_name().to_string();
                    let profiles = self.mapping_manager.list_saved_profiles();

                    egui::ComboBox::from_id_salt("profile_combo")
                        .width(160.0)
                        .selected_text(&current_name)
                        .show_ui(ui, |ui| {
                            for p in profiles {
                                let is_selected = p == current_name;
                                if ui.selectable_label(is_selected, &p).clicked() && !is_selected {
                                    if let Err(e) = self.mapping_manager.load_profile(&p) {
                                        self.log(format!("Failed to load profile '{}': {}", p, e));
                                    } else {
                                        self.interceptor.update_mappings(
                                            self.current_mappings_for_interceptor(),
                                        );
                                        self.log(format!("Loaded profile '{}'", p));
                                    }
                                }
                            }
                        });

                    if ui
                        .button(RichText::new("💾 Save").color(Color32::WHITE))
                        .clicked()
                    {
                        self.save_profile_input = current_name;
                        self.modal_save_profile = true;
                    }

                    if ui
                        .button(RichText::new("➕ New").color(Color32::WHITE))
                        .clicked()
                    {
                        self.new_profile_input = "My Custom Profile".to_string();
                        self.modal_new_profile = true;
                    }

                    if ui
                        .button(RichText::new("↺ Default").color(Color32::WHITE))
                        .clicked()
                    {
                        self.mapping_manager.apply_default_fps_profile();
                        self.interceptor
                            .update_mappings(self.current_mappings_for_interceptor());
                        self.log("Restored Default FPS profile.");
                    }

                    if ui
                        .button(RichText::new("🗑 Clear All").color(Color32::from_rgb(239, 68, 68)))
                        .clicked()
                    {
                        self.modal_clear_all = true;
                    }

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        // Badge de status
                        if self.emulation_active {
                            ui.label(
                                RichText::new("● Emulating: Xbox 360 pad")
                                    .color(Color32::from_rgb(34, 197, 94))
                                    .strong(),
                            );
                        } else if self.permission_error.is_some() {
                            ui.label(
                                RichText::new("✖ Permission Error")
                                    .color(Color32::from_rgb(239, 68, 68))
                                    .strong(),
                            );
                        } else {
                            ui.label(
                                RichText::new("○ Inactive")
                                    .color(Color32::from_rgb(148, 163, 184))
                                    .strong(),
                            );
                        }

                        ui.add_space(10.0);

                        // Botão principal de Ligar/Desligar
                        let (btn_text, btn_color) = if self.emulation_active {
                            ("⏹ Stop Emulation", Color32::from_rgb(239, 68, 68))
                        } else {
                            ("▶ Start Emulation", Color32::from_rgb(14, 165, 233))
                        };

                        let btn = egui::Button::new(
                            RichText::new(btn_text).strong().size(13.5).color(Color32::WHITE),
                        )
                        .fill(btn_color)
                        .min_size(vec2(140.0, 32.0))
                        .corner_radius(CornerRadius::same(6));

                        if ui.add(btn).clicked() {
                            self.toggle_emulation();
                        }
                    });
                });
            });
    }

    fn render_permission_banner(&mut self, ui: &mut Ui, path: &str) {
        egui::Frame::new()
            .fill(Color32::from_rgb(69, 26, 26))
            .stroke(Stroke::new(1.0, Color32::from_rgb(239, 68, 68)))
            .corner_radius(CornerRadius::same(8))
            .inner_margin(Margin::symmetric(14, 10))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("⚠ Permission Denied:")
                            .color(Color32::from_rgb(248, 113, 113))
                            .strong(),
                    );
                    ui.label(format!("Cannot read input device or /dev/uinput at '{}'.", path));

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui
                            .button(RichText::new("Run Fix Script (Terminal)").strong().color(Color32::WHITE))
                            .clicked()
                        {
                            let _ = std::process::Command::new("xdg-terminal-exec")
                                .arg("./setup_permissions.sh")
                                .spawn();
                        }

                        ui.label(
                            RichText::new("Tip: Run ./setup_permissions.sh in terminal")
                                .monospace()
                                .color(Color32::from_rgb(252, 211, 77)),
                        );
                    });
                });
            });
    }

    fn render_tab_bar(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let is_cols = self.active_tab == ViewTab::Columns;
            let is_diag = self.active_tab == ViewTab::Diagram;

            if ui
                .selectable_label(is_cols, RichText::new("📋 Key Bindings Matrix").strong())
                .clicked()
            {
                self.active_tab = ViewTab::Columns;
            }

            if ui
                .selectable_label(is_diag, RichText::new("🎮 Controller Diagram").strong())
                .clicked()
            {
                self.active_tab = ViewTab::Diagram;
            }

            ui.add_space(20.0);
            if let Some(listening_btn) = self.listening_button {
                ui.label(
                    RichText::new(format!(
                        "⚡ Listening for key for [{:?}]... (Press any key, or Esc to cancel)",
                        listening_btn
                    ))
                    .color(Color32::from_rgb(251, 191, 36))
                    .strong(),
                );
            }
        });
    }

    fn render_columns_view(&mut self, ui: &mut Ui) {
        let all_buttons = get_all_gamepad_buttons();

        let mut left_stick_btns = Vec::new();
        let mut action_btns = Vec::new();
        let mut right_stick_btns = Vec::new();
        let mut dpad_menu_btns = Vec::new();

        for b in all_buttons {
            if b.category.contains("Left Stick") {
                left_stick_btns.push(b);
            } else if b.category.contains("Action") || b.category.contains("Shoulders") {
                action_btns.push(b);
            } else if b.category.contains("Right Stick") {
                right_stick_btns.push(b);
            } else {
                dpad_menu_btns.push(b);
            }
        }

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .max_height(ui.available_height() - 110.0)
            .show(ui, |ui| {
                ui.columns(4, |cols| {
                    self.render_button_column(&mut cols[0], "Left Stick (LS)", &left_stick_btns);
                    self.render_button_column(&mut cols[1], "Buttons & Shoulders", &action_btns);
                    self.render_button_column(&mut cols[2], "Right Stick (RS)", &right_stick_btns);
                    self.render_button_column(&mut cols[3], "D-Pad & Menus", &dpad_menu_btns);
                });
            });
    }

    fn render_button_column(&mut self, ui: &mut Ui, title: &str, buttons: &[&GamepadButtonInfo]) {
        egui::Frame::new()
            .fill(Color32::from_rgb(20, 29, 44))
            .stroke(Stroke::new(1.0, Color32::from_rgb(37, 51, 74)))
            .corner_radius(CornerRadius::same(8))
            .inner_margin(Margin::same(10))
            .show(ui, |ui| {
                ui.label(
                    RichText::new(title)
                        .strong()
                        .size(13.5)
                        .color(Color32::from_rgb(56, 189, 248)),
                );
                ui.add_space(8.0);

                for b in buttons {
                    let is_listening = self.listening_button == Some(b.id);
                    let is_pressed = self.diagram.pressed_buttons.contains(&b.id);
                    let binding = self.mapping_manager.get_binding(b.id).cloned();

                    ui.horizontal(|ui| {
                        // Nome do botão com tooltip
                        let name_color = if is_pressed {
                            Color32::from_rgb(34, 197, 94)
                        } else {
                            Color32::from_rgb(226, 232, 240)
                        };

                        let _ = ui
                            .add_sized([75.0, 24.0], egui::Label::new(
                                RichText::new(b.name).color(name_color).strong(),
                            ))
                            .on_hover_text(b.description);

                        // Botão de remapeamento
                        let (btn_text, btn_fill, btn_text_color) = if is_listening {
                            (
                                "🟡 Press key...".to_string(),
                                Color32::from_rgb(180, 83, 9),
                                Color32::WHITE,
                            )
                        } else if let Some(bind) = binding {
                            (
                                format!("[ {} ]", bind.key_name),
                                Color32::from_rgb(30, 41, 59),
                                Color32::from_rgb(56, 189, 248),
                            )
                        } else {
                            (
                                "[ Unbound ]".to_string(),
                                Color32::from_rgb(26, 36, 52),
                                Color32::from_rgb(148, 163, 184),
                            )
                        };

                        let rebind_btn = egui::Button::new(
                            RichText::new(btn_text)
                                .monospace()
                                .size(11.5)
                                .color(btn_text_color),
                        )
                        .fill(btn_fill)
                        .min_size(vec2(ui.available_width() - 32.0, 24.0))
                        .corner_radius(CornerRadius::same(5));

                        if ui.add(rebind_btn).clicked() {
                            if is_listening {
                                self.listening_button = None;
                                self.diagram.listening_button = None;
                                self.interceptor.set_paused(false);
                            } else {
                                self.listening_button = Some(b.id);
                                self.diagram.listening_button = Some(b.id);
                                self.interceptor.set_paused(true);
                            }
                        }

                        // Botão unbind "×"
                        if ui
                            .add_sized([22.0, 22.0], egui::Button::new(RichText::new("×").color(Color32::from_rgb(239, 68, 68))))
                            .on_hover_text("Unbind key")
                            .clicked()
                        {
                            self.mapping_manager.clear_binding(b.id);
                            self.interceptor.update_mappings(self.current_mappings_for_interceptor());
                            self.log(format!("Unbound [{:?}]", b.id));
                        }
                    });

                    ui.add_space(4.0);
                }
            });
    }

    fn render_diagram_view(&mut self, ui: &mut Ui) {
        if let Some(clicked_btn) = self.diagram.show(ui, &mut self.mapping_manager) {
            if self.listening_button == Some(clicked_btn) {
                self.listening_button = None;
                self.diagram.listening_button = None;
                self.interceptor.set_paused(false);
            } else {
                self.listening_button = Some(clicked_btn);
                self.diagram.listening_button = Some(clicked_btn);
                self.interceptor.set_paused(true);
            }
        }
    }

    fn render_bottom_panel(&mut self, ui: &mut Ui) {
        egui::Frame::new()
            .fill(Color32::from_rgb(20, 29, 44))
            .stroke(Stroke::new(1.0, Color32::from_rgb(37, 51, 74)))
            .corner_radius(CornerRadius::same(8))
            .inner_margin(Margin::symmetric(12, 8))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Keyboard:").strong());

                    let current_kbd_label = self
                        .keyboards
                        .get(self.selected_keyboard_idx)
                        .map(|k| format!("{} ({})", k.name, k.path))
                        .unwrap_or_else(|| "No keyboard detected".to_string());

                    let mut newly_selected = None;
                    egui::ComboBox::from_id_salt("kbd_combo")
                        .width(260.0)
                        .selected_text(&current_kbd_label)
                        .show_ui(ui, |ui| {
                            for (idx, kbd) in self.keyboards.iter().enumerate() {
                                let label = format!("{} ({})", kbd.name, kbd.path);
                                if ui
                                    .selectable_label(idx == self.selected_keyboard_idx, &label)
                                    .clicked()
                                {
                                    newly_selected = Some((idx, label));
                                }
                            }
                        });

                    if let Some((idx, label)) = newly_selected {
                        self.selected_keyboard_idx = idx;
                        self.log(format!("Selected keyboard: {}", label));
                    }

                    if ui
                        .button(RichText::new("🔄 Refresh").color(Color32::WHITE))
                        .on_hover_text("Scan for connected keyboards")
                        .clicked()
                    {
                        self.keyboards = KeyboardInterceptor::detect_keyboards();
                        self.selected_keyboard_idx = 0;
                        self.log(format!(
                            "Detected {} keyboard devices.",
                            self.keyboards.len()
                        ));
                    }

                    ui.add_space(10.0);

                    ui.checkbox(
                        &mut self.exclusive_grab,
                        "Exclusive Mode (block typing in system)",
                    )
                    .on_hover_text(
                        "Grabs the keyboard so keystrokes won't type into other apps/games while emulating.",
                    );

                    ui.add_space(10.0);

                    // Console de Log rápido em linha única / botão para limpar
                    let latest_log = self
                        .logs
                        .back()
                        .map(|(t, m)| format!("[{}] {}", t, m))
                        .unwrap_or_else(|| self.status_message.clone());

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui
                            .button(RichText::new("Clear").size(10.5))
                            .on_hover_text("Clear logs")
                            .clicked()
                        {
                            self.logs.clear();
                        }

                        ui.add_sized(
                            [ui.available_width() - 10.0, 20.0],
                            egui::Label::new(
                                RichText::new(latest_log)
                                    .monospace()
                                    .size(11.0)
                                    .color(Color32::from_rgb(148, 163, 184)),
                            ),
                        );
                    });
                });
            });
    }

    fn render_modals(&mut self, ctx: &egui::Context) {
        // Modal: Save Profile
        if self.modal_save_profile {
            Window::new("Save Profile")
                .collapsible(false)
                .resizable(false)
                .anchor(Align2::CENTER_CENTER, vec2(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.label("Enter a name for this profile:");
                    ui.text_edit_singleline(&mut self.save_profile_input);
                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        if ui.button("Save").clicked() {
                            let name = self.save_profile_input.trim();
                            if !name.is_empty() {
                                if let Err(e) = self.mapping_manager.save_profile(name) {
                                    self.log(format!("Failed to save profile: {}", e));
                                } else {
                                    self.log(format!("Profile '{}' saved successfully.", name));
                                    self.modal_save_profile = false;
                                }
                            }
                        }

                        if ui.button("Cancel").clicked() {
                            self.modal_save_profile = false;
                        }
                    });
                });
        }

        // Modal: New Profile
        if self.modal_new_profile {
            Window::new("New Profile")
                .collapsible(false)
                .resizable(false)
                .anchor(Align2::CENTER_CENTER, vec2(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.label("Profile Name:");
                    ui.text_edit_singleline(&mut self.new_profile_input);
                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        if ui.button("Create & Clear Bindings").clicked() {
                            let name = self.new_profile_input.trim();
                            if !name.is_empty() {
                                self.mapping_manager.clear_all_bindings();
                                let _ = self.mapping_manager.save_profile(name);
                                self.interceptor
                                    .update_mappings(self.current_mappings_for_interceptor());
                                self.log(format!("Created new profile '{}'.", name));
                                self.modal_new_profile = false;
                            }
                        }

                        if ui.button("Cancel").clicked() {
                            self.modal_new_profile = false;
                        }
                    });
                });
        }

        // Modal: Confirm Clear All
        if self.modal_clear_all {
            Window::new("Clear All Bindings?")
                .collapsible(false)
                .resizable(false)
                .anchor(Align2::CENTER_CENTER, vec2(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.label("Are you sure you want to unbind all controller buttons?");
                    ui.add_space(12.0);

                    ui.horizontal(|ui| {
                        if ui
                            .button(RichText::new("Yes, Unbind All").color(Color32::from_rgb(239, 68, 68)))
                            .clicked()
                        {
                            self.mapping_manager.clear_all_bindings();
                            self.interceptor
                                .update_mappings(self.current_mappings_for_interceptor());
                            self.log("All bindings cleared.");
                            self.modal_clear_all = false;
                        }

                        if ui.button("Cancel").clicked() {
                            self.modal_clear_all = false;
                        }
                    });
                });
        }
    }
}

fn chrono_time_string() -> String {
    // Retorna timestamp hh:mm:ss simples
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let secs = now % 60;
    let mins = (now / 60) % 60;
    let hours = (now / 3600) % 24;

    format!("{:02}:{:02}:{:02}", hours, mins, secs)
}
