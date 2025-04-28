// @generated
impl serde::Serialize for CoordinatorError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "Coordinator_ERROR_UNSPECIFIED",
            Self::UnknownError => "Coordinator_ERROR_UNKNOWN_ERROR",
            Self::InvalidRequestError => "Coordinator_ERROR_INVALID_REQUEST_ERROR",
            Self::NoWorkerAvailableError => "COORDINATOR_ERROR_NO_WORKER_AVAILABLE_ERROR",
            Self::NotReadyError => "COORDINATOR_ERROR_NOT_READY_ERROR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for CoordinatorError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "Coordinator_ERROR_UNSPECIFIED",
            "Coordinator_ERROR_UNKNOWN_ERROR",
            "Coordinator_ERROR_INVALID_REQUEST_ERROR",
            "COORDINATOR_ERROR_NO_WORKER_AVAILABLE_ERROR",
            "COORDINATOR_ERROR_NOT_READY_ERROR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CoordinatorError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(CoordinatorError::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(CoordinatorError::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "Coordinator_ERROR_UNSPECIFIED" => Ok(CoordinatorError::Unspecified),
                    "Coordinator_ERROR_UNKNOWN_ERROR" => Ok(CoordinatorError::UnknownError),
                    "Coordinator_ERROR_INVALID_REQUEST_ERROR" => Ok(CoordinatorError::InvalidRequestError),
                    "COORDINATOR_ERROR_NO_WORKER_AVAILABLE_ERROR" => Ok(CoordinatorError::NoWorkerAvailableError),
                    "COORDINATOR_ERROR_NOT_READY_ERROR" => Ok(CoordinatorError::NotReadyError),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for EnclaveAgentError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ENCLAVE_AGENT_ERROR_UNSPECIFIED",
            Self::UnknownError => "ENCLAVE_AGENT_ERROR_UNKNOWN_ERROR",
            Self::InvalidRequestError => "ENCLAVE_AGENT_ERROR_INVALID_REQUEST_ERROR",
            Self::StartEnclaveError => "ENCLAVE_AGENT_ERROR_START_ENCLAVE_ERROR",
            Self::StopEnclaveError => "ENCLAVE_AGENT_ERROR_STOP_ENCLAVE_ERROR",
            Self::DescribeEnclaveError => "ENCLAVE_AGENT_ERROR_DESCRIBE_ENCLAVE_ERROR",
            Self::IoError => "ENCLAVE_AGENT_ERROR_IO_ERROR",
            Self::SerdeJsonError => "ENCLAVE_AGENT_ERROR_SERDE_JSON_ERROR",
            Self::ResponseDecodeError => "ENCLAVE_AGENT_ERROR_RESPONSE_DECODE_ERROR",
            Self::VsockClientError => "ENCLAVE_AGENT_ERROR_VSOCK_CLIENT_ERROR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for EnclaveAgentError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ENCLAVE_AGENT_ERROR_UNSPECIFIED",
            "ENCLAVE_AGENT_ERROR_UNKNOWN_ERROR",
            "ENCLAVE_AGENT_ERROR_INVALID_REQUEST_ERROR",
            "ENCLAVE_AGENT_ERROR_START_ENCLAVE_ERROR",
            "ENCLAVE_AGENT_ERROR_STOP_ENCLAVE_ERROR",
            "ENCLAVE_AGENT_ERROR_DESCRIBE_ENCLAVE_ERROR",
            "ENCLAVE_AGENT_ERROR_IO_ERROR",
            "ENCLAVE_AGENT_ERROR_SERDE_JSON_ERROR",
            "ENCLAVE_AGENT_ERROR_RESPONSE_DECODE_ERROR",
            "ENCLAVE_AGENT_ERROR_VSOCK_CLIENT_ERROR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EnclaveAgentError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(EnclaveAgentError::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(EnclaveAgentError::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ENCLAVE_AGENT_ERROR_UNSPECIFIED" => Ok(EnclaveAgentError::Unspecified),
                    "ENCLAVE_AGENT_ERROR_UNKNOWN_ERROR" => Ok(EnclaveAgentError::UnknownError),
                    "ENCLAVE_AGENT_ERROR_INVALID_REQUEST_ERROR" => Ok(EnclaveAgentError::InvalidRequestError),
                    "ENCLAVE_AGENT_ERROR_START_ENCLAVE_ERROR" => Ok(EnclaveAgentError::StartEnclaveError),
                    "ENCLAVE_AGENT_ERROR_STOP_ENCLAVE_ERROR" => Ok(EnclaveAgentError::StopEnclaveError),
                    "ENCLAVE_AGENT_ERROR_DESCRIBE_ENCLAVE_ERROR" => Ok(EnclaveAgentError::DescribeEnclaveError),
                    "ENCLAVE_AGENT_ERROR_IO_ERROR" => Ok(EnclaveAgentError::IoError),
                    "ENCLAVE_AGENT_ERROR_SERDE_JSON_ERROR" => Ok(EnclaveAgentError::SerdeJsonError),
                    "ENCLAVE_AGENT_ERROR_RESPONSE_DECODE_ERROR" => Ok(EnclaveAgentError::ResponseDecodeError),
                    "ENCLAVE_AGENT_ERROR_VSOCK_CLIENT_ERROR" => Ok(EnclaveAgentError::VsockClientError),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for EnclaveError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ENCLAVE_ERROR_UNSPECIFIED",
            Self::UnknownError => "ENCLAVE_ERROR_UNKNOWN_ERROR",
            Self::ClusterNotInitializedError => "ENCLAVE_ERROR_CLUSTER_NOT_INITIALIZED_ERROR",
            Self::ClusterAlreadyInitializedError => "ENCLAVE_ERROR_CLUSTER_ALREADY_INITIALIZED_ERROR",
            Self::SeedCannotProcessRequestError => "ENCLAVE_ERROR_SEED_CANNOT_PROCESS_REQUEST_ERROR",
            Self::InvalidRequestError => "ENCLAVE_ERROR_INVALID_REQUEST_ERROR",
            Self::InvalidSignatureError => "ENCLAVE_ERROR_INVALID_SIGNATURE_ERROR",
            Self::InvalidAccountError => "ENCLAVE_ERROR_INVALID_ACCOUNT_ERROR",
            Self::WalletGenerationError => "ENCLAVE_ERROR_WALLET_GENERATION_ERROR",
            Self::InvalidParameterError => "ENCLAVE_ERROR_INVALID_PARAMETER_ERROR",
            Self::InvalidAttestationDocumentError => "ENCLAVE_ERROR_INVALID_ATTESTATION_DOCUMENT_ERROR",
            Self::EnclaveCryptoError => "ENCLAVE_ERROR_ENCLAVE_CRYPTO_ERROR",
            Self::InvalidClusterPublicKeyError => "ENCLAVE_ERROR_INVALID_CLUSTER_PUBLIC_KEY_ERROR",
            Self::ProtobufEncodeError => "ENCLAVE_ERROR_PROTOBUF_ENCODE_ERROR",
            Self::ProtobufDecodeError => "ENCLAVE_ERROR_PROTOBUF_DECODE_ERROR",
            Self::ResponseProtobufDecodeError => "ENCLAVE_ERROR_RESPONSE_PROTOBUF_DECODE_ERROR",
            Self::PostcardError => "ENCLAVE_ERROR_POSTCARD_ERROR",
            Self::LogError => "ENCLAVE_ERROR_LOG_ERROR",
            Self::IoError => "ENCLAVE_ERROR_IO_ERROR",
            Self::AnyhowError => "ENCLAVE_ERROR_ANYHOW_ERROR",
            Self::SerdeJsonError => "ENCLAVE_ERROR_SERDE_JSON_ERROR",
            Self::NsmApiError => "ENCLAVE_ERROR_NSM_API_ERROR",
            Self::AttestationError => "ENCLAVE_ERROR_ATTESTATION_ERROR",
            Self::VsockClientError => "ENCLAVE_ERROR_VSOCK_CLIENT_ERROR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for EnclaveError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ENCLAVE_ERROR_UNSPECIFIED",
            "ENCLAVE_ERROR_UNKNOWN_ERROR",
            "ENCLAVE_ERROR_CLUSTER_NOT_INITIALIZED_ERROR",
            "ENCLAVE_ERROR_CLUSTER_ALREADY_INITIALIZED_ERROR",
            "ENCLAVE_ERROR_SEED_CANNOT_PROCESS_REQUEST_ERROR",
            "ENCLAVE_ERROR_INVALID_REQUEST_ERROR",
            "ENCLAVE_ERROR_INVALID_SIGNATURE_ERROR",
            "ENCLAVE_ERROR_INVALID_ACCOUNT_ERROR",
            "ENCLAVE_ERROR_WALLET_GENERATION_ERROR",
            "ENCLAVE_ERROR_INVALID_PARAMETER_ERROR",
            "ENCLAVE_ERROR_INVALID_ATTESTATION_DOCUMENT_ERROR",
            "ENCLAVE_ERROR_ENCLAVE_CRYPTO_ERROR",
            "ENCLAVE_ERROR_INVALID_CLUSTER_PUBLIC_KEY_ERROR",
            "ENCLAVE_ERROR_PROTOBUF_ENCODE_ERROR",
            "ENCLAVE_ERROR_PROTOBUF_DECODE_ERROR",
            "ENCLAVE_ERROR_RESPONSE_PROTOBUF_DECODE_ERROR",
            "ENCLAVE_ERROR_POSTCARD_ERROR",
            "ENCLAVE_ERROR_LOG_ERROR",
            "ENCLAVE_ERROR_IO_ERROR",
            "ENCLAVE_ERROR_ANYHOW_ERROR",
            "ENCLAVE_ERROR_SERDE_JSON_ERROR",
            "ENCLAVE_ERROR_NSM_API_ERROR",
            "ENCLAVE_ERROR_ATTESTATION_ERROR",
            "ENCLAVE_ERROR_VSOCK_CLIENT_ERROR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EnclaveError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(EnclaveError::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(EnclaveError::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ENCLAVE_ERROR_UNSPECIFIED" => Ok(EnclaveError::Unspecified),
                    "ENCLAVE_ERROR_UNKNOWN_ERROR" => Ok(EnclaveError::UnknownError),
                    "ENCLAVE_ERROR_CLUSTER_NOT_INITIALIZED_ERROR" => Ok(EnclaveError::ClusterNotInitializedError),
                    "ENCLAVE_ERROR_CLUSTER_ALREADY_INITIALIZED_ERROR" => Ok(EnclaveError::ClusterAlreadyInitializedError),
                    "ENCLAVE_ERROR_SEED_CANNOT_PROCESS_REQUEST_ERROR" => Ok(EnclaveError::SeedCannotProcessRequestError),
                    "ENCLAVE_ERROR_INVALID_REQUEST_ERROR" => Ok(EnclaveError::InvalidRequestError),
                    "ENCLAVE_ERROR_INVALID_SIGNATURE_ERROR" => Ok(EnclaveError::InvalidSignatureError),
                    "ENCLAVE_ERROR_INVALID_ACCOUNT_ERROR" => Ok(EnclaveError::InvalidAccountError),
                    "ENCLAVE_ERROR_WALLET_GENERATION_ERROR" => Ok(EnclaveError::WalletGenerationError),
                    "ENCLAVE_ERROR_INVALID_PARAMETER_ERROR" => Ok(EnclaveError::InvalidParameterError),
                    "ENCLAVE_ERROR_INVALID_ATTESTATION_DOCUMENT_ERROR" => Ok(EnclaveError::InvalidAttestationDocumentError),
                    "ENCLAVE_ERROR_ENCLAVE_CRYPTO_ERROR" => Ok(EnclaveError::EnclaveCryptoError),
                    "ENCLAVE_ERROR_INVALID_CLUSTER_PUBLIC_KEY_ERROR" => Ok(EnclaveError::InvalidClusterPublicKeyError),
                    "ENCLAVE_ERROR_PROTOBUF_ENCODE_ERROR" => Ok(EnclaveError::ProtobufEncodeError),
                    "ENCLAVE_ERROR_PROTOBUF_DECODE_ERROR" => Ok(EnclaveError::ProtobufDecodeError),
                    "ENCLAVE_ERROR_RESPONSE_PROTOBUF_DECODE_ERROR" => Ok(EnclaveError::ResponseProtobufDecodeError),
                    "ENCLAVE_ERROR_POSTCARD_ERROR" => Ok(EnclaveError::PostcardError),
                    "ENCLAVE_ERROR_LOG_ERROR" => Ok(EnclaveError::LogError),
                    "ENCLAVE_ERROR_IO_ERROR" => Ok(EnclaveError::IoError),
                    "ENCLAVE_ERROR_ANYHOW_ERROR" => Ok(EnclaveError::AnyhowError),
                    "ENCLAVE_ERROR_SERDE_JSON_ERROR" => Ok(EnclaveError::SerdeJsonError),
                    "ENCLAVE_ERROR_NSM_API_ERROR" => Ok(EnclaveError::NsmApiError),
                    "ENCLAVE_ERROR_ATTESTATION_ERROR" => Ok(EnclaveError::AttestationError),
                    "ENCLAVE_ERROR_VSOCK_CLIENT_ERROR" => Ok(EnclaveError::VsockClientError),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for StatusCode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.success {
            len += 1;
        }
        if self.error_message.is_some() {
            len += 1;
        }
        if self.error.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.common.v1.StatusCode", len)?;
        if self.success {
            struct_ser.serialize_field("success", &self.success)?;
        }
        if let Some(v) = self.error_message.as_ref() {
            struct_ser.serialize_field("errorMessage", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            match v {
                status_code::Error::EnclaveError(v) => {
                    let v = EnclaveError::from_i32(*v)
                        .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("enclaveError", &v)?;
                }
                status_code::Error::EnclaveAgentError(v) => {
                    let v = EnclaveAgentError::from_i32(*v)
                        .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("enclaveAgentError", &v)?;
                }
                status_code::Error::CoordinatorError(v) => {
                    let v = CoordinatorError::from_i32(*v)
                        .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("coordinatorError", &v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StatusCode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "success",
            "error_message",
            "errorMessage",
            "enclave_error",
            "enclaveError",
            "enclave_agent_error",
            "enclaveAgentError",
            "coordinator_error",
            "coordinatorError",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Success,
            ErrorMessage,
            EnclaveError,
            EnclaveAgentError,
            CoordinatorError,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "success" => Ok(GeneratedField::Success),
                            "errorMessage" | "error_message" => Ok(GeneratedField::ErrorMessage),
                            "enclaveError" | "enclave_error" => Ok(GeneratedField::EnclaveError),
                            "enclaveAgentError" | "enclave_agent_error" => Ok(GeneratedField::EnclaveAgentError),
                            "coordinatorError" | "coordinator_error" => Ok(GeneratedField::CoordinatorError),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StatusCode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.common.v1.StatusCode")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StatusCode, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut success__ = None;
                let mut error_message__ = None;
                let mut error__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Success => {
                            if success__.is_some() {
                                return Err(serde::de::Error::duplicate_field("success"));
                            }
                            success__ = Some(map.next_value()?);
                        }
                        GeneratedField::ErrorMessage => {
                            if error_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorMessage"));
                            }
                            error_message__ = map.next_value()?;
                        }
                        GeneratedField::EnclaveError => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enclaveError"));
                            }
                            error__ = map.next_value::<::std::option::Option<EnclaveError>>()?.map(|x| status_code::Error::EnclaveError(x as i32));
                        }
                        GeneratedField::EnclaveAgentError => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enclaveAgentError"));
                            }
                            error__ = map.next_value::<::std::option::Option<EnclaveAgentError>>()?.map(|x| status_code::Error::EnclaveAgentError(x as i32));
                        }
                        GeneratedField::CoordinatorError => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("coordinatorError"));
                            }
                            error__ = map.next_value::<::std::option::Option<CoordinatorError>>()?.map(|x| status_code::Error::CoordinatorError(x as i32));
                        }
                    }
                }
                Ok(StatusCode {
                    success: success__.unwrap_or_default(),
                    error_message: error_message__,
                    error: error__,
                })
            }
        }
        deserializer.deserialize_struct("vaultron.common.v1.StatusCode", FIELDS, GeneratedVisitor)
    }
}
