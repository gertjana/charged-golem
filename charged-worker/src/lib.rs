use bindings::*;
use conversions::from_wit_charger;
use exports::charged::worker::api::{
    Api, Charger as WitCharger, CommandResult as WitCommandResult, WorkerResult as WitWorkerResult,
};
use once_cell::sync::Lazy;
mod worker;
use worker::*;

mod conversions;

fn with_state<T>(f: impl FnOnce(&mut WorkerState) -> T) -> T {
    unsafe { f(&mut STATE.state) }
}

struct WitState {
    state: Lazy<worker::WorkerState>,
}

static mut STATE: WitState = WitState {
    state: Lazy::new(worker::WorkerState::new),
};

struct ChargeWorkerImpl;

impl Api for ChargeWorkerImpl {
    fn initialize(charger: WitCharger) -> WitWorkerResult {
        with_state(|worker_state| {
            worker_state.charger = Some(from_wit_charger(charger.clone()));

            WitWorkerResult::Ok(format!("Initialized charger {}", charger.name))
        })
    }
    fn command(name: String, params: Vec<String>) -> WitCommandResult {
        with_state(
            |worker_state| match worker_state.add_to_queue(name.clone(), params.clone()) {
                Ok(_) => WitCommandResult::Ok("Command added to queue".to_string()),
                Err(err) => WitCommandResult::Error("Command rejected: ".to_string() + err),
            },
        )
    }
}
