use eframe::egui;
use crate::game::{Difficulty, GuessingGame, Screen};

impl eframe::App for GuessingGame {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.5);

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.screen {
                Screen::Menu => self.draw_menu(ui),
                Screen::Playing => self.draw_playing(ui, ctx),
            }
        });
    }
}

impl GuessingGame {
    fn draw_menu(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(60.0);
            ui.heading("🎲 Gæt et tal!");
            ui.add_space(10.0);
            ui.label(egui::RichText::new("Vælg sværhedsgrad:").size(18.0));
            ui.add_space(25.0);

            for diff in [Difficulty::Easy, Difficulty::Normal, Difficulty::Hard] {
                let btn = egui::Button::new(egui::RichText::new(diff.label()).size(20.0))
                    .min_size(egui::vec2(220.0, 48.0));
                if ui.add(btn).clicked() {
                    self.start_new_game(diff);
                }
                ui.add_space(12.0);
            }
        });
    }

    fn draw_playing(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.vertical_centered(|ui| {
            ui.add_space(30.0);

            // Top bar to return to menu
            if ui.button("⬅ Tilbage til menu").clicked() {
                self.screen = Screen::Menu;
                return;
            }

            ui.add_space(10.0);
            ui.heading("🎲 Gæt et tal!");
            ui.add_space(5.0);
            ui.label(egui::RichText::new(format!("Forsøg: #{}", self.attempts)).size(16.0));
            ui.add_space(15.0);

            // 1. Input field
            let text_input_response = ui.add(
                egui::TextEdit::singleline(&mut self.guess_input)
                    .hint_text("Indtast dit gæt her...")
                    .font(egui::FontId::proportional(20.0)),
            );
            ui.add_space(15.0);

            // 2. Submit button
            let submit_clicked = ui
                .add(
                    egui::Button::new(egui::RichText::new("Gæt").size(22.0))
                        .min_size(egui::vec2(220.0, 48.0)),
                )
                .clicked();
            ui.add_space(15.0);

            // 3. Feedback message
            ui.label(egui::RichText::new(&self.message).size(18.0).color(self.message_color));

            // 4. History
            let mut history_to_reuse = None;
            let mut clear_history_clicked = false;

            if !self.history.is_empty() {
                ui.add_space(15.0);
                ui.separator();
                ui.add_space(5.0);
                ui.label(egui::RichText::new("Tidligere gæt (tryk for at genbruge):").size(14.0));
                ui.add_space(5.0);

                egui::Frame::NONE
                    .fill(egui::Color32::from_white_alpha(5))
                    .inner_margin(8.0)
                    .show(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .max_height(280.0)
                            .id_salt("history_scroll")
                            .show(ui, |ui| {
                                ui.horizontal_wrapped(|ui| {
                                    ui.spacing_mut().item_spacing = egui::vec2(8.0, 8.0);
                                    for h in self.history.iter().rev() {
                                        let btn = egui::Button::new(egui::RichText::new(h).size(16.0))
                                            .min_size(egui::vec2(65.0, 45.0));
                                        if ui.add(btn).clicked() {
                                            history_to_reuse = Some(h.clone());
                                        }
                                    }
                                });

                                ui.add_space(15.0);
                                if ui
                                    .button(egui::RichText::new("🗑 Ryd historik").color(egui::Color32::LIGHT_RED))
                                    .clicked()
                                {
                                    clear_history_clicked = true;
                                }
                            });
                    });
            }

            if clear_history_clicked {
                self.history.clear();
            }

            if let Some(val) = history_to_reuse {
                self.guess_input = val;
                text_input_response.request_focus();
            }

            let enter_pressed = text_input_response.lost_focus()
                && ctx.input(|i| i.key_pressed(egui::Key::Enter));

            if submit_clicked || enter_pressed {
                self.submit_guess();
                text_input_response.request_focus();
            }
        });
    }
}
