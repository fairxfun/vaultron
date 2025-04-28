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
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.v1.EnclaveClusterRequest", len)?;
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
                formatter.write_str("struct vaultron.enclave.v1.EnclaveClusterRequest")
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
        deserializer.deserialize_struct("vaultron.enclave.v1.EnclaveClusterRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.v1.EnclaveInternalRequest", len)?;
        if let Some(v) = self.request.as_ref() {
            match v {
                enclave_internal_request::Request::PingRequest(v) => {
                    struct_ser.serialize_field("pingRequest", v)?;
                }
                enclave_internal_request::Request::GetAttributesRequest(v) => {
                    struct_ser.serialize_field("getAttributesRequest", v)?;
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
            "get_attributes_request",
            "getAttributesRequest",
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
            GetAttributesRequest,
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
                            "getAttributesRequest" | "get_attributes_request" => Ok(GeneratedField::GetAttributesRequest),
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
                formatter.write_str("struct vaultron.enclave.v1.EnclaveInternalRequest")
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
                        GeneratedField::GetAttributesRequest => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("getAttributesRequest"));
                            }
                            request__ = map.next_value::<::std::option::Option<_>>()?.map(enclave_internal_request::Request::GetAttributesRequest)
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
        deserializer.deserialize_struct("vaultron.enclave.v1.EnclaveInternalRequest", FIELDS, GeneratedVisitor)
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
                formatter.write_str("struct vaultron.enclave.v1.EnclaveRequest")
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
        if self.code.is_some() {
            len += 1;
        }
        if !self.attestation_document.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.v1.EnclaveResponse", len)?;
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
                formatter.write_str("struct vaultron.enclave.v1.EnclaveResponse")
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
        deserializer.deserialize_struct("vaultron.enclave.v1.EnclaveResponse", FIELDS, GeneratedVisitor)
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
        let struct_ser = serializer.serialize_struct("vaultron.enclave.v1.EnclaveResponsePadding", len)?;
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
                formatter.write_str("struct vaultron.enclave.v1.EnclaveResponsePadding")
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
        deserializer.deserialize_struct("vaultron.enclave.v1.EnclaveResponsePadding", FIELDS, GeneratedVisitor)
    }
}
