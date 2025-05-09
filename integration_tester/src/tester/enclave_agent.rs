use crate::{error::EnclaveTesterError, MessageHandler};
use anyhow::Result;
use enclave_agent::EnclaveDescribeStatus;
use enclave_protos::vaultron::agent::v1::{
    enclave_agent_request, enclave_agent_response, DescribeEnclaveRequest, EnclaveAgentRequest, StartEnclaveRequest,
};
use log::{error, info};
use std::{str::FromStr, time::Duration};
use tokio::time::sleep;
pub async fn enclave_agent_test(handler: &mut MessageHandler) -> Result<(), EnclaveTesterError> {
    test_start_enclave(handler).await?;
    sleep(Duration::from_secs(5)).await;
    Ok(())
}

async fn test_start_enclave(handler: &mut MessageHandler) -> Result<(), EnclaveTesterError> {
    info!("test start enclave");
    let start_enclave_request = StartEnclaveRequest::builder().build();
    let enclave_agent_request = EnclaveAgentRequest::builder()
        .request(enclave_agent_request::Request::StartRequest(start_enclave_request))
        .build();
    let response = handler.send_enclave_agent_request(enclave_agent_request).await?;
    if !matches!(
        response.response,
        Some(enclave_agent_response::Response::StartResponse(_))
    ) {
        error!("start enclave response: {:?}", response);
        return Err(EnclaveTesterError::InvalidResponseError);
    }

    check_enclave_status(handler, Some(EnclaveDescribeStatus::Running)).await?;
    Ok(())
}

// async fn test_restart_enclave(handler: &mut MessageHandler) -> Result<(), EnclaveTesterError> {
//     info!("test restart enclave");
//     let restart_enclave_request = RestartEnclaveRequest::builder().build();
//     let enclave_agent_request = EnclaveAgentRequest::builder()
//         .request(enclave_agent_request::Request::RestartRequest(restart_enclave_request))
//         .build();
//     let response = handler.send_enclave_agent_request(enclave_agent_request).await?;
//     if !matches!(
//         response.response,
//         Some(enclave_agent_response::Response::RestartResponse(_))
//     ) {
//         error!("restart enclave response: {:?}", response);
//         return Err(EnclaveTesterError::InvalidResponseError);
//     }

//     check_enclave_status(handler, Some(EnclaveDescribeStatus::Running)).await?;
//     Ok(())
// }

// async fn test_stop_enclave(handler: &mut MessageHandler) -> Result<(), EnclaveTesterError> {
//     info!("test stop enclave");
//     let stop_enclave_request = StopEnclaveRequest::builder().build();
//     let enclave_agent_request = EnclaveAgentRequest::builder()
//         .request(enclave_agent_request::Request::StopRequest(stop_enclave_request))
//         .build();
//     let response = handler.send_enclave_agent_request(enclave_agent_request).await?;
//     if !matches!(
//         response.response,
//         Some(enclave_agent_response::Response::StopResponse(_))
//     ) {
//         error!("stop enclave response: {:?}", response);
//         return Err(EnclaveTesterError::InvalidResponseError);
//     }
//     sleep(Duration::from_secs(2)).await;
//     check_enclave_status(handler, None).await?;
//     Ok(())
// }

async fn check_enclave_status(
    handler: &mut MessageHandler,
    expected_status: Option<EnclaveDescribeStatus>,
) -> Result<(), EnclaveTesterError> {
    let describe_enclave_request = DescribeEnclaveRequest::builder().build();
    let enclave_agent_request = EnclaveAgentRequest::builder()
        .request(enclave_agent_request::Request::DescribeRequest(
            describe_enclave_request,
        ))
        .build();
    let response = handler.send_enclave_agent_request(enclave_agent_request).await?;
    let info = match response.response {
        Some(enclave_agent_response::Response::DescribeResponse(describe_response)) => describe_response.info,
        _ => return Err(EnclaveTesterError::InvalidResponseError),
    };
    match expected_status {
        Some(expected_status) => {
            let state = EnclaveDescribeStatus::from_str(&info.unwrap().state).expect("Invalid enclave state");
            if state == expected_status {
                Ok(())
            } else {
                error!("enclave state: {:?}", state);
                Err(EnclaveTesterError::InvalidResponseError)
            }
        }
        None => {
            if info.is_some() {
                error!("enclave state: {:?}", info);
                Err(EnclaveTesterError::InvalidResponseError)
            } else {
                Ok(())
            }
        }
    }
}
