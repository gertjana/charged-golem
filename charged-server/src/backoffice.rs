use std::collections::HashMap;

use charger_model::{Charger, ChargerId, Command};

use crate::worker::{create, send};

pub struct BackOfficeState {
    chargers: HashMap<ChargerId, Charger>,
}

impl BackOfficeState {
    pub fn new() -> Self {
        Self {
            chargers: HashMap::new(),
        }
    }

    pub fn register_charger(&mut self, charger: Charger) {
        self.chargers
            .insert(charger.charger_id.clone(), charger.clone());

        create(charger);
    }

    pub fn send(&mut self, charger_id: ChargerId, command: Command) {
        send(charger_id, command);
    }

    pub fn get_charger(&self, charger_id: &ChargerId) -> Option<&Charger> {
        self.chargers.get(charger_id)
    }
}
