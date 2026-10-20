use eframe::egui;
use std::cmp::Ordering;

#[cfg_attr(target_os = "android", no_mangle)]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    let mut options = eframe::NativeOptions::default();
    options.android_app = Some(app);
    let _ = eframe::run_native(
        "Guessing Game",
        options,
        Box::new(|_cc| Ok(Box::new(GuessingGame::default()))),
    );
}

#[allow(dead_code)]
fn main() {}

struct GuessingGame {
    secret_number: f64,
    guess_input: String,
    attempts: u32,
    message: String,
    message_color: egui::Color32,
    history: Vec<String>,
}

impl Default for GuessingGame {
    fn default() -> Self {
        Self {
            secret_number: rand::random_range(1.0..=2.0),
            guess_input: String::new(),
            attempts: 0,
            message: "Please input your guess (1.0 to 2.0)".to_owned(),
            message_color: egui::Color32::WHITE,
            history: Vec::new(),
        }
    }
}


impl eframe::App for GuessingGame {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Tvinger appen til at være stor og nem at læse på mobilskærme
        ctx.set_pixels_per_point(1.5);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(40.0); // Luft under telefonens statusbar
                
                ui.heading("🎯 Guess the number! 🎯");
                ui.add_space(5.0);
                ui.label(egui::RichText::new(format!("Attempt: #{}", self.attempts)).size(16.0));
                ui.add_space(15.0);

                // 1. INPUT FELTET
                let text_input_response = ui.add(
                    egui::TextEdit::singleline(&mut self.guess_input)
                        .hint_text("Indtast dit gæt her...")
                        .font(egui::FontId::proportional(20.0))
                );

                ui.add_space(15.0);

                // 2. SUBMIT KNAPPEN
                let submit_clicked = ui.add(
                    egui::Button::new(egui::RichText::new("Submit Guess").size(22.0))
                        .min_size(egui::vec2(220.0, 48.0))
                ).clicked();

                ui.add_space(15.0);

                // 3. RESPONS / FEEDBACK BESKED
                ui.label(egui::RichText::new(&self.message).size(18.0).color(self.message_color));

                // 4. HISTORIK (Nu meget længere lodret og med scrollbar!)
                let mut history_to_reuse = None;
                let mut clear_history_clicked = false;

                if !self.history.is_empty() {
                    ui.add_space(15.0);
                    ui.separator();
                    ui.add_space(5.0);
                    ui.label(egui::RichText::new("Tidligere gæt (tryk for at genbruge):").size(14.0));
                    ui.add_space(5.0);
                    
                    // Gør rammen lodret meget længere (ændret til 320.0) og tilføjer en svag baggrundsfarve
                    egui::Frame::none()
                        .fill(egui::Color32::from_white_alpha(5)) // Gør scroll-området let synligt
                        .inner_margin(8.0)
                        .show(ui, |ui| {
                            egui::ScrollArea::vertical()
                                .max_height(320.0) // Kæmpe lodret boks til masser af gæt!
                                .id_source("history_scroll")
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

                                    // Tilføjer en knap i bunden af listen til at nulstille historikken
                                    ui.add_space(15.0);
                                    if ui.button(egui::RichText::new("🗑 Ryd historik-liste").color(egui::Color32::LIGHT_RED)).clicked() {
                                        clear_history_clicked = true;
                                    }
                                });
                        });
                }

                // --- SPIL-LOGIK OG INPUT-HÅNDTERING ---
                
                if clear_history_clicked {
                    self.history.clear();
                }

                if let Some(history_val) = history_to_reuse {
                    self.guess_input = history_val;
                    text_input_response.request_focus();
                }

                let enter_pressed = text_input_response.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter));
                
                if submit_clicked || enter_pressed {
                    let trimmed = self.guess_input.trim().to_string();
                    
                    if trimmed == "exit" {
                        std::process::exit(0);
                    }

                    if !trimmed.is_empty() {
                        if let Some(index) = self.history.iter().position(|x| x == &trimmed) {
                            self.history.remove(index);
                        }
                        self.history.push(trimmed.clone());
                    }

                    match trimmed.parse::<f64>() {
                        Ok(num) => {
                            self.attempts += 1;
                            match num.total_cmp(&self.secret_number) {
                                Ordering::Less => {
                                    self.message = format!("{num} er for lille!");
                                    self.message_color = egui::Color32::from_rgb(255, 100, 100);
                                }
                                Ordering::Greater => {
                                    self.message = format!("{num} er for stort!");
                                    self.message_color = egui::Color32::from_rgb(255, 100, 100);
                                }
                                Ordering::Equal => {
                                    self.message = format!("🎉 Korrekt! Du brugte {} gæt!", self.attempts);
                                    self.message_color = egui::Color32::LIGHT_GREEN;
                                }
                            }
                        }
                        Err(_) => {
                            self.message = "Det er ikke et tal! Prøv igen.".to_owned();
                            self.message_color = egui::Color32::YELLOW;
                        }
                    }
                    
                    self.guess_input.clear();
                    text_input_response.request_focus();
                }
            });
        });
    }
}
