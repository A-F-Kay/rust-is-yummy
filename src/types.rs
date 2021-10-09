use chrono::prelude::Date;
use chrono::Utc;

// id, username
pub type SimpleUser = (i32, String);

// name, surname(optional)
type UserName = (String, Option<String>);

#[derive(Debug)]
pub enum Sex {
    Female,
    Male,
}

#[derive(Debug)]
pub enum Gender {
    Sex(Sex),
    Trans(Sex),
    Other(String),
}

#[derive(Debug)]
pub enum Country {
    US,
    RU,
    UA,
    // TODO: Add more?
}

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub gender: Gender,
    pub name: UserName,
    pub born_at: Date<Utc>,
    pub country: Option<Country>,
}
