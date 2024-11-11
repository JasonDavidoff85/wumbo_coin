use crate::key::Key;

#[derive(Clone, Debug)]
pub struct User {
    pub username: String,
    balance: u16,
    key: Key
}

