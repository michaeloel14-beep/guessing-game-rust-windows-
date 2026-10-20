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

        // 1. BUND-PANEL: Klistrer Submit-knappen og historikken helt derned, hvor tommelfingeren er!
        egui::TopBottomPanel::bottom("ergonomic_controls")
            .frame(egui::Frame::none().inner_margin(10.0)) // Lidt luft i bunden
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    // Gør "Submit"-knappen stor og placerer den lige over tastaturet
                    let submit_clicked = ui.add(
                        egui::Button::new(egui::RichText::new("Submit Guess").size(22.0))
                            .min_size(egui::vec2(220.0, 50.0))
                    ).clicked();

                    // Hvis knappen klikkes (vi håndterer logikken efter inputfeltet er tegnet i central panel)
                    ctx.data_mut(|d| d.insert_temp(egui::Id::new("submit_clicked"), submit_clicked));

                    // Tidligere gæt som store, klikbare knapper lige under submit
                    if !self.history.is_empty() {
                        ui.add_space(15.0);
                        ui.separator();
                        ui.add_space(5.0);
                        ui.label(egui::RichText::new("Seneste gæt (tryk for at genbruge):").size(13.0));
                        ui.add_space(5.0);
                        
                        ui.horizontal_wrapped(|ui| {
                            for h in self.history.iter().rev() {
                                if ui.add(egui::Button::new(h).min_size(egui::vec2(55.0, 38.0))).clicked() {
                                    // Gemmer midlertidigt hvilket historik-element der blev trykket på
                                    ctx.data_mut(|d| d.insert_temp(egui::Id::new("history_pushed"), h.clone()));
                                }
                            }
                        });
                    }
                    ui.add_space(10.0);
                });
            });

        // 2. CENTRAL-PANEL: Holder titlen, forsøgene og inputfeltet i den øverste halvdel
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(50.0); // Luft under statusbaren
                
                ui.heading("🎯 Guess the number! 🎯");
                ui.add_space(5.0);
                ui.label(egui::RichText::new(format!("Attempt: #{}", self.attempts)).size(16.0));
                ui.add_space(20.0);

                // Gemmer tekstfeltets respons, så vi kan styre tastaturet
                let text_input_response = ui.add(
                    egui::TextEdit::singleline(&mut self.guess_input)
                        .hint_text("Indtast dit gæt her...")
                        .font(egui::FontId::proportional(20.0))
                );

                ui.add_space(20.0);
                ui.separator();
                ui.add_space(10.0);

                // Feedback-besked (f.eks. "1.95 er for stort!") midt på skærmen
                ui.label(egui::RichText::new(&self.message).size(18.0).color(self.message_color));

                // Hent klik-events fra bund-panelet
                let mut submit_triggered = ctx.data(|d| d.get_temp::<bool>(egui::Id::new("submit_clicked")).unwrap_or(false));
                
                // Hvis der blev trykket på en historik-knap i bunden
                if let Some(history_val) = ctx.data_mut(|d| d.remove_temp::<String>(egui::Id::new("history_pushed"))) {
                    self.guess_input = history_val;
                    text_input_response.request_focus();
                }

                // Hvis man trykker Enter på tastaturet
                if text_input_response.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                    submit_triggered = true;
                }

                // Kør spil-logikken hvis der skal submittes
                if submit_triggered {
                    let trimmed = self.guess_input.trim().to_string();
                    
                    if trimmed == "exit" {
                        std::process::exit(0);
                    }

                    if !trimmed.is_empty() {
                        if let Some(index) = self.history.iter().position(|x| x == &trimmed) {
                            self.history.remove(index);
                        }
                        if self.history.len() >= 6 {
                            self.history.remove(0);
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
                    text_input_response.request_focus(); // Tving tastaturet til at blive oppe
                }
            });
        });
    }
}
