use charger_model::Charger;
use queues::{IsQueue, Queue};

pub struct WorkerState {
    pub charger: Option<Charger>,
    pub queue: Queue<(String, Vec<String>)>,
}
// TODO
// - run a websocket server (not yet supported in Golem)
// - loop that checks the queue and sends commands on the charger
// - support ocpp 1.6/2.0.1 messages

impl WorkerState {
    pub fn new() -> WorkerState {
        WorkerState {
            charger: None,
            queue: Queue::new(),
        }
    }

    pub fn add_to_queue(
        &mut self,
        name: String,
        params: Vec<String>,
    ) -> Result<Option<(String, Vec<String>)>, &str> {
        println!(
            "Adding command {name} with {p} queue",
            p = params.join(", ")
        );
        self.queue.add((name, params))
    }
}
