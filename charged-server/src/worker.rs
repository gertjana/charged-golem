use std::env;

use anyhow::Context;
use charger_model::{Charger, ChargerId, Command};
use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;
use serde::Serialize;

static COMP_BASE_URL: &str = "https://release.api.golem.cloud/components";

static GOLEM_API_TOKEN: Lazy<String, fn() -> String> = Lazy::<String, _>::new(|| {
    "Bearer: ".to_string()
        + &env::var_os("GOLEM_API_TOKEN")
            .unwrap()
            .into_string()
            .unwrap()
});

pub fn create(charger: Charger) {
    let _ = create_instance(charger.charger_id.clone());
    let invocation_key = get_invocation_key(charger.charger_id.clone());
    let body = charger.clone();
    invoke_function::<Charger>(
        charger.charger_id,
        body,
        invocation_key,
        "initialize".to_string(),
    )
}

pub fn send(charger_id: ChargerId, command: Command) {
    let invocation_key = get_invocation_key(charger_id.clone());
    invoke_function::<Command>(charger_id, command, invocation_key, "command".to_string())
}

// API Calls

fn create_instance(charger_id: ChargerId) -> Result<(), reqwest::Error>{
    let client = reqwest::blocking::Client::new();
    let component_id = "charger";
    let instance_id = format!("charger-{}", charger_id.id);
    let url = format!("{COMP_BASE_URL}/{component_id}/instances?instance-name={instance_id}",);
    if let Err(error) = client
            .post(url)
            .header("Authorization", format!("Bearer: {}", *GOLEM_API_TOKEN))
            .send() {
        Err(error)
    } else {
        Ok(())
    }    
}

fn get_invocation_key(charger_id: ChargerId) -> String {
    let client = reqwest::blocking::Client::new();
    let component_id = "charger";
    let instance_id = format!("charger-{}", charger_id.id);
    let url = format!("{COMP_BASE_URL}/{component_id}/instances/{instance_id}/key",);
    client
        .post(url)
        .header("Authorization", format!("Bearer: {}", *GOLEM_API_TOKEN))
        .send()
        .unwrap()
        .text()
        .unwrap()
}

fn invoke_function<S>(charger_id: ChargerId, body: S, invocation_key: String, function_name: String)
where
    S: Serialize,
{
    let client = reqwest::blocking::Client::new();
    let component_id = "charger";
    let instance_id = format!("charger-{}", charger_id.id);
    let url = format!("{COMP_BASE_URL}/{component_id}/instances/{instance_id}/invoke?invocation-key={invocation_key}&function-name={function_name}");
    let _ = client
        .post(url)
        .json(&body)
        .header("Authorization", format!("Bearer: {}", *GOLEM_API_TOKEN))
        .send();
}

fn _invoke_and_await_function<S, D>(
    charger_id: ChargerId,
    body: S,
    invocation_key: String,
    function_name: String,
) -> D
where
    S: Serialize,
    D: DeserializeOwned,
{
    let client = reqwest::blocking::Client::new();
    let component_id = "charger";
    let instance_id = format!("charger-{}", charger_id.id);
    let url = format!("{COMP_BASE_URL}/{component_id}/instances/{instance_id}/invoke-and-await?invocation-key={invocation_key}&function-name={function_name}");
    let response = client
        .post(url)
        .json(&body)
        .header("Authorization", format!("Bearer: {}", *GOLEM_API_TOKEN))
        .send()
        .unwrap()
        .text()
        .unwrap();

    serde_json::from_str(&response)
        .with_context(|| {
            format!("Unable to deserialise response. Body was: \"{response}\"")
        })
        .unwrap()
}
