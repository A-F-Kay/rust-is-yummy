// id, username
pub type SimpleUser = (i32, String);

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
pub struct User {
    pub id: i32,
    pub username: String,
    pub gender: Gender,
}
