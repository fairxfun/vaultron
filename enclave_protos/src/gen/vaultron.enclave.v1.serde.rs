// @generated
impl serde::Serialize for CreateEnclaveWalletRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_id.is_empty() {
            len += 1;
        }
        if !self.user_public_key.is_empty() {
            len += 1;
        }
        if self.signature_type != 0 {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        if !self.signature.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.v1.CreateEnclaveWalletRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", pbjson::private::base64::encode(&self.user_id).as_str())?;
        }
        if !self.user_public_key.is_empty() {
            struct_ser.serialize_field("userPublicKey", pbjson::private::base64::encode(&self.user_public_key).as_str())?;
        }
        if self.signature_type != 0 {
            let v = SignatureType::from_i32(self.signature_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.signature_type)))?;
            struct_ser.serialize_field("signatureType", &v)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", pbjson::private::base64::encode(&self.message).as_str())?;
        }
        if !self.signature.is_empty() {
            struct_ser.serialize_field("signature", pbjson::private::base64::encode(&self.signature).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateEnclaveWalletRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "user_public_key",
            "userPublicKey",
            "signature_type",
            "signatureType",
            "message",
            "signature",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            UserPublicKey,
            SignatureType,
            Message,
            Signature,
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
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "userPublicKey" | "user_public_key" => Ok(GeneratedField::UserPublicKey),
                            "signatureType" | "signature_type" => Ok(GeneratedField::SignatureType),
                            "message" => Ok(GeneratedField::Message),
                            "signature" => Ok(GeneratedField::Signature),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateEnclaveWalletRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.enclave.v1.CreateEnclaveWalletRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CreateEnclaveWalletRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut user_public_key__ = None;
                let mut signature_type__ = None;
                let mut message__ = None;
                let mut signature__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::UserPublicKey => {
                            if user_public_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userPublicKey"));
                            }
                            user_public_key__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SignatureType => {
                            if signature_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signatureType"));
                            }
                            signature_type__ = Some(map.next_value::<SignatureType>()? as i32);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Signature => {
                            if signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signature"));
                            }
                            signature__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(CreateEnclaveWalletRequest {
                    user_id: user_id__.unwrap_or_default(),
                    user_public_key: user_public_key__.unwrap_or_default(),
                    signature_type: signature_type__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                    signature: signature__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vaultron.enclave.v1.CreateEnclaveWalletRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateEnclaveWalletResponse {
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
        if !self.wallet_encrypted_data.is_empty() {
            len += 1;
        }
        if !self.eth_public_key.is_empty() {
            len += 1;
        }
        if !self.eth_encrypted_data.is_empty() {
            len += 1;
        }
        if !self.solana_public_key.is_empty() {
            len += 1;
        }
        if !self.solana_encrypted_data.is_empty() {
            len += 1;
        }
        if !self.sui_public_key.is_empty() {
            len += 1;
        }
        if !self.sui_encrypted_data.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.v1.CreateEnclaveWalletResponse", len)?;
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        if !self.wallet_encrypted_data.is_empty() {
            struct_ser.serialize_field("walletEncryptedData", pbjson::private::base64::encode(&self.wallet_encrypted_data).as_str())?;
        }
        if !self.eth_public_key.is_empty() {
            struct_ser.serialize_field("ethPublicKey", pbjson::private::base64::encode(&self.eth_public_key).as_str())?;
        }
        if !self.eth_encrypted_data.is_empty() {
            struct_ser.serialize_field("ethEncryptedData", pbjson::private::base64::encode(&self.eth_encrypted_data).as_str())?;
        }
        if !self.solana_public_key.is_empty() {
            struct_ser.serialize_field("solanaPublicKey", pbjson::private::base64::encode(&self.solana_public_key).as_str())?;
        }
        if !self.solana_encrypted_data.is_empty() {
            struct_ser.serialize_field("solanaEncryptedData", pbjson::private::base64::encode(&self.solana_encrypted_data).as_str())?;
        }
        if !self.sui_public_key.is_empty() {
            struct_ser.serialize_field("suiPublicKey", pbjson::private::base64::encode(&self.sui_public_key).as_str())?;
        }
        if !self.sui_encrypted_data.is_empty() {
            struct_ser.serialize_field("suiEncryptedData", pbjson::private::base64::encode(&self.sui_encrypted_data).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateEnclaveWalletResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "wallet_encrypted_data",
            "walletEncryptedData",
            "eth_public_key",
            "ethPublicKey",
            "eth_encrypted_data",
            "ethEncryptedData",
            "solana_public_key",
            "solanaPublicKey",
            "solana_encrypted_data",
            "solanaEncryptedData",
            "sui_public_key",
            "suiPublicKey",
            "sui_encrypted_data",
            "suiEncryptedData",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            WalletEncryptedData,
            EthPublicKey,
            EthEncryptedData,
            SolanaPublicKey,
            SolanaEncryptedData,
            SuiPublicKey,
            SuiEncryptedData,
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
                            "walletEncryptedData" | "wallet_encrypted_data" => Ok(GeneratedField::WalletEncryptedData),
                            "ethPublicKey" | "eth_public_key" => Ok(GeneratedField::EthPublicKey),
                            "ethEncryptedData" | "eth_encrypted_data" => Ok(GeneratedField::EthEncryptedData),
                            "solanaPublicKey" | "solana_public_key" => Ok(GeneratedField::SolanaPublicKey),
                            "solanaEncryptedData" | "solana_encrypted_data" => Ok(GeneratedField::SolanaEncryptedData),
                            "suiPublicKey" | "sui_public_key" => Ok(GeneratedField::SuiPublicKey),
                            "suiEncryptedData" | "sui_encrypted_data" => Ok(GeneratedField::SuiEncryptedData),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateEnclaveWalletResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.enclave.v1.CreateEnclaveWalletResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CreateEnclaveWalletResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut wallet_encrypted_data__ = None;
                let mut eth_public_key__ = None;
                let mut eth_encrypted_data__ = None;
                let mut solana_public_key__ = None;
                let mut solana_encrypted_data__ = None;
                let mut sui_public_key__ = None;
                let mut sui_encrypted_data__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = map.next_value()?;
                        }
                        GeneratedField::WalletEncryptedData => {
                            if wallet_encrypted_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("walletEncryptedData"));
                            }
                            wallet_encrypted_data__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EthPublicKey => {
                            if eth_public_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ethPublicKey"));
                            }
                            eth_public_key__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EthEncryptedData => {
                            if eth_encrypted_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ethEncryptedData"));
                            }
                            eth_encrypted_data__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SolanaPublicKey => {
                            if solana_public_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("solanaPublicKey"));
                            }
                            solana_public_key__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SolanaEncryptedData => {
                            if solana_encrypted_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("solanaEncryptedData"));
                            }
                            solana_encrypted_data__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SuiPublicKey => {
                            if sui_public_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("suiPublicKey"));
                            }
                            sui_public_key__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SuiEncryptedData => {
                            if sui_encrypted_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("suiEncryptedData"));
                            }
                            sui_encrypted_data__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(CreateEnclaveWalletResponse {
                    code: code__,
                    wallet_encrypted_data: wallet_encrypted_data__.unwrap_or_default(),
                    eth_public_key: eth_public_key__.unwrap_or_default(),
                    eth_encrypted_data: eth_encrypted_data__.unwrap_or_default(),
                    solana_public_key: solana_public_key__.unwrap_or_default(),
                    solana_encrypted_data: solana_encrypted_data__.unwrap_or_default(),
                    sui_public_key: sui_public_key__.unwrap_or_default(),
                    sui_encrypted_data: sui_encrypted_data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vaultron.enclave.v1.CreateEnclaveWalletResponse", FIELDS, GeneratedVisitor)
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
            Self::KmsDecryptError => "ENCLAVE_ERROR_KMS_DECRYPT_ERROR",
            Self::InvalidRequestError => "ENCLAVE_ERROR_INVALID_REQUEST_ERROR",
            Self::InvalidSignatureError => "ENCLAVE_ERROR_INVALID_SIGNATURE_ERROR",
            Self::InvalidAccountError => "ENCLAVE_ERROR_INVALID_ACCOUNT_ERROR",
            Self::WalletGenerationError => "ENCLAVE_ERROR_WALLET_GENERATION_ERROR",
            Self::InvalidParameterError => "ENCLAVE_ERROR_INVALID_PARAMETER_ERROR",
            Self::InvalidKmsEncryptedDataError => "ENCLAVE_ERROR_INVALID_KMS_ENCRYPTED_DATA_ERROR",
            Self::ProtobufEncodeError => "ENCLAVE_ERROR_PROTOBUF_ENCODE_ERROR",
            Self::ProtobufDecodeError => "ENCLAVE_ERROR_PROTOBUF_DECODE_ERROR",
            Self::EnclaveKmstoolError => "ENCLAVE_ERROR_ENCLAVE_KMSTOOL_ERROR",
            Self::EnclaveWalletError => "ENCLAVE_ERROR_ENCLAVE_WALLET_ERROR",
            Self::PostcardError => "ENCLAVE_ERROR_POSTCARD_ERROR",
            Self::LogError => "ENCLAVE_ERROR_LOG_ERROR",
            Self::IoError => "ENCLAVE_ERROR_IO_ERROR",
            Self::AnyhowError => "ENCLAVE_ERROR_ANYHOW_ERROR",
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
            "ENCLAVE_ERROR_KMS_DECRYPT_ERROR",
            "ENCLAVE_ERROR_INVALID_REQUEST_ERROR",
            "ENCLAVE_ERROR_INVALID_SIGNATURE_ERROR",
            "ENCLAVE_ERROR_INVALID_ACCOUNT_ERROR",
            "ENCLAVE_ERROR_WALLET_GENERATION_ERROR",
            "ENCLAVE_ERROR_INVALID_PARAMETER_ERROR",
            "ENCLAVE_ERROR_INVALID_KMS_ENCRYPTED_DATA_ERROR",
            "ENCLAVE_ERROR_PROTOBUF_ENCODE_ERROR",
            "ENCLAVE_ERROR_PROTOBUF_DECODE_ERROR",
            "ENCLAVE_ERROR_ENCLAVE_KMSTOOL_ERROR",
            "ENCLAVE_ERROR_ENCLAVE_WALLET_ERROR",
            "ENCLAVE_ERROR_POSTCARD_ERROR",
            "ENCLAVE_ERROR_LOG_ERROR",
            "ENCLAVE_ERROR_IO_ERROR",
            "ENCLAVE_ERROR_ANYHOW_ERROR",
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
                    "ENCLAVE_ERROR_KMS_DECRYPT_ERROR" => Ok(EnclaveError::KmsDecryptError),
                    "ENCLAVE_ERROR_INVALID_REQUEST_ERROR" => Ok(EnclaveError::InvalidRequestError),
                    "ENCLAVE_ERROR_INVALID_SIGNATURE_ERROR" => Ok(EnclaveError::InvalidSignatureError),
                    "ENCLAVE_ERROR_INVALID_ACCOUNT_ERROR" => Ok(EnclaveError::InvalidAccountError),
                    "ENCLAVE_ERROR_WALLET_GENERATION_ERROR" => Ok(EnclaveError::WalletGenerationError),
                    "ENCLAVE_ERROR_INVALID_PARAMETER_ERROR" => Ok(EnclaveError::InvalidParameterError),
                    "ENCLAVE_ERROR_INVALID_KMS_ENCRYPTED_DATA_ERROR" => Ok(EnclaveError::InvalidKmsEncryptedDataError),
                    "ENCLAVE_ERROR_PROTOBUF_ENCODE_ERROR" => Ok(EnclaveError::ProtobufEncodeError),
                    "ENCLAVE_ERROR_PROTOBUF_DECODE_ERROR" => Ok(EnclaveError::ProtobufDecodeError),
                    "ENCLAVE_ERROR_ENCLAVE_KMSTOOL_ERROR" => Ok(EnclaveError::EnclaveKmstoolError),
                    "ENCLAVE_ERROR_ENCLAVE_WALLET_ERROR" => Ok(EnclaveError::EnclaveWalletError),
                    "ENCLAVE_ERROR_POSTCARD_ERROR" => Ok(EnclaveError::PostcardError),
                    "ENCLAVE_ERROR_LOG_ERROR" => Ok(EnclaveError::LogError),
                    "ENCLAVE_ERROR_IO_ERROR" => Ok(EnclaveError::IoError),
                    "ENCLAVE_ERROR_ANYHOW_ERROR" => Ok(EnclaveError::AnyhowError),
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
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.v1.EnclaveRequest", len)?;
        if let Some(v) = self.request.as_ref() {
            match v {
                enclave_request::Request::PingRequest(v) => {
                    struct_ser.serialize_field("pingRequest", v)?;
                }
                enclave_request::Request::InitKmstoolRequest(v) => {
                    struct_ser.serialize_field("initKmstoolRequest", v)?;
                }
                enclave_request::Request::UpdateAwsCredentialsRequest(v) => {
                    struct_ser.serialize_field("updateAwsCredentialsRequest", v)?;
                }
                enclave_request::Request::CreateEnclaveWalletRequest(v) => {
                    struct_ser.serialize_field("createEnclaveWalletRequest", v)?;
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
            "ping_request",
            "pingRequest",
            "init_kmstool_request",
            "initKmstoolRequest",
            "update_aws_credentials_request",
            "updateAwsCredentialsRequest",
            "create_enclave_wallet_request",
            "createEnclaveWalletRequest",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PingRequest,
            InitKmstoolRequest,
            UpdateAwsCredentialsRequest,
            CreateEnclaveWalletRequest,
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
                            "initKmstoolRequest" | "init_kmstool_request" => Ok(GeneratedField::InitKmstoolRequest),
                            "updateAwsCredentialsRequest" | "update_aws_credentials_request" => Ok(GeneratedField::UpdateAwsCredentialsRequest),
                            "createEnclaveWalletRequest" | "create_enclave_wallet_request" => Ok(GeneratedField::CreateEnclaveWalletRequest),
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
                formatter.write_str("struct vaultron.enclave.v1.EnclaveRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EnclaveRequest, V::Error>
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
                            request__ = map.next_value::<::std::option::Option<_>>()?.map(enclave_request::Request::PingRequest)
;
                        }
                        GeneratedField::InitKmstoolRequest => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initKmstoolRequest"));
                            }
                            request__ = map.next_value::<::std::option::Option<_>>()?.map(enclave_request::Request::InitKmstoolRequest)
;
                        }
                        GeneratedField::UpdateAwsCredentialsRequest => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateAwsCredentialsRequest"));
                            }
                            request__ = map.next_value::<::std::option::Option<_>>()?.map(enclave_request::Request::UpdateAwsCredentialsRequest)
;
                        }
                        GeneratedField::CreateEnclaveWalletRequest => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createEnclaveWalletRequest"));
                            }
                            request__ = map.next_value::<::std::option::Option<_>>()?.map(enclave_request::Request::CreateEnclaveWalletRequest)
;
                        }
                    }
                }
                Ok(EnclaveRequest {
                    request: request__,
                })
            }
        }
        deserializer.deserialize_struct("vaultron.enclave.v1.EnclaveRequest", FIELDS, GeneratedVisitor)
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
        if self.response.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.v1.EnclaveResponse", len)?;
        if let Some(v) = self.response.as_ref() {
            match v {
                enclave_response::Response::PingResponse(v) => {
                    struct_ser.serialize_field("pingResponse", v)?;
                }
                enclave_response::Response::InitKmstoolResponse(v) => {
                    struct_ser.serialize_field("initKmstoolResponse", v)?;
                }
                enclave_response::Response::UpdateAwsCredentialsResponse(v) => {
                    struct_ser.serialize_field("updateAwsCredentialsResponse", v)?;
                }
                enclave_response::Response::CreateEnclaveWalletResponse(v) => {
                    struct_ser.serialize_field("createEnclaveWalletResponse", v)?;
                }
            }
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
            "ping_response",
            "pingResponse",
            "init_kmstool_response",
            "initKmstoolResponse",
            "update_aws_credentials_response",
            "updateAwsCredentialsResponse",
            "create_enclave_wallet_response",
            "createEnclaveWalletResponse",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PingResponse,
            InitKmstoolResponse,
            UpdateAwsCredentialsResponse,
            CreateEnclaveWalletResponse,
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
                            "pingResponse" | "ping_response" => Ok(GeneratedField::PingResponse),
                            "initKmstoolResponse" | "init_kmstool_response" => Ok(GeneratedField::InitKmstoolResponse),
                            "updateAwsCredentialsResponse" | "update_aws_credentials_response" => Ok(GeneratedField::UpdateAwsCredentialsResponse),
                            "createEnclaveWalletResponse" | "create_enclave_wallet_response" => Ok(GeneratedField::CreateEnclaveWalletResponse),
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
                formatter.write_str("struct vaultron.enclave.v1.EnclaveResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EnclaveResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut response__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PingResponse => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pingResponse"));
                            }
                            response__ = map.next_value::<::std::option::Option<_>>()?.map(enclave_response::Response::PingResponse)
;
                        }
                        GeneratedField::InitKmstoolResponse => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initKmstoolResponse"));
                            }
                            response__ = map.next_value::<::std::option::Option<_>>()?.map(enclave_response::Response::InitKmstoolResponse)
;
                        }
                        GeneratedField::UpdateAwsCredentialsResponse => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateAwsCredentialsResponse"));
                            }
                            response__ = map.next_value::<::std::option::Option<_>>()?.map(enclave_response::Response::UpdateAwsCredentialsResponse)
;
                        }
                        GeneratedField::CreateEnclaveWalletResponse => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createEnclaveWalletResponse"));
                            }
                            response__ = map.next_value::<::std::option::Option<_>>()?.map(enclave_response::Response::CreateEnclaveWalletResponse)
;
                        }
                    }
                }
                Ok(EnclaveResponse {
                    response: response__,
                })
            }
        }
        deserializer.deserialize_struct("vaultron.enclave.v1.EnclaveResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InitKmstoolRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.enable_logging.is_some() {
            len += 1;
        }
        if self.proxy_port.is_some() {
            len += 1;
        }
        if !self.aws_region.is_empty() {
            len += 1;
        }
        if !self.aws_access_key_id.is_empty() {
            len += 1;
        }
        if !self.aws_secret_access_key.is_empty() {
            len += 1;
        }
        if !self.aws_session_token.is_empty() {
            len += 1;
        }
        if !self.kms_key_id.is_empty() {
            len += 1;
        }
        if self.kms_encryption_algorithm.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.v1.InitKmstoolRequest", len)?;
        if let Some(v) = self.enable_logging.as_ref() {
            struct_ser.serialize_field("enableLogging", v)?;
        }
        if let Some(v) = self.proxy_port.as_ref() {
            struct_ser.serialize_field("proxyPort", v)?;
        }
        if !self.aws_region.is_empty() {
            struct_ser.serialize_field("awsRegion", &self.aws_region)?;
        }
        if !self.aws_access_key_id.is_empty() {
            struct_ser.serialize_field("awsAccessKeyId", &self.aws_access_key_id)?;
        }
        if !self.aws_secret_access_key.is_empty() {
            struct_ser.serialize_field("awsSecretAccessKey", &self.aws_secret_access_key)?;
        }
        if !self.aws_session_token.is_empty() {
            struct_ser.serialize_field("awsSessionToken", &self.aws_session_token)?;
        }
        if !self.kms_key_id.is_empty() {
            struct_ser.serialize_field("kmsKeyId", &self.kms_key_id)?;
        }
        if let Some(v) = self.kms_encryption_algorithm.as_ref() {
            struct_ser.serialize_field("kmsEncryptionAlgorithm", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InitKmstoolRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "enable_logging",
            "enableLogging",
            "proxy_port",
            "proxyPort",
            "aws_region",
            "awsRegion",
            "aws_access_key_id",
            "awsAccessKeyId",
            "aws_secret_access_key",
            "awsSecretAccessKey",
            "aws_session_token",
            "awsSessionToken",
            "kms_key_id",
            "kmsKeyId",
            "kms_encryption_algorithm",
            "kmsEncryptionAlgorithm",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EnableLogging,
            ProxyPort,
            AwsRegion,
            AwsAccessKeyId,
            AwsSecretAccessKey,
            AwsSessionToken,
            KmsKeyId,
            KmsEncryptionAlgorithm,
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
                            "enableLogging" | "enable_logging" => Ok(GeneratedField::EnableLogging),
                            "proxyPort" | "proxy_port" => Ok(GeneratedField::ProxyPort),
                            "awsRegion" | "aws_region" => Ok(GeneratedField::AwsRegion),
                            "awsAccessKeyId" | "aws_access_key_id" => Ok(GeneratedField::AwsAccessKeyId),
                            "awsSecretAccessKey" | "aws_secret_access_key" => Ok(GeneratedField::AwsSecretAccessKey),
                            "awsSessionToken" | "aws_session_token" => Ok(GeneratedField::AwsSessionToken),
                            "kmsKeyId" | "kms_key_id" => Ok(GeneratedField::KmsKeyId),
                            "kmsEncryptionAlgorithm" | "kms_encryption_algorithm" => Ok(GeneratedField::KmsEncryptionAlgorithm),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InitKmstoolRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.enclave.v1.InitKmstoolRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<InitKmstoolRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut enable_logging__ = None;
                let mut proxy_port__ = None;
                let mut aws_region__ = None;
                let mut aws_access_key_id__ = None;
                let mut aws_secret_access_key__ = None;
                let mut aws_session_token__ = None;
                let mut kms_key_id__ = None;
                let mut kms_encryption_algorithm__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::EnableLogging => {
                            if enable_logging__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableLogging"));
                            }
                            enable_logging__ = map.next_value()?;
                        }
                        GeneratedField::ProxyPort => {
                            if proxy_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proxyPort"));
                            }
                            proxy_port__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::AwsRegion => {
                            if aws_region__.is_some() {
                                return Err(serde::de::Error::duplicate_field("awsRegion"));
                            }
                            aws_region__ = Some(map.next_value()?);
                        }
                        GeneratedField::AwsAccessKeyId => {
                            if aws_access_key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("awsAccessKeyId"));
                            }
                            aws_access_key_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::AwsSecretAccessKey => {
                            if aws_secret_access_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("awsSecretAccessKey"));
                            }
                            aws_secret_access_key__ = Some(map.next_value()?);
                        }
                        GeneratedField::AwsSessionToken => {
                            if aws_session_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("awsSessionToken"));
                            }
                            aws_session_token__ = Some(map.next_value()?);
                        }
                        GeneratedField::KmsKeyId => {
                            if kms_key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kmsKeyId"));
                            }
                            kms_key_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::KmsEncryptionAlgorithm => {
                            if kms_encryption_algorithm__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kmsEncryptionAlgorithm"));
                            }
                            kms_encryption_algorithm__ = map.next_value()?;
                        }
                    }
                }
                Ok(InitKmstoolRequest {
                    enable_logging: enable_logging__,
                    proxy_port: proxy_port__,
                    aws_region: aws_region__.unwrap_or_default(),
                    aws_access_key_id: aws_access_key_id__.unwrap_or_default(),
                    aws_secret_access_key: aws_secret_access_key__.unwrap_or_default(),
                    aws_session_token: aws_session_token__.unwrap_or_default(),
                    kms_key_id: kms_key_id__.unwrap_or_default(),
                    kms_encryption_algorithm: kms_encryption_algorithm__,
                })
            }
        }
        deserializer.deserialize_struct("vaultron.enclave.v1.InitKmstoolRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InitKmstoolResponse {
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
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.v1.InitKmstoolResponse", len)?;
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InitKmstoolResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InitKmstoolResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.enclave.v1.InitKmstoolResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<InitKmstoolResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = map.next_value()?;
                        }
                    }
                }
                Ok(InitKmstoolResponse {
                    code: code__,
                })
            }
        }
        deserializer.deserialize_struct("vaultron.enclave.v1.InitKmstoolResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PingRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("vaultron.enclave.v1.PingRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PingRequest {
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
            type Value = PingRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.enclave.v1.PingRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PingRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PingRequest {
                })
            }
        }
        deserializer.deserialize_struct("vaultron.enclave.v1.PingRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PingResponse {
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
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.v1.PingResponse", len)?;
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PingResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PingResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.enclave.v1.PingResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PingResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = map.next_value()?;
                        }
                    }
                }
                Ok(PingResponse {
                    code: code__,
                })
            }
        }
        deserializer.deserialize_struct("vaultron.enclave.v1.PingResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SignatureType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SIGNATURE_TYPE_UNSPECIFIED",
            Self::Fairx => "SIGNATURE_TYPE_FAIRX",
            Self::WalletBitcoin => "SIGNATURE_TYPE_WALLET_BITCOIN",
            Self::WalletEth => "SIGNATURE_TYPE_WALLET_ETH",
            Self::WalletSolana => "SIGNATURE_TYPE_WALLET_SOLANA",
            Self::WalletSui => "SIGNATURE_TYPE_WALLET_SUI",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SignatureType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SIGNATURE_TYPE_UNSPECIFIED",
            "SIGNATURE_TYPE_FAIRX",
            "SIGNATURE_TYPE_WALLET_BITCOIN",
            "SIGNATURE_TYPE_WALLET_ETH",
            "SIGNATURE_TYPE_WALLET_SOLANA",
            "SIGNATURE_TYPE_WALLET_SUI",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SignatureType;

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
                    .and_then(SignatureType::from_i32)
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
                    .and_then(SignatureType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "SIGNATURE_TYPE_UNSPECIFIED" => Ok(SignatureType::Unspecified),
                    "SIGNATURE_TYPE_FAIRX" => Ok(SignatureType::Fairx),
                    "SIGNATURE_TYPE_WALLET_BITCOIN" => Ok(SignatureType::WalletBitcoin),
                    "SIGNATURE_TYPE_WALLET_ETH" => Ok(SignatureType::WalletEth),
                    "SIGNATURE_TYPE_WALLET_SOLANA" => Ok(SignatureType::WalletSolana),
                    "SIGNATURE_TYPE_WALLET_SUI" => Ok(SignatureType::WalletSui),
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
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.v1.StatusCode", len)?;
        if self.success {
            struct_ser.serialize_field("success", &self.success)?;
        }
        if let Some(v) = self.error_message.as_ref() {
            struct_ser.serialize_field("errorMessage", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            match v {
                status_code::Error::Enclave(v) => {
                    let v = EnclaveError::from_i32(*v)
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
                formatter.write_str("struct vaultron.enclave.v1.StatusCode")
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
                            error__ = map.next_value::<::std::option::Option<EnclaveError>>()?.map(|x| status_code::Error::Enclave(x as i32));
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
        deserializer.deserialize_struct("vaultron.enclave.v1.StatusCode", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateAwsCredentialsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.aws_access_key_id.is_empty() {
            len += 1;
        }
        if !self.aws_secret_access_key.is_empty() {
            len += 1;
        }
        if !self.aws_session_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.v1.UpdateAwsCredentialsRequest", len)?;
        if !self.aws_access_key_id.is_empty() {
            struct_ser.serialize_field("awsAccessKeyId", &self.aws_access_key_id)?;
        }
        if !self.aws_secret_access_key.is_empty() {
            struct_ser.serialize_field("awsSecretAccessKey", &self.aws_secret_access_key)?;
        }
        if !self.aws_session_token.is_empty() {
            struct_ser.serialize_field("awsSessionToken", &self.aws_session_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateAwsCredentialsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "aws_access_key_id",
            "awsAccessKeyId",
            "aws_secret_access_key",
            "awsSecretAccessKey",
            "aws_session_token",
            "awsSessionToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AwsAccessKeyId,
            AwsSecretAccessKey,
            AwsSessionToken,
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
                            "awsAccessKeyId" | "aws_access_key_id" => Ok(GeneratedField::AwsAccessKeyId),
                            "awsSecretAccessKey" | "aws_secret_access_key" => Ok(GeneratedField::AwsSecretAccessKey),
                            "awsSessionToken" | "aws_session_token" => Ok(GeneratedField::AwsSessionToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateAwsCredentialsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.enclave.v1.UpdateAwsCredentialsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdateAwsCredentialsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut aws_access_key_id__ = None;
                let mut aws_secret_access_key__ = None;
                let mut aws_session_token__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AwsAccessKeyId => {
                            if aws_access_key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("awsAccessKeyId"));
                            }
                            aws_access_key_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::AwsSecretAccessKey => {
                            if aws_secret_access_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("awsSecretAccessKey"));
                            }
                            aws_secret_access_key__ = Some(map.next_value()?);
                        }
                        GeneratedField::AwsSessionToken => {
                            if aws_session_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("awsSessionToken"));
                            }
                            aws_session_token__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(UpdateAwsCredentialsRequest {
                    aws_access_key_id: aws_access_key_id__.unwrap_or_default(),
                    aws_secret_access_key: aws_secret_access_key__.unwrap_or_default(),
                    aws_session_token: aws_session_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vaultron.enclave.v1.UpdateAwsCredentialsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateAwsCredentialsResponse {
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
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.v1.UpdateAwsCredentialsResponse", len)?;
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateAwsCredentialsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateAwsCredentialsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.enclave.v1.UpdateAwsCredentialsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdateAwsCredentialsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = map.next_value()?;
                        }
                    }
                }
                Ok(UpdateAwsCredentialsResponse {
                    code: code__,
                })
            }
        }
        deserializer.deserialize_struct("vaultron.enclave.v1.UpdateAwsCredentialsResponse", FIELDS, GeneratedVisitor)
    }
}
