use std::*;
use std::cmp::Ordering;
use eframe::egui::{self};
use rand::Rng;

struct GameStatus {
    result: String,
    attempts: i32,
    secret_number: u32,
}

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    let min_number = 1;
    let max_number = 10;

    let mut guess = String::new();
    let mut game_status = GameStatus { result: String::new(), attempts: 0, secret_number: rand::thread_rng().gen_range(min_number..max_number) };

    eframe::run_simple_native("Guess Game", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let mut guess_label = ui.label(format!("Choose a number between {min_number} and {max_number}: "));
                let response = ui.text_edit_singleline(&mut guess).labelled_by(guess_label.id);
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    process_guess(&guess, &mut game_status);
                }
            });
            if ui.button("Try out!").clicked() {
                process_guess(&guess, &mut game_status);
            }
            if ui.button("Reset").clicked() {
                reset_game(&mut game_status, min_number, max_number);
            }
            ui.label(format!("Attempts: {}", game_status.attempts));
            ui.label(format!("{}", game_status.result));
        });
    })
}

fn process_guess(guess: &String, game_status: &mut GameStatus) {
    let guess: u32 = guess.trim().parse().expect("Error");

    match guess.cmp(&game_status.secret_number) {
        Ordering::Less =>  {
            game_status.attempts += 1;
            game_status.result = "Too small!".to_string();
        },
        Ordering::Equal => {
            game_status.attempts += 1;
            game_status.result = "You win!".to_string();
        },
        Ordering::Greater => {
            game_status.attempts += 1;
            game_status.result = "To big!".to_string();

        },
    }
}

fn reset_game(game_status: &mut GameStatus, min_number: u32, max_number: u32) {
    game_status.attempts = 0;
    game_status.result = String::new();
    game_status.secret_number = rand::thread_rng().gen_range(min_number..max_number);
}