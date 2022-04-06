#[derive(Clone, Copy)]
pub enum Role {
    Regular,
    Editor,
}

pub struct Data {
    pub id: i64,
    pub username: String,
    pub handle: String,
    pub email: String,
    pub role: Role,
    pub password_hash: String,
}

pub struct CreateData {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub struct PersonalData {
    pub email: String,
}

pub struct PublicData {
    pub username: String,
    pub handle: String,
    pub role: Role,
}