use eframe::egui;
use eframe::egui::{vec2, Vec2, Visuals};

fn main() -> eframe::Result {
    let app = AppGUI::default();
    eframe::run_native(
        "eframe fit content",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(app))),
    )
}

struct AppGUI {
    _do_pre_render: bool,
    _used_size: Option<Vec2>,
    _first_resize: bool,
}

impl Default for AppGUI {
    fn default() -> Self {
        Self {
            _do_pre_render: true,
            _used_size: None,
            _first_resize: true,
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
        ui.label(format!("self._used_size: {}", match self._used_size {
            None => vec2(123.0123456, 123.0123456),
            Some(i) => i,
        }));

        for n in 1..=10 {
            ui.label(format!("Line: {}", n));
        }
    }
}

impl eframe::App for AppGUI {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(Visuals::light());
        if self._do_pre_render {
            let window_size = ctx.used_size();
            self.pre_render(ctx);
            println!("window_size: {} x {}", window_size.x, window_size.y);
            if !(window_size.y < 0.0 || window_size.x < 0.0) {
                self._used_size = Some(window_size);
                self._do_pre_render = false;
            }
        } else {
            if self._first_resize {
                if let Some(size) = self._used_size {
                    ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(size));
                }
                self._first_resize = false;
            }
            egui::CentralPanel::default().show(ctx, |ui| {
                self.render(ui);
            });
        }
    }
}