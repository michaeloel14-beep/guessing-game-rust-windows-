use eframe::egui;
use std::cmp::Ordering;
use std::ops::RangeInclusive;

#[derive(Clone, Copy, PartialEq)]
pub enum Screen {
    Menu,
    Playing,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

impl Difficulty {
    pub fn range(&self) -> RangeInclusive<f64> {
        match self {
            Difficulty::Easy => 1.0..=2.0,
            Difficulty::Normal => 1.0..=10.0,
            Difficulty::Hard => 1.0..=100.0,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Difficulty::Easy => "Let (1.0 - 2.0)",
            Difficulty::Normal => "Normal (1.0 - 10.0)",
            Difficulty::Hard => "Svær (1.0 - 100.0)",
        }
    }
}

pub struct GuessingGame {
    pub screen: Screen,
    pub difficulty: Difficulty,
    pub secret_number: f64,
    pub guess_input: String,
    pub attempts: u32,
    pub message: String,
    pub message_color: egui::Color32,
    pub history: Vec<String>,
}

impl Default for GuessingGame {
    fn default() -> Self {
        let mut game = Self {
            screen: Screen::Menu,
            difficulty: Difficulty::Easy,
            secret_number: 1.0,
            guess_input: String::new(),
            attempts: 0,
            message: String::new(),
            message_color: egui::Color32::WHITE,
            history: Vec::new(),
        };
        game.start_new_game(Difficulty::Easy);
        game.screen = Screen::Menu;
        game
    }
}

impl GuessingGame {
    pub fn start_new_game(&mut self, difficulty: Difficulty) {
        self.difficulty = difficulty;
        let range = difficulty.range();
        // Round to 2 decimal places so floating-point guesses are fair
        let raw = rand::random_range(range.clone());
        self.secret_number = (raw * 100.0).round() / 100.0;
        self.guess_input.clear();
        self.attempts = 0;
        self.history.clear();
        self.message = format!("Gæt et tal mellem {:.1} og {:.1}", range.start(), range.end());
        self.message_color = egui::Color32::WHITE;
        self.screen = Screen::Playing;
    }

    pub fn submit_guess(&mut self) {
        let trimmed = self.guess_input.trim().replace(',', "."); // Allow both ',' and '.' for decimals
        if trimmed.is_empty() {
            return;
        }
        if trimmed == "exit" {
            std::process::exit(0);
        }

        // Update history without duplicates
        if let Some(index) = self.history.iter().position(|x| x == &trimmed) {
            self.history.remove(index);
        }
        self.history.push(trimmed.clone());

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
    }
}
