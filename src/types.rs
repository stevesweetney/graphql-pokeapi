use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TypeResponse {
    slot: u32,
    #[serde(rename = "type")]
    pub type_data: Data,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MoveResponse {
    #[serde(rename = "move")]
    pub move_data: Data,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    pub url: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PokemonResponse {
    pub id: i32,
    pub name: String,
    pub moves: Vec<MoveResponse>,
    pub types: Vec<TypeResponse>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EffectEntry {
    effect: String,
    short_effect: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MoveDetailResponse {
    id: i32,
    name: String,
    effect_entries: Vec<EffectEntry>,
}

pub mod convert {
    use super::*;
    use crate::schema::{Move, Pokemon, Type};

    pub fn response_to_pokemon(res: PokemonResponse) -> Pokemon {
        let id = res.id;
        let name = res.name;
        let types: Vec<Type> = res
            .types
            .iter()
            .map(|type_res| {
                type_res
                    .type_data
                    .name
                    .parse()
                    .expect("Could not parse type name")
            })
            .collect();
        let moves: Vec<Move> = res
            .moves
            .iter()
            .map(|move_res| Move {
                url: move_res.move_data.url.clone(),
                name: move_res.move_data.name.clone(),
            })
            .collect();

        Pokemon {
            id,
            name,
            types,
            moves,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    #[test]
    fn deserialize_pokemon() {
        let sample_data = include_str!("../samples/pokemon.json");
        let json = serde_json::from_str::<PokemonResponse>(sample_data);
        assert!(json.is_ok());
    }

    #[test]
    fn deserialize_move() {
        let sample_data = include_str!("../samples/move.json");
        let json = serde_json::from_str::<MoveDetailResponse>(sample_data);
        assert!(json.is_ok());
    }
}
