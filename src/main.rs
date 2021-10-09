#[allow(dead_code)] // FIXME: remove?
mod types;

fn main() {
    let (id, username): types::SimpleUser = (9000, String::from("afkay"));
    let afkay = types::User {
        id,
        username,
        gender: types::Gender::Trans(types::Sex::Female),
    };

    println!("Hello, world!");
    println!("{:?}", afkay);
}
