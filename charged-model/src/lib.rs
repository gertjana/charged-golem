use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ChargerId {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Charger {
    pub name: String,
    pub charger_id: ChargerId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub name: String,
    pub params: Vec<String>,
}
