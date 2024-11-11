use crate::user;

#[derive(Clone, Debug)]
pub struct Transaction {
    sender: user::User,
    recipient: user::User,
    amount: u16,
}


pub struct TransactionHashable {
    pub sender: String,
    pub recipient: String,
    pub amount: u16,
}

impl Transaction {
    pub fn to_hashable(&self) -> TransactionHashable {
        TransactionHashable { sender: self.sender.username.to_owned() , recipient: self.recipient.username.to_owned() , amount: self.amount }
    }
}