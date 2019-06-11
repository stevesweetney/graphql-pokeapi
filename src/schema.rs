use crate::api::{pokemon_query, QueryParam};
use crate::types::convert;
use juniper::{EmptyMutation, FieldResult, GraphQLEnum, GraphQLObject};
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(Deserialize, EnumString, Serialize, GraphQLEnum)]
pub enum Type {
    #[graphql(name = "normal")]
    Normal,
    #[graphql(name = "fire")]
    Fire,
    #[graphql(name = "fighting")]
    Fighting,
    #[graphql(name = "water")]
    Water,
    #[graphql(name = "flying")]
    Flying,
    #[graphql(name = "grass")]
    Grass,
    #[graphql(name = "poison")]
    Poison,
    #[graphql(name = "electric")]
    Electric,
    #[graphql(name = "ground")]
    Ground,
    #[graphql(name = "pyschic")]
    Psychic,
    #[graphql(name = "rock")]
    Rock,
    #[graphql(name = "ice")]
    Ice,
    #[graphql(name = "bug")]
    Bug,
    #[graphql(name = "dragon")]
    Dragon,
    #[graphql(name = "ghost")]
    Ghost,
    #[graphql(name = "dark")]
    Dark,
    #[graphql(name = "steel")]
    Steel,
    #[graphql(name = "fairy")]
    Fairy,
}

#[derive(Deserialize, Serialize, GraphQLObject)]
pub struct Move {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Serialize, GraphQLObject)]
pub struct Pokemon {
    pub id: i32,
    pub name: String,
    pub moves: Vec<Move>,
    pub types: Vec<Type>,
}

pub struct Query;

#[juniper::object]
impl Query {
    fn pokemon(id: Option<i32>, name: Option<String>) -> FieldResult<Pokemon> {
        let pokemon_res = match (id, name) {
            (Some(val), _) => pokemon_query(QueryParam::Id(val)),
            (None, Some(val)) => pokemon_query(QueryParam::Name(&val)),
            // Default to showing the pokemon with id 1
            // if no args are present
            _ => pokemon_query(QueryParam::Id(1)),
        }?;

        let p = convert::response_to_pokemon(pokemon_res);
        Ok(p)
    }
}

pub type Schema = juniper::RootNode<'static, Query, EmptyMutation<()>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, EmptyMutation::new())
}
