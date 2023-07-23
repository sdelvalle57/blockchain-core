use std::io;
use web3::signing::{keccak256, recover};
use std::{vec, result, io::Error};
use serde_json;
use serde::{Deserialize, Serialize};

use juniper::{
    GraphQLEnum, GraphQLInputObject, GraphQLObject, FieldError
};

#[derive(Serialize, Deserialize, Debug)]
struct Accounts {
    data: Vec<String>
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

    fn message_to_vec(&self) -> result::Result<Vec<String>, FieldError>  {
        let v: Accounts = serde_json::from_str(&self.message)?;
        println!("{:?}", v);
        Ok(v.data)
    }

    fn validate_signature(&self) -> Result<(), io::Error>  {
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
            Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid Signature Format")),
            _ => ()
        };

        //TODO: message validation is not working, probably message needs to be arrayfy before recovering 
        let pub_key = recover(&eth_message, &signature.unwrap()[..64], 0).unwrap();
        let pub_key = format!("{:02X?}", pub_key);
        if pub_key == self.sender {
            return Ok(())
        }
        Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid Signature"))
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
                    chain_id: self.chain_id
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
    MATIC_MAINNET = 137
}

impl TryFrom<i32> for ChainName {
    type Error = Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ChainName::MAINNET),
            137 => Ok(ChainName::MATIC_MAINNET),
            _ => Err(Error::new(std::io::ErrorKind::InvalidInput, format!("Invalid Chain, use chain ids {} or {}", 1, 2)))
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
    chain_id: i32
}




#[test]
fn test_recover() {
    let message: RpcCall = {
        RpcCall { 
            message: "0x63f9a92d8d61b48a9fff8d58080425a3012d05c8igwyk4r1o7o".to_string(), 
            signature: "382a3e04daf88f322730f6a2972475fc5646ea8c4a7f3b5e83a90b10ba08a7364cd2f55348f2b6d210fbed7fc485abf19ecb2f3967e410d6349dd7dd1d4487751b".to_string(),
            sender: "0x63f9a92d8d61b48a9fff8d58080425a3012d05c8".to_string(), 
            chain_id: 1
        }
    };
    let res = message.validate_signature().unwrap();
    assert_eq!(res, ());
    
}