pub use crate::gen::vaultron::agent::v1::*;

use crate::vaultron::common::v1::{EnclaveAgentError, StatusCode};

impl StartEnclaveResponse {
    pub fn success() -> Self {
        Self::builder().build()
    }
}

impl From<StartEnclaveResponse> for EnclaveAgentResponse {
    fn from(response: StartEnclaveResponse) -> Self {
        EnclaveAgentResponse::builder()
            .code(StatusCode::success())
            .response(enclave_agent_response::Response::StartResponse(response))
            .build()
    }
}

impl StopEnclaveResponse {
    pub fn success() -> Self {
        Self::builder().build()
    }
}

impl From<StopEnclaveResponse> for EnclaveAgentResponse {
    fn from(response: StopEnclaveResponse) -> Self {
        EnclaveAgentResponse::builder()
            .code(StatusCode::success())
            .response(enclave_agent_response::Response::StopResponse(response))
            .build()
    }
}

impl RestartEnclaveResponse {
    pub fn success() -> Self {
        Self::builder().build()
    }
}

impl From<RestartEnclaveResponse> for EnclaveAgentResponse {
    fn from(response: RestartEnclaveResponse) -> Self {
        EnclaveAgentResponse::builder()
            .code(StatusCode::success())
            .response(enclave_agent_response::Response::RestartResponse(response))
            .build()
    }
}

impl DescribeEnclaveResponse {
    pub fn success(info: Option<DescribeEnclaveInfo>) -> Self {
        Self::builder().info(info).build()
    }
}

impl From<DescribeEnclaveResponse> for EnclaveAgentResponse {
    fn from(response: DescribeEnclaveResponse) -> Self {
        EnclaveAgentResponse::builder()
            .code(StatusCode::success())
            .response(enclave_agent_response::Response::DescribeResponse(response))
            .build()
    }
}

impl EnclaveAgentResponse {
    pub fn error<E>(err: E) -> Self
    where
        E: Into<EnclaveAgentError>,
    {
        Self::builder().code(StatusCode::enclave_agent_error(err)).build()
    }

    pub fn unknown_error<T>(error_message: T) -> Self
    where
        T: ToString,
    {
        Self::builder()
            .code(StatusCode::enclave_unknown_error(error_message))
            .build()
    }
}
