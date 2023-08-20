use std::io;

pub struct Transaction {
    from: String,
    to: String,
    signature: String,
    value: u128,
    timestamp: u128,
    nonce: u128,
}

impl Transaction {
    pub fn build_genesys_transaction(
        accounts: Vec<String>,
        values: Vec<u128>,
    ) -> Result<(), io::Error> {
        if accounts.len() != values.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid Length",
            ))
        }
        Ok(())
    }
}
