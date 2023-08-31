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

pub fn authorize(charger_id: ChargerId, token: String) -> bool {
    let invocation_key = get_invocation_key(charger_id.clone());
    // _invoke_and_await_function<???>(charger_id, token, invocation_key, "authorize".to_string())
    false
}

// API Calls

fn get_invocation_key(charger_id: ChargerId) -> String {
    let client = reqwest::Client::new();
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
    let client = reqwest::Client::new();
    let component_id = "charger";
    let instance_id = format!("charger-{}", charger_id.id);
    let url = format!("{COMP_BASE_URL}/{component_id}/instances/{instance_id}/invoke?invocation-key={invocation_key}&function-name={function_name}");
    client
        .post(url)
        .json(&body)
        .header("Authorization", format!("Bearer: {}", *GOLEM_API_TOKEN))
        .send()
        .unwrap();
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
    let client = reqwest::Client::new();
    let component_id = "charger";
    let instance_id = format!("charger-{}", charger_id.id);
    let url = format!("{COMP_BASE_URL}/{component_id}/instances/{instance_id}/invoke-and-await?invocation-key={invocation_key}&function-name={function_name}");
    let response = client
        .post(url)
        .json(&body)
        .header("Authorization", format!("Bearer: {}", *GOLEM_API_TOKEN))
        .send();
    match response {
        Ok(response) => {
            let contents = response.text().unwrap();
            serde_json::from_str(&contents)
                .with_context(|| {
                    format!("Unable to deserialise response. Body was: \"{contents}\"")
                })
                .unwrap()
        }
        Err(error) => {
            panic!("Error: {error:?}")
        }
    }
}
