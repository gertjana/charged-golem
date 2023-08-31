use bindings::*;
use exports::charged::server::api::{
    Api, Charger as WitCharger, ChargerId as WitChargerId, ChargerResult as WitChargerResult,
    Command as WitCommand,
};
use once_cell::sync::Lazy;

mod backoffice;
mod conversions;
mod worker;

struct WitState {
    state: Lazy<backoffice::BackOfficeState>,
}

static mut STATE: WitState = WitState {
    state: Lazy::new(backoffice::BackOfficeState::new),
};

fn with_state<T>(f: impl FnOnce(&mut backoffice::BackOfficeState) -> T) -> T {
    unsafe { f(&mut STATE.state) }
}

struct ChargeNetWorkImpl;

impl Api for ChargeNetWorkImpl {
    fn register(charger: WitCharger) -> WitChargerResult {
        with_state(|backoffice_state| {
            backoffice_state.register_charger(conversions::from_wit_charger(charger.clone()));
            WitChargerResult::Ok(charger)
        })
    }

    fn send(charger_id: WitChargerId, command: WitCommand) {
        with_state(|backoffice_state| {
            backoffice_state.send(
                conversions::from_wit_charger_id(charger_id.clone()),
                conversions::from_wit_command(command.clone()),
            );
        })
    }

    fn authorize(charger_id: WitChargerId, _token: String) -> bool {
        with_state(|backoffice_state| {
            // for now accept all authorizations as valid
            backoffice_state
                .get_charger(&conversions::from_wit_charger_id(charger_id.clone()))
                .is_some()
        })
    }
}
