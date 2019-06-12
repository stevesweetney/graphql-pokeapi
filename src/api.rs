use crate::types::{MoveDetailResponse, PokemonResponse};
use reqwest::{self, Error};
use std::fmt;

const BASE_API_URL: &str = "https://pokeapi.co/api/v2/";

pub fn pokemon_query(param: QueryParam) -> Result<PokemonResponse, Error> {
    let full_url = format!("{}pokemon/{}", BASE_API_URL, param.to_string());
    let data: PokemonResponse = reqwest::get(&full_url)?.json()?;

    Ok(data)
}

pub fn move_query(param: QueryParam) -> Result<MoveDetailResponse, Error> {
    let full_url = format!("{}moves/{}", BASE_API_URL, param.to_string());
    let data: MoveDetailResponse = reqwest::get(&full_url)?.json()?;

    Ok(data)
}

pub enum QueryParam<'a> {
    Name(&'a str),
    Id(i32),
}

impl<'a> fmt::Display for QueryParam<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            QueryParam::Name(name) => write!(f, "{}", name),
            QueryParam::Id(id) => write!(f, "{:?}", id),
        }
    }
}
