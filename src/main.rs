use eframe::egui;
use eframe::egui::{Vec2, Visuals, vec2};

fn main() -> eframe::Result {
    let app = AppGUI::new();
    eframe::run_native(
        "eframe fit content",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(app))),
    )
}

enum Stage {
    PreRender(isize),
    FirstRender(Vec2),
    FirstResize(Vec2),
    Initialized(Vec2),
}

struct AppGUI {
    _render_stage: Stage,
}

impl AppGUI {
    fn new() -> Self {
        Self {
            _render_stage: Stage::PreRender(2_isize),
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
            match self._render_stage {
                Stage::PreRender(_) => vec2(123.012_344, 123.012_344),
                Stage::FirstRender(size) => size,
                Stage::FirstResize(size) => size,
                Stage::Initialized(size) => size,
            }
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
            Stage::PreRender(mut pre_render_cycle) => {
                self.pre_render(ctx);
                pre_render_cycle -= 1;
                if pre_render_cycle > 0 {
                    self._render_stage = Stage::PreRender(pre_render_cycle)
                } else {
                    self._render_stage = Stage::FirstRender(ctx.used_size());
                }
            }
            Stage::FirstRender(size) => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.render(ui);
                });
                self._render_stage = Stage::FirstResize(size)
            }
            Stage::FirstResize(size) => {
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(size));
                self._render_stage = Stage::Initialized(size)
            }
            Stage::Initialized(_size) => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.render(ui);
                });
            }
        }
    }
}
