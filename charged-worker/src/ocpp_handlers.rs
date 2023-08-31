use rust_ocpp::v2_0_1::messages::unlock_connector::{UnlockConnectorRequest, UnlockConnectorResponse};
use rust_ocpp::v2_0_1::enumerations::unlock_status_enum_type::UnlockStatusEnumType;

pub fn _handle_unlock_connector(params: Vec<String>) -> UnlockConnectorResponse {
    let _req = UnlockConnectorRequest{
        evse_id: params[0].parse::<i64>().unwrap(),
        connector_id: params[1].parse::<i64>().unwrap()
    };

    // TODO: pass the message to the charger and return the response instead of this dummy response

    let res = UnlockConnectorResponse{
        status: UnlockStatusEnumType::Unlocked,
        status_info: None};
    println!("Sending response {res:?}");
    res 
}