use crate::guess_game::{GuessError, GuessGame, WrongNumber};
use chrono::{Date, TimeZone, Utc};
use std::io;

mod guess_game;
#[allow(dead_code)] // FIXME: remove?
mod user_types;

fn main() {
    let (id, username): user_types::SimpleUser = (9000, String::from("afkay"));
    let afkay = user_types::User {
        id,
        username,
        gender: user_types::Gender::Trans(user_types::Sex::Female),
        born_at: Date::from(Utc.ymd(1997, 3, 3)),
        name: (String::from("Lara"), None),
        country: Some(user_types::Country::UA),
    };

    let game = GuessGame {
        win_number: 24,
        guess_range: (1, 100),
        hello_message: String::from("Hi. Please guess my age :)"),
    };
    game.say_hello();

    let mut input = String::new();

    loop {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading line");

        match game.make_guess(&input) {
            Ok(()) => {
                println!("Nice one! You guessed it right! :)");
                break;
            },
            Err(GuessError::OutOfRange) => {
                println!("Out of range. Valid range: {}", game.range_str());
                continue;
            }
            Err(GuessError::NaN) => {
                println!("Please enter a valid number.");
                continue;
            }
            Err(GuessError::WrongNumber(WrongNumber::TooLess)) => {
                println!("Too small value :) Try again.");
                continue;
            }
            Err(GuessError::WrongNumber(WrongNumber::TooMuch)) => {
                println!("Try smaller value :3");
                continue;
            }
        };
    }

    println!("Hello, world!");
    println!("{:#?}", afkay);
}
