use crate::guess_game::GuessGame;
use chrono::{Date, TimeZone, Utc};

mod guess_game;
#[allow(dead_code)] // FIXME: remove?
mod user_types;

fn main() {
    let afkay_basic: user_types::SimpleUser = (9000, String::from("afkay"));
    let afkay = user_types::User::new(
        afkay_basic,
        user_types::Gender::Trans(user_types::Sex::Female),
        (String::from("Lara"), None),
        Date::from(Utc.ymd(1997, 3, 3)),
        Some(user_types::Country::UA),
    );

    let game = GuessGame {
        win_number: afkay.get_age(),
        guess_range: (1, 100),
        hello_message: String::from("Hi. Please guess my age :)"),
    };
    game.play();
}
