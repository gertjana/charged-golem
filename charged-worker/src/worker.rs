use crate::ocpp_handlers::{_handle_reset, _handle_unlock_connector};
use charger_model::{Charger, Command};
use queues::{IsQueue, Queue};

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

    pub fn _handle_queue(&mut self) -> Result<String, &'static str> {
        if let Ok(command) = self.queue.remove() {
            println!(
                "Sending command {} with {} queue",
                command.name,
                command.params.join(", ")
            );
            let result = match command.name.as_str() {
                "unlock_connector" => {
                    serde_json::to_string(&_handle_unlock_connector(command.params))
                        .map_err(|_| "Error serializing response")
                }
                "reset" => serde_json::to_string(&_handle_reset(command.params))
                    .map_err(|_| "Error serializing response"),
                _ => {
                    println!("Unknown command {}", command.name);
                    Err::<String, &str>("Unknown command")
                }
            };
            return result;
        }
        Err::<String, &str>("Queue is empty")
    }
}
