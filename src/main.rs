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
        // Gør alt indhold 40% større (Zoom ind). Standard er 1.0. Juster tallet efter smag!
        ctx.set_pixels_per_point(1.4);

        // Skaber en tom, usynlig bar i toppen for at skubbe spillet væk fra statusbaren
        egui::TopBottomPanel::top("status_bar_spacer")
            .frame(egui::Frame::none()) 
            .show(ctx, |ui| {
                ui.add_space(35.0); // Da vi har zoomet ind, behøver dette tal ikke være lige så højt som før
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Guess the number!");
            ui.add_space(10.0);
            ui.label(format!("Attempt: #{}", self.attempts));
            
            ui.horizontal(|ui| {
                ui.label("Your guess:");
                ui.text_edit_singleline(&mut self.guess_input);
            });

            if ui.button("Submit Guess").clicked() {
                let trimmed = self.guess_input.trim().to_string();
                if trimmed == "exit" {
                    std::process::exit(0);
                }

                if !trimmed.is_empty() && !self.history.contains(&trimmed) {
                    if self.history.len() >= 8 {
                        self.history.remove(0);
                    }
                    self.history.push(trimmed.clone());
                }

                match trimmed.parse::<f64>() {
                    Ok(num) => {
                        self.attempts += 1;
                        match num.total_cmp(&self.secret_number) {
                            Ordering::Less => {
                                self.message = "Too small!".to_owned();
                                self.message_color = egui::Color32::RED;
                            }
                            Ordering::Greater => {
                                self.message = "Too big!".to_owned();
                                self.message_color = egui::Color32::RED;
                            }
                            Ordering::Equal => {
                                self.message = format!("You guessed correctly in {} guesses!", self.attempts);
                                self.message_color = egui::Color32::LIGHT_GREEN;
                            }
                        }
                    }
                    Err(_) => {
                        self.message = "Thats not a number! type in a number.".to_owned();
                        self.message_color = egui::Color32::GREEN;
                    }
                }
                self.guess_input.clear();
            }

            ui.separator();
            ui.colored_label(self.message_color, &self.message);

            if !self.history.is_empty() {
                ui.separator();
                ui.label("Recent History (tap to reuse):");
                for h in self.history.iter().rev() {
                    if ui.small_button(h).clicked() {
                        self.guess_input = h.clone();
                    }
                }
            }
        });
    }
