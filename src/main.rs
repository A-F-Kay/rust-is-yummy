use chrono::{Date, TimeZone, Utc};

#[allow(dead_code)] // FIXME: remove?
mod types;

fn main() {
    let (id, username): types::SimpleUser = (9000, String::from("afkay"));
    let afkay = types::User {
        id,
        username,
        gender: types::Gender::Trans(types::Sex::Female),
        born_at: Date::from(Utc.ymd(1997, 3, 3)),
        name: (String::from("Lara"), None),
        country: Some(types::Country::UA),
    };

    println!("Hello, world!");
    println!("{:#?}", afkay);
}
