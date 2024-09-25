use std::*;
use std::cmp::Ordering;
use eframe::egui::{self};
use rand::Rng;

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    let min_number = 1;
    let max_number = 500;
    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(min_number..max_number);
    let mut result = "";

    eframe::run_simple_native("Guess Game", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let mut guess_label = ui.label(format!("Digite um numero entre {min_number} e {max_number}: "));
                let response = ui.text_edit_singleline(&mut guess).labelled_by(guess_label.id);
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    result = process_guess(&guess, &secret_number);
                }

            });
            if ui.button("Try out!").clicked() {
                result = process_guess(&guess, &secret_number);
            }
            ui.label(format!("{result}"));
        });
    })
}

fn process_guess(guess: &String, secret_number: &u32) -> &'static str {
    let guess: u32 = guess.trim().parse().expect("Erro");
    let mut result = "";

    match guess.cmp(secret_number) {
        Ordering::Less =>  {
            result = "Too small!";
        },
        Ordering::Equal => {
            result = "You win!";
        },
        Ordering::Greater => {
            result = "To big!";

        },
    }
    return result;
}