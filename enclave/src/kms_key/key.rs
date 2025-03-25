use crate::common::EnclaveError;
use anyhow::Result;
use enclave_kmstool::{KmstoolGetKeyPolicyResult, KmstoolListKeyPoliciesResult};
use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer, Serialize};
use std::convert::TryFrom;

pub const DEFAULT_POLICY_NAME: &str = "default";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KmsKeyPolicies {
    #[serde(rename = "PolicyNames")]
    pub policy_names: Vec<String>,
    #[serde(rename = "Truncated")]
    pub truncated: bool,
}

impl TryFrom<KmstoolListKeyPoliciesResult> for KmsKeyPolicies {
    type Error = EnclaveError;

    fn try_from(result: KmstoolListKeyPoliciesResult) -> Result<Self, Self::Error> {
        let policies = serde_json::from_slice(&result.policies)?;
        Ok(policies)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KmsKeyPolicy {
    #[serde(rename = "PolicyName")]
    pub policy_name: String,
    #[serde(rename = "Policy", deserialize_with = "deserialize_policy_data")]
    pub policy: KmsKeyPolicyData,
}

fn deserialize_policy_data<'de, D>(deserializer: D) -> Result<KmsKeyPolicyData, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    serde_json::from_str(&s).map_err(SerdeError::custom)
}

impl TryFrom<KmstoolGetKeyPolicyResult> for KmsKeyPolicy {
    type Error = EnclaveError;

    fn try_from(result: KmstoolGetKeyPolicyResult) -> Result<Self, Self::Error> {
        let policy = serde_json::from_slice(&result.policy)?;
        Ok(policy)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KmsKeyPolicyData {
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Statement")]
    pub statement: Vec<Statement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Statement {
    #[serde(rename = "Sid")]
    pub sid: String,
    #[serde(rename = "Effect")]
    pub effect: String,
    #[serde(rename = "Principal")]
    pub principal: Principal,
    #[serde(rename = "Action")]
    pub action: Action,
    #[serde(rename = "Resource")]
    pub resource: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Condition")]
    pub condition: Option<Condition>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Condition {
    #[serde(rename = "StringEqualsIgnoreCase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_equals_ignore_case: Option<StringEqualsIgnoreCase>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StringEqualsIgnoreCase {
    #[serde(rename = "kms:RecipientAttestation:ImageSha384")]
    pub recipient_attestation_image_sha384: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Principal {
    #[serde(rename = "AWS")]
    pub aws: AwsPrincipal,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AwsPrincipal {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Action {
    Single(String),
    Multiple(Vec<String>),
}

impl KmsKeyPolicies {
    pub fn verify(&self) -> bool {
        if self.policy_names.len() != 1 {
            return false;
        }

        if self.policy_names[0] != DEFAULT_POLICY_NAME {
            return false;
        }

        if self.truncated {
            return false;
        }

        true
    }
}

impl KmsKeyPolicy {
    pub fn verify(&self, _enclave_pcr: String) -> bool {
        if self.policy_name != DEFAULT_POLICY_NAME {
            return false;
        }

        true
    }
}
