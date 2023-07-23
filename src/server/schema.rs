use std::{vec, io::Error};

use juniper::{
    EmptySubscription, FieldResult, GraphQLObject, RootNode,
};

use super::rpc_call::{RpcCall, BlockchainInitiated};



#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Human {
    id: String,
    name: String,
    home_planet: String,
}

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn human(_id: String) -> FieldResult<Human> {
        Ok(Human {
            id: _id.to_owned(),
            name: "Luke".to_owned(),
            home_planet: "Mars".to_owned(),
        })
    }
}

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    
    fn init_new_blockchain(data: RpcCall) -> FieldResult<BlockchainInitiated> {
        data.try_into()
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}