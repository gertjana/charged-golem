use charger_model::{Charger, Command};
use queues::{IsQueue, Queue};
use crate::ocpp_handlers::_handle_unlock_connector;

pub struct WorkerState {
    pub charger: Option<Charger>,
    pub queue: Queue<Command>,
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

    pub fn add_to_queue(&mut self, command: Command) -> Result<Option<Command>, &str> {
        println!(
            "Adding command {} with {} queue",
            command.name,
            command.params.join(", ")
        );
        self.queue.add(command)
    }

    pub fn _handle_queue(&mut self) {
        if let Ok(command) = self.queue.remove() {
            println!(
                "Sending command {} with {} queue",
                command.name,
                command.params.join(", ")
            );
            match command.name.as_str() {
                "unlock_connector" => 
                   _handle_unlock_connector(command.params),
                _ => panic!("Unknown command {}", command.name),            
            };
        }
    }
}
