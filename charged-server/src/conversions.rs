use crate::bindings::exports::charged::server::api::{
    Charger as WitCharger, ChargerId as WitChargerId, Command as WitCommand,
};
use charger_model::{Charger, ChargerId, Command};

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

pub fn from_wit_command(command: WitCommand) -> Command {
    Command {
        name: command.name,
        params: command.params,
    }
}
