use std::vec;

use juniper::{
    EmptySubscription, FieldResult, GraphQLEnum, GraphQLInputObject, GraphQLObject, RootNode,
};

#[derive(GraphQLInputObject)]
#[graphql(description = "Data to init a new blockchain")]
struct RpcCall {
    sender: String,
    message: String,
    signature: String,
}
impl TryFrom for RpcCall {
    fn try_from(value: T) -> Result<Self, Self::Error> {
        
    }
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct BlockchainInitiated {
    accounts: Vec<String>,
    values: Vec<String>,
}

#[derive(GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewHuman {
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn human(_id: String) -> FieldResult<Human> {
        Ok(Human {
            id: _id.to_owned(),
            name: "Luke".to_owned(),
            appears_in: vec![Episode::NewHope],
            home_planet: "Mars".to_owned(),
        })
    }
}

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    fn create_human(new_human: NewHuman) -> FieldResult<Human> {
        Ok(Human {
            id: "1234".to_owned(),
            name: new_human.name,
            appears_in: new_human.appears_in,
            home_planet: new_human.home_planet,
        })
    }
    fn init_new_blockchain(data: RpcCall) -> FieldResult<BlockchainInitiated> {
        Ok(BlockchainInitiated {
            accounts: data.message,
            values: vec::Vec::new(),
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}