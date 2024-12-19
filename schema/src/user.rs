#[derive(Clone, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String, // In a real app, store hashed passwords
}

