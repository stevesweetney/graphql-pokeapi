use crate::api::{pokemon_query, QueryParam};
use crate::types::convert;
use juniper::{EmptyMutation, FieldResult, GraphQLEnum, GraphQLObject};
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(Deserialize, EnumString, Serialize, GraphQLEnum)]
pub enum Type {
    #[graphql(name = "normal")]
    #[strum(serialize = "normal")]
    Normal,
    #[graphql(name = "fire")]
    #[strum(serialize = "fire")]
    Fire,
    #[graphql(name = "fighting")]
    #[strum(serialize = "fighting")]
    Fighting,
    #[graphql(name = "water")]
    #[strum(serialize = "water")]
    Water,
    #[graphql(name = "flying")]
    #[strum(serialize = "flying")]
    Flying,
    #[graphql(name = "grass")]
    #[strum(serialize = "grass")]
    Grass,
    #[graphql(name = "poison")]
    #[strum(serialize = "poison")]
    Poison,
    #[graphql(name = "electric")]
    #[strum(serialize = "electric")]
    Electric,
    #[graphql(name = "ground")]
    #[strum(serialize = "ground")]
    Ground,
    #[graphql(name = "pyschic")]
    #[strum(serialize = "pyschic")]
    Psychic,
    #[graphql(name = "rock")]
    #[strum(serialize = "rock")]
    Rock,
    #[graphql(name = "ice")]
    #[strum(serialize = "ice")]
    Ice,
    #[graphql(name = "bug")]
    #[strum(serialize = "bug")]
    Bug,
    #[graphql(name = "dragon")]
    #[strum(serialize = "dragon")]
    Dragon,
    #[graphql(name = "ghost")]
    #[strum(serialize = "ghost")]
    Ghost,
    #[graphql(name = "dark")]
    #[strum(serialize = "dark")]
    Dark,
    #[graphql(name = "steel")]
    #[strum(serialize = "steel")]
    Steel,
    #[graphql(name = "fairy")]
    #[strum(serialize = "fairy")]
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
