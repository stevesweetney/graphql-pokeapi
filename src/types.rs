use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TypeResponse {
    slot: u32,
    #[serde(rename = "type")]
    type_data: Data,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MoveResponse {
    #[serde(rename = "move")]
    move_data: Data,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    url: String,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PokemonResponse {
    id: i32,
    name: String,
    moves: Vec<MoveResponse>,
    types: Vec<TypeResponse>,
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
