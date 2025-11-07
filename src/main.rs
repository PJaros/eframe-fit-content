use eframe::egui;
use eframe::egui::{Vec2, Visuals, vec2};

fn main() -> eframe::Result {
    let app = AppGUI::default();
    eframe::run_native(
        "eframe fit content",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(app))),
    )
}

enum Stage {
    PreRender,
    FirstRender,
    FirstResize,
    Initialized,
}

struct AppGUI {
    _render_stage: Stage,
    _initial_size: Option<Vec2>,
}

impl Default for AppGUI {
    fn default() -> Self {
        Self {
            _render_stage: Stage::PreRender,
            _initial_size: None,
        }
    }
}

impl AppGUI {
    fn pre_render(&mut self, ctx: &eframe::egui::Context) {
        egui::Window::new("pre_render")
            .title_bar(false)
            .fixed_pos((0.0, 0.0))
            .show(ctx, |ui| {
                self.render(ui);
            });
    }
    fn render(&mut self, ui: &mut egui::Ui) {
        ui.add(
            egui::Label::new(egui::RichText::new("Hello, world!").heading())
                .wrap_mode(egui::TextWrapMode::Extend),
        );
        ui.label(format!(
            "self._used_size: {}",
            self._initial_size
                .unwrap_or_else(|| vec2(123.012_344, 123.012_344))
        ));
        for n in 1..=10 {
            ui.label(format!("Line: {}", n));
        }
    }
}

impl eframe::App for AppGUI {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(Visuals::light());
        match self._render_stage {
            Stage::PreRender => {
                self.pre_render(ctx);
                self._initial_size = Some(ctx.used_size());
                self._render_stage = Stage::FirstRender;
            }
            Stage::FirstRender => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.render(ui);
                });
                self._render_stage = Stage::FirstResize;
            }
            Stage::FirstResize => {
                if let Some(size) = self._initial_size {
                    ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(size));
                    self._render_stage = Stage::Initialized;
                }
            }
            Stage::Initialized => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.render(ui);
                });
            }
        }
    }
}
