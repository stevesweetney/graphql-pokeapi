use juniper::{GraphQLEnum, GraphQLObject};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, GraphQLEnum)]
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
    id: i32,
    name: String,
    effect: String,
}

#[derive(Deserialize, Serialize)]
pub struct Pokemon {
    id: i32,
    name: String,
    moves: Vec<Move>,
    types: Vec<Type>,
}

#[juniper::object]
impl Pokemon {
    fn id(&self) -> i32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }
}
