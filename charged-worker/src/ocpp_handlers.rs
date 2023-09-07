use rust_ocpp::v2_0_1::enumerations::reset_enum_type::ResetEnumType;
use rust_ocpp::v2_0_1::enumerations::reset_status_enum_type::ResetStatusEnumType;
use rust_ocpp::v2_0_1::enumerations::unlock_status_enum_type::UnlockStatusEnumType;
use rust_ocpp::v2_0_1::messages::reset::{ResetRequest, ResetResponse};
use rust_ocpp::v2_0_1::messages::unlock_connector::{
    UnlockConnectorRequest, UnlockConnectorResponse,
};

pub fn _handle_unlock_connector(params: Vec<String>) -> UnlockConnectorResponse {
    let _req = UnlockConnectorRequest {
        evse_id: params[0].parse::<i64>().unwrap(),
        connector_id: params[1].parse::<i64>().unwrap(),
    };

    // TODO: pass the message to the charger and return the response instead of this dummy response

    UnlockConnectorResponse {
        status: UnlockStatusEnumType::Unlocked,
        status_info: None,
    }
}

pub fn _handle_reset(params: Vec<String>) -> ResetResponse {
    let on_idle = params[0].parse::<bool>().unwrap();
    let request_type = if on_idle {
        ResetEnumType::OnIdle
    } else {
        ResetEnumType::Immediate
    };
    let _req = ResetRequest {
        evse_id: None,
        request_type,
    };

    // TODO pass the message to the charger and return the response instead of this dummy response

    ResetResponse {
        status: ResetStatusEnumType::Accepted,
        status_info: None,
    }
}
