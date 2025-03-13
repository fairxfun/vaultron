use anyhow::Result;
use enclave_agent::{create_enclave_agent, EnclaveAgentCreateOptions};
use enclave_protos::enclave::v1::{InitRequest, UpdateAwsCredentialsRequest};

use crate::EnclaveAgentTesterError;

pub async fn start_tester() -> Result<(), Box<dyn std::error::Error>> {
    let options = EnclaveAgentCreateOptions::builder().cid(1000).port(1000).build();
    let agent = create_enclave_agent(options)?;

    let (access_key_id, secret_access_key, session_token) = get_aws_credentials().await?;
    let request = InitRequest::builder()
        .enable_logging(false)
        .aws_region("ap-southeast-1".to_string())
        .aws_access_key_id(access_key_id)
        .aws_secret_access_key(secret_access_key)
        .aws_session_token(session_token)
        .kms_key_id("60b8ce3a-7466-42b7-96a7-a3868f0fd1bf".to_string())
        .build();
    let response = agent.init(request).await?;
    println!("Init response: {:?}", response);
    let request = UpdateAwsCredentialsRequest::builder().build();
    let response = agent.update_aws_credentials(request).await?;
    println!("Update aws credentials response: {:?}", response);

    Ok(())
}

async fn get_aws_credentials() -> Result<(String, String, String), EnclaveAgentTesterError> {
    let url = "http://169.254.169.254/latest/meta-data/iam/security-credentials/fairx_enclave_role";
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    let credentials: serde_json::Value = serde_json::from_str(&body)?;
    let access_key_id = credentials["AccessKeyId"]
        .as_str()
        .ok_or_else(|| EnclaveAgentTesterError::AwsCredentialRequestError("AccessKeyId not found".to_string()))?;
    let secret_access_key = credentials["SecretAccessKey"]
        .as_str()
        .ok_or_else(|| EnclaveAgentTesterError::AwsCredentialRequestError("SecretAccessKey not found".to_string()))?;
    let session_token = credentials["Token"]
        .as_str()
        .ok_or_else(|| EnclaveAgentTesterError::AwsCredentialRequestError("Token not found".to_string()))?;
    Ok((
        access_key_id.to_string(),
        secret_access_key.to_string(),
        session_token.to_string(),
    ))
}
