// @generated
impl serde::Serialize for EnclaveClusterRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.request.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.v1.EnclaveClusterRequest", len)?;
        if let Some(v) = self.request.as_ref() {
            match v {
                enclave_cluster_request::Request::CreateUserWalletRequest(v) => {
                    struct_ser.serialize_field("createUserWalletRequest", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EnclaveClusterRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "create_user_wallet_request",
            "createUserWalletRequest",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CreateUserWalletRequest,
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
                            "createUserWalletRequest" | "create_user_wallet_request" => Ok(GeneratedField::CreateUserWalletRequest),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EnclaveClusterRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.v1.EnclaveClusterRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EnclaveClusterRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CreateUserWalletRequest => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createUserWalletRequest"));
                            }
                            request__ = map.next_value::<::std::option::Option<_>>()?.map(enclave_cluster_request::Request::CreateUserWalletRequest)
;
                        }
                    }
                }
                Ok(EnclaveClusterRequest {
                    request: request__,
                })
            }
        }
        deserializer.deserialize_struct("vaultron.v1.EnclaveClusterRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EnclaveInternalRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.request.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.v1.EnclaveInternalRequest", len)?;
        if let Some(v) = self.request.as_ref() {
            match v {
                enclave_internal_request::Request::PingRequest(v) => {
                    struct_ser.serialize_field("pingRequest", v)?;
                }
                enclave_internal_request::Request::InitClusterKeyRequest(v) => {
                    struct_ser.serialize_field("initClusterKeyRequest", v)?;
                }
                enclave_internal_request::Request::InitClusterKeySyncRequest(v) => {
                    struct_ser.serialize_field("initClusterKeySyncRequest", v)?;
                }
                enclave_internal_request::Request::ForwardClusterKeySyncRequest(v) => {
                    struct_ser.serialize_field("forwardClusterKeySyncRequest", v)?;
                }
                enclave_internal_request::Request::ReplyClusterKeySyncRequest(v) => {
                    struct_ser.serialize_field("replyClusterKeySyncRequest", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EnclaveInternalRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ping_request",
            "pingRequest",
            "init_cluster_key_request",
            "initClusterKeyRequest",
            "init_cluster_key_sync_request",
            "initClusterKeySyncRequest",
            "forward_cluster_key_sync_request",
            "forwardClusterKeySyncRequest",
            "reply_cluster_key_sync_request",
            "replyClusterKeySyncRequest",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PingRequest,
            InitClusterKeyRequest,
            InitClusterKeySyncRequest,
            ForwardClusterKeySyncRequest,
            ReplyClusterKeySyncRequest,
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
                            "pingRequest" | "ping_request" => Ok(GeneratedField::PingRequest),
                            "initClusterKeyRequest" | "init_cluster_key_request" => Ok(GeneratedField::InitClusterKeyRequest),
                            "initClusterKeySyncRequest" | "init_cluster_key_sync_request" => Ok(GeneratedField::InitClusterKeySyncRequest),
                            "forwardClusterKeySyncRequest" | "forward_cluster_key_sync_request" => Ok(GeneratedField::ForwardClusterKeySyncRequest),
                            "replyClusterKeySyncRequest" | "reply_cluster_key_sync_request" => Ok(GeneratedField::ReplyClusterKeySyncRequest),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EnclaveInternalRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.v1.EnclaveInternalRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EnclaveInternalRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PingRequest => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pingRequest"));
                            }
                            request__ = map.next_value::<::std::option::Option<_>>()?.map(enclave_internal_request::Request::PingRequest)
;
                        }
                        GeneratedField::InitClusterKeyRequest => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initClusterKeyRequest"));
                            }
                            request__ = map.next_value::<::std::option::Option<_>>()?.map(enclave_internal_request::Request::InitClusterKeyRequest)
;
                        }
                        GeneratedField::InitClusterKeySyncRequest => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initClusterKeySyncRequest"));
                            }
                            request__ = map.next_value::<::std::option::Option<_>>()?.map(enclave_internal_request::Request::InitClusterKeySyncRequest)
;
                        }
                        GeneratedField::ForwardClusterKeySyncRequest => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("forwardClusterKeySyncRequest"));
                            }
                            request__ = map.next_value::<::std::option::Option<_>>()?.map(enclave_internal_request::Request::ForwardClusterKeySyncRequest)
;
                        }
                        GeneratedField::ReplyClusterKeySyncRequest => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("replyClusterKeySyncRequest"));
                            }
                            request__ = map.next_value::<::std::option::Option<_>>()?.map(enclave_internal_request::Request::ReplyClusterKeySyncRequest)
;
                        }
                    }
                }
                Ok(EnclaveInternalRequest {
                    request: request__,
                })
            }
        }
        deserializer.deserialize_struct("vaultron.v1.EnclaveInternalRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EnclaveProtoError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::EnclaveErrorUnspecified => "ENCLAVE_ERROR_UNSPECIFIED",
            Self::EnclaveErrorUnknownError => "ENCLAVE_ERROR_UNKNOWN_ERROR",
            Self::EnclaveErrorClusterNotInitialized => "ENCLAVE_ERROR_CLUSTER_NOT_INITIALIZED",
            Self::EnclaveErrorClusterAlreadyInitialized => "ENCLAVE_ERROR_CLUSTER_ALREADY_INITIALIZED",
            Self::EnclaveErrorInvalidRequestError => "ENCLAVE_ERROR_INVALID_REQUEST_ERROR",
            Self::EnclaveErrorInvalidSignatureError => "ENCLAVE_ERROR_INVALID_SIGNATURE_ERROR",
            Self::EnclaveErrorInvalidAccountError => "ENCLAVE_ERROR_INVALID_ACCOUNT_ERROR",
            Self::EnclaveErrorWalletGenerationError => "ENCLAVE_ERROR_WALLET_GENERATION_ERROR",
            Self::EnclaveErrorInvalidParameterError => "ENCLAVE_ERROR_INVALID_PARAMETER_ERROR",
            Self::EnclaveErrorInvalidAttestationDocumentError => "ENCLAVE_ERROR_INVALID_ATTESTATION_DOCUMENT_ERROR",
            Self::EnclaveErrorEnclaveCryptoError => "ENCLAVE_ERROR_ENCLAVE_CRYPTO_ERROR",
            Self::EnclaveErrorInvalidClusterPublicKeyError => "ENCLAVE_ERROR_INVALID_CLUSTER_PUBLIC_KEY_ERROR",
            Self::EnclaveErrorProtobufEncodeError => "ENCLAVE_ERROR_PROTOBUF_ENCODE_ERROR",
            Self::EnclaveErrorProtobufDecodeError => "ENCLAVE_ERROR_PROTOBUF_DECODE_ERROR",
            Self::EnclaveErrorPostcardError => "ENCLAVE_ERROR_POSTCARD_ERROR",
            Self::EnclaveErrorLogError => "ENCLAVE_ERROR_LOG_ERROR",
            Self::EnclaveErrorIoError => "ENCLAVE_ERROR_IO_ERROR",
            Self::EnclaveErrorAnyhowError => "ENCLAVE_ERROR_ANYHOW_ERROR",
            Self::EnclaveErrorSerdeJsonError => "ENCLAVE_ERROR_SERDE_JSON_ERROR",
            Self::EnclaveErrorNsmApiError => "ENCLAVE_ERROR_NSM_API_ERROR",
            Self::EnclaveErrorAttestationError => "ENCLAVE_ERROR_ATTESTATION_ERROR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for EnclaveProtoError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ENCLAVE_ERROR_UNSPECIFIED",
            "ENCLAVE_ERROR_UNKNOWN_ERROR",
            "ENCLAVE_ERROR_CLUSTER_NOT_INITIALIZED",
            "ENCLAVE_ERROR_CLUSTER_ALREADY_INITIALIZED",
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
            "ENCLAVE_ERROR_POSTCARD_ERROR",
            "ENCLAVE_ERROR_LOG_ERROR",
            "ENCLAVE_ERROR_IO_ERROR",
            "ENCLAVE_ERROR_ANYHOW_ERROR",
            "ENCLAVE_ERROR_SERDE_JSON_ERROR",
            "ENCLAVE_ERROR_NSM_API_ERROR",
            "ENCLAVE_ERROR_ATTESTATION_ERROR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EnclaveProtoError;

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
                    .and_then(EnclaveProtoError::from_i32)
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
                    .and_then(EnclaveProtoError::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ENCLAVE_ERROR_UNSPECIFIED" => Ok(EnclaveProtoError::EnclaveErrorUnspecified),
                    "ENCLAVE_ERROR_UNKNOWN_ERROR" => Ok(EnclaveProtoError::EnclaveErrorUnknownError),
                    "ENCLAVE_ERROR_CLUSTER_NOT_INITIALIZED" => Ok(EnclaveProtoError::EnclaveErrorClusterNotInitialized),
                    "ENCLAVE_ERROR_CLUSTER_ALREADY_INITIALIZED" => Ok(EnclaveProtoError::EnclaveErrorClusterAlreadyInitialized),
                    "ENCLAVE_ERROR_INVALID_REQUEST_ERROR" => Ok(EnclaveProtoError::EnclaveErrorInvalidRequestError),
                    "ENCLAVE_ERROR_INVALID_SIGNATURE_ERROR" => Ok(EnclaveProtoError::EnclaveErrorInvalidSignatureError),
                    "ENCLAVE_ERROR_INVALID_ACCOUNT_ERROR" => Ok(EnclaveProtoError::EnclaveErrorInvalidAccountError),
                    "ENCLAVE_ERROR_WALLET_GENERATION_ERROR" => Ok(EnclaveProtoError::EnclaveErrorWalletGenerationError),
                    "ENCLAVE_ERROR_INVALID_PARAMETER_ERROR" => Ok(EnclaveProtoError::EnclaveErrorInvalidParameterError),
                    "ENCLAVE_ERROR_INVALID_ATTESTATION_DOCUMENT_ERROR" => Ok(EnclaveProtoError::EnclaveErrorInvalidAttestationDocumentError),
                    "ENCLAVE_ERROR_ENCLAVE_CRYPTO_ERROR" => Ok(EnclaveProtoError::EnclaveErrorEnclaveCryptoError),
                    "ENCLAVE_ERROR_INVALID_CLUSTER_PUBLIC_KEY_ERROR" => Ok(EnclaveProtoError::EnclaveErrorInvalidClusterPublicKeyError),
                    "ENCLAVE_ERROR_PROTOBUF_ENCODE_ERROR" => Ok(EnclaveProtoError::EnclaveErrorProtobufEncodeError),
                    "ENCLAVE_ERROR_PROTOBUF_DECODE_ERROR" => Ok(EnclaveProtoError::EnclaveErrorProtobufDecodeError),
                    "ENCLAVE_ERROR_POSTCARD_ERROR" => Ok(EnclaveProtoError::EnclaveErrorPostcardError),
                    "ENCLAVE_ERROR_LOG_ERROR" => Ok(EnclaveProtoError::EnclaveErrorLogError),
                    "ENCLAVE_ERROR_IO_ERROR" => Ok(EnclaveProtoError::EnclaveErrorIoError),
                    "ENCLAVE_ERROR_ANYHOW_ERROR" => Ok(EnclaveProtoError::EnclaveErrorAnyhowError),
                    "ENCLAVE_ERROR_SERDE_JSON_ERROR" => Ok(EnclaveProtoError::EnclaveErrorSerdeJsonError),
                    "ENCLAVE_ERROR_NSM_API_ERROR" => Ok(EnclaveProtoError::EnclaveErrorNsmApiError),
                    "ENCLAVE_ERROR_ATTESTATION_ERROR" => Ok(EnclaveProtoError::EnclaveErrorAttestationError),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for EnclaveRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.request.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.v1.EnclaveRequest", len)?;
        if let Some(v) = self.request.as_ref() {
            match v {
                enclave_request::Request::InternalRequest(v) => {
                    struct_ser.serialize_field("internalRequest", v)?;
                }
                enclave_request::Request::ClusterRequest(v) => {
                    struct_ser.serialize_field("clusterRequest", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EnclaveRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "internal_request",
            "internalRequest",
            "cluster_request",
            "clusterRequest",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InternalRequest,
            ClusterRequest,
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
                            "internalRequest" | "internal_request" => Ok(GeneratedField::InternalRequest),
                            "clusterRequest" | "cluster_request" => Ok(GeneratedField::ClusterRequest),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EnclaveRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.v1.EnclaveRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EnclaveRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::InternalRequest => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("internalRequest"));
                            }
                            request__ = map.next_value::<::std::option::Option<_>>()?.map(enclave_request::Request::InternalRequest)
;
                        }
                        GeneratedField::ClusterRequest => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterRequest"));
                            }
                            request__ = map.next_value::<::std::option::Option<_>>()?.map(enclave_request::Request::ClusterRequest)
;
                        }
                    }
                }
                Ok(EnclaveRequest {
                    request: request__,
                })
            }
        }
        deserializer.deserialize_struct("vaultron.v1.EnclaveRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EnclaveResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code.is_some() {
            len += 1;
        }
        if !self.attestation_document.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.v1.EnclaveResponse", len)?;
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        if !self.attestation_document.is_empty() {
            struct_ser.serialize_field("attestationDocument", pbjson::private::base64::encode(&self.attestation_document).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EnclaveResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "attestation_document",
            "attestationDocument",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            AttestationDocument,
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
                            "code" => Ok(GeneratedField::Code),
                            "attestationDocument" | "attestation_document" => Ok(GeneratedField::AttestationDocument),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EnclaveResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.v1.EnclaveResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EnclaveResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut attestation_document__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = map.next_value()?;
                        }
                        GeneratedField::AttestationDocument => {
                            if attestation_document__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attestationDocument"));
                            }
                            attestation_document__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(EnclaveResponse {
                    code: code__,
                    attestation_document: attestation_document__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vaultron.v1.EnclaveResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EnclaveResponsePadding {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("vaultron.v1.EnclaveResponsePadding", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EnclaveResponsePadding {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EnclaveResponsePadding;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.v1.EnclaveResponsePadding")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EnclaveResponsePadding, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(EnclaveResponsePadding {
                })
            }
        }
        deserializer.deserialize_struct("vaultron.v1.EnclaveResponsePadding", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("vaultron.v1.StatusCode", len)?;
        if self.success {
            struct_ser.serialize_field("success", &self.success)?;
        }
        if let Some(v) = self.error_message.as_ref() {
            struct_ser.serialize_field("errorMessage", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            match v {
                status_code::Error::Enclave(v) => {
                    let v = EnclaveProtoError::from_i32(*v)
                        .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("enclave", &v)?;
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
            "enclave",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Success,
            ErrorMessage,
            Enclave,
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
                            "enclave" => Ok(GeneratedField::Enclave),
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
                formatter.write_str("struct vaultron.v1.StatusCode")
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
                        GeneratedField::Enclave => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enclave"));
                            }
                            error__ = map.next_value::<::std::option::Option<EnclaveProtoError>>()?.map(|x| status_code::Error::Enclave(x as i32));
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
        deserializer.deserialize_struct("vaultron.v1.StatusCode", FIELDS, GeneratedVisitor)
    }
}
