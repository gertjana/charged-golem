use bindings::*;
use exports::charged::worker::api::{Charger as WitCharger, ChargerId as WitChargerId};

use charger_model::{Charger, ChargerId};

pub fn from_wit_charger_id(charger_id: WitChargerId) -> ChargerId {
    ChargerId { id: charger_id.id }
}

pub fn from_wit_charger(charger: WitCharger) -> Charger {
    Charger {
        name: charger.name,
        charger_id: ChargerId {
            id: from_wit_charger_id(charger.charger_id).id,
        },
    }
}
