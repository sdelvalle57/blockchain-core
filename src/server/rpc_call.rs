use serde::{Deserialize, Serialize};
use serde_json;
use std::io;
use std::{io::Error, result, vec};
use web3::signing::{keccak256, recover};

use juniper::{FieldError, GraphQLEnum, GraphQLInputObject, GraphQLObject};

#[derive(Serialize, Deserialize, Debug)]
struct Accounts {
    data: Vec<String>,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Data to init a new blockchain")]
pub struct RpcCall {
    sender: String,
    message: String,
    chain_id: i32,
    signature: String,
}

impl RpcCall {
    fn message_to_vec(&self) -> result::Result<Vec<String>, FieldError> {
        let v: Accounts = serde_json::from_str(&self.message)?;
        println!("{:?}", v);
        Ok(v.data)
    }

    fn validate_signature(&self) -> Result<(), io::Error> {
        let eth_message = keccak256(
            format!(
                "{}{}{}",
                "\x19Ethereum Signed Message:\n",
                self.message.len(),
                self.message
            )
            .as_bytes(),
        );
        println!("{}", self.signature);
        let signature = hex::decode(&self.signature);
        match signature {
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid Signature Format",
                ))
            }
            _ => (),
        };

        //TODO: message validation is not working, probably message needs to be arrayfy before recovering
        let pub_key = recover(&eth_message, &signature.unwrap()[..64], 0).unwrap();
        let pub_key = format!("{:02X?}", pub_key);
        if pub_key == self.sender {
            return Ok(());
        }
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid Signature",
        ))
    }
}

impl TryInto<BlockchainInitiated> for RpcCall {
    type Error = FieldError;
    fn try_into(self) -> Result<BlockchainInitiated, Self::Error> {
        // Ok(BlockchainInitiated {
        //     accounts: vec::Vec::new(),
        //     values: vec::Vec::new(),
        //     chain: ChainName::MAINNET,
        //     chain_id: self.chain_id
        // })
        match self.message_to_vec() {
            Ok(accounts) => {
                let chain = self.chain_id.try_into()?;

                self.validate_signature()?;
                Ok(BlockchainInitiated {
                    accounts,
                    values: vec::Vec::new(),
                    chain,
                    chain_id: self.chain_id,
                })
            }
            Err(error) => {
                println!("{:?}", error);
                Err(error)
            }
        }
    }
}

#[derive(GraphQLEnum)]
#[graphql(description = "Types of blockchains")]
enum ChainName {
    MAINNET = 1,

    #[allow(non_camel_case_types)]
    MATIC_MAINNET = 137,
}

impl TryFrom<i32> for ChainName {
    type Error = Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ChainName::MAINNET),
            137 => Ok(ChainName::MATIC_MAINNET),
            _ => Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid Chain, use chain ids {} or {}", 1, 2),
            )),
        }
    }
}

impl ToString for ChainName {
    fn to_string(&self) -> String {
        match self {
            ChainName::MAINNET => String::from("MAINNET"),
            ChainName::MATIC_MAINNET => String::from("MATIC_MAINNET"),
        }
    }
}

#[derive(GraphQLObject)]
#[graphql(description = "Response for a successful init blovkchain request")]
pub struct BlockchainInitiated {
    accounts: Vec<String>,
    values: Vec<String>,
    chain: ChainName,
    chain_id: i32,
}

pub fn eth_message(message: String) -> [u8; 32] {
    keccak256(
        format!(
            "{}{}{}",
            "\x19Ethereum Signed Message:\n",
            message.len(),
            message
        )
        .as_bytes(),
    )
}

#[test]
fn test_recover() {
    let account = "0x23C6599aAdF44Be7cbaD6D9051bb4C2255b2f713".to_string();
    let message = "heelo".to_string();
    let message = eth_message(message);
    let signature = hex::decode("7dabc0471d53ed34ec21c0257d8b40f7234d2bce8ccdbfba540a8f2be56183a954e49162be4768c14efeb6ad7f1a836d2a3385df628ede34f7047af65f18cc621c").unwrap();
    println!("{} {:?} {:?}", account, message, signature);
    let pubkey = recover(&message, &signature[..64], 0);
    assert!(pubkey.is_ok());
    let pubkey = pubkey.unwrap();
    let pubkey = format!("{:02X?}", pubkey);
    assert_eq!(account, pubkey)
}
