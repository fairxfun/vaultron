// @generated
impl serde::Serialize for AwsCredentials {
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
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.v1.AwsCredentials", len)?;
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
impl<'de> serde::Deserialize<'de> for AwsCredentials {
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
            type Value = AwsCredentials;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.enclave.v1.AwsCredentials")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AwsCredentials, V::Error>
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
                Ok(AwsCredentials {
                    aws_access_key_id: aws_access_key_id__.unwrap_or_default(),
                    aws_secret_access_key: aws_secret_access_key__.unwrap_or_default(),
                    aws_session_token: aws_session_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vaultron.enclave.v1.AwsCredentials", FIELDS, GeneratedVisitor)
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
            Self::InvalidParams => "ENCLAVE_ERROR_INVALID_PARAMS",
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
            "ENCLAVE_ERROR_INVALID_PARAMS",
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
                    "ENCLAVE_ERROR_INVALID_PARAMS" => Ok(EnclaveError::InvalidParams),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for InitOptions {
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
        if !self.kms_key_id.is_empty() {
            len += 1;
        }
        if self.kms_encryption_algorithm.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.v1.InitOptions", len)?;
        if let Some(v) = self.enable_logging.as_ref() {
            struct_ser.serialize_field("enableLogging", v)?;
        }
        if let Some(v) = self.proxy_port.as_ref() {
            struct_ser.serialize_field("proxyPort", v)?;
        }
        if !self.aws_region.is_empty() {
            struct_ser.serialize_field("awsRegion", &self.aws_region)?;
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
impl<'de> serde::Deserialize<'de> for InitOptions {
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
            type Value = InitOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.enclave.v1.InitOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<InitOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut enable_logging__ = None;
                let mut proxy_port__ = None;
                let mut aws_region__ = None;
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
                Ok(InitOptions {
                    enable_logging: enable_logging__,
                    proxy_port: proxy_port__,
                    aws_region: aws_region__.unwrap_or_default(),
                    kms_key_id: kms_key_id__.unwrap_or_default(),
                    kms_encryption_algorithm: kms_encryption_algorithm__,
                })
            }
        }
        deserializer.deserialize_struct("vaultron.enclave.v1.InitOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InitRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.options.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.v1.InitRequest", len)?;
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InitRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "options",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Options,
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
                            "options" => Ok(GeneratedField::Options),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InitRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.enclave.v1.InitRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<InitRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut options__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = map.next_value()?;
                        }
                    }
                }
                Ok(InitRequest {
                    options: options__,
                })
            }
        }
        deserializer.deserialize_struct("vaultron.enclave.v1.InitRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InitResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.v1.InitResponse", len)?;
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InitResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
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
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InitResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.enclave.v1.InitResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<InitResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                    }
                }
                Ok(InitResponse {
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("vaultron.enclave.v1.InitResponse", FIELDS, GeneratedVisitor)
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
        if self.error.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.v1.StatusCode", len)?;
        if self.success {
            struct_ser.serialize_field("success", &self.success)?;
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
            "enclave",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Success,
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
                let mut error__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Success => {
                            if success__.is_some() {
                                return Err(serde::de::Error::duplicate_field("success"));
                            }
                            success__ = Some(map.next_value()?);
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
        if self.aws_credentials.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.v1.UpdateAwsCredentialsRequest", len)?;
        if let Some(v) = self.aws_credentials.as_ref() {
            struct_ser.serialize_field("awsCredentials", v)?;
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
            "aws_credentials",
            "awsCredentials",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AwsCredentials,
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
                            "awsCredentials" | "aws_credentials" => Ok(GeneratedField::AwsCredentials),
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
                let mut aws_credentials__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AwsCredentials => {
                            if aws_credentials__.is_some() {
                                return Err(serde::de::Error::duplicate_field("awsCredentials"));
                            }
                            aws_credentials__ = map.next_value()?;
                        }
                    }
                }
                Ok(UpdateAwsCredentialsRequest {
                    aws_credentials: aws_credentials__,
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
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.v1.UpdateAwsCredentialsResponse", len)?;
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
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
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
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
                            "status" => Ok(GeneratedField::Status),
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
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                    }
                }
                Ok(UpdateAwsCredentialsResponse {
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("vaultron.enclave.v1.UpdateAwsCredentialsResponse", FIELDS, GeneratedVisitor)
    }
}
