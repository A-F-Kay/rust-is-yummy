use chrono::{prelude::Date, Datelike, Utc};

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
    id: i32,
    username: String,
    gender: Gender,
    name: UserName,
    born_at: Date<Utc>,
    country: Option<Country>,
}

impl User {
    pub fn new(
        basic_info: SimpleUser,
        gender: Gender,
        name: UserName,
        born_at: Date<Utc>,
        country: Option<Country>,
    ) -> Self {
        let (id, username) = basic_info;

        User {
            id,
            username,
            gender,
            name,
            born_at,
            country,
        }
    }

    pub fn get_age(&self) -> i32 {
        let today = Utc::now().date();
        let (month, day) = (today.month(), today.day());

        let (birth_month, birth_day) = (self.born_at.month(), self.born_at.day());

        let passed_birthday_this_year = if month == birth_month {
            day >= birth_day
        } else {
            month > birth_month
        };

        let age_modifier = if passed_birthday_this_year { 0 } else { -1 };

        (today.year() - self.born_at.year()) + age_modifier
    }
}
