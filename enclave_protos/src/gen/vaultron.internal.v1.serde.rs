// @generated
impl serde::Serialize for EnclaveData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.enclave_pcr0.is_empty() {
            len += 1;
        }
        if !self.enclave_public_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.internal.v1.EnclaveData", len)?;
        if !self.enclave_pcr0.is_empty() {
            struct_ser.serialize_field("enclavePcr0", pbjson::private::base64::encode(&self.enclave_pcr0).as_str())?;
        }
        if !self.enclave_public_key.is_empty() {
            struct_ser.serialize_field("enclavePublicKey", pbjson::private::base64::encode(&self.enclave_public_key).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EnclaveData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "enclave_pcr0",
            "enclavePcr0",
            "enclave_public_key",
            "enclavePublicKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EnclavePcr0,
            EnclavePublicKey,
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
                            "enclavePcr0" | "enclave_pcr0" => Ok(GeneratedField::EnclavePcr0),
                            "enclavePublicKey" | "enclave_public_key" => Ok(GeneratedField::EnclavePublicKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EnclaveData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.internal.v1.EnclaveData")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EnclaveData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut enclave_pcr0__ = None;
                let mut enclave_public_key__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::EnclavePcr0 => {
                            if enclave_pcr0__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enclavePcr0"));
                            }
                            enclave_pcr0__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EnclavePublicKey => {
                            if enclave_public_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enclavePublicKey"));
                            }
                            enclave_public_key__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(EnclaveData {
                    enclave_pcr0: enclave_pcr0__.unwrap_or_default(),
                    enclave_public_key: enclave_public_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vaultron.internal.v1.EnclaveData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ForwardClusterKeySyncRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.requester_doc.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.internal.v1.ForwardClusterKeySyncRequest", len)?;
        if !self.requester_doc.is_empty() {
            struct_ser.serialize_field("requesterDoc", pbjson::private::base64::encode(&self.requester_doc).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ForwardClusterKeySyncRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "requester_doc",
            "requesterDoc",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RequesterDoc,
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
                            "requesterDoc" | "requester_doc" => Ok(GeneratedField::RequesterDoc),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ForwardClusterKeySyncRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.internal.v1.ForwardClusterKeySyncRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ForwardClusterKeySyncRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut requester_doc__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RequesterDoc => {
                            if requester_doc__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requesterDoc"));
                            }
                            requester_doc__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ForwardClusterKeySyncRequest {
                    requester_doc: requester_doc__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vaultron.internal.v1.ForwardClusterKeySyncRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ForwardClusterKeySyncResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.encrypted_cluster_seed.is_empty() {
            len += 1;
        }
        if !self.cluster_public_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.internal.v1.ForwardClusterKeySyncResponse", len)?;
        if !self.encrypted_cluster_seed.is_empty() {
            struct_ser.serialize_field("encryptedClusterSeed", pbjson::private::base64::encode(&self.encrypted_cluster_seed).as_str())?;
        }
        if !self.cluster_public_key.is_empty() {
            struct_ser.serialize_field("clusterPublicKey", pbjson::private::base64::encode(&self.cluster_public_key).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ForwardClusterKeySyncResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "encrypted_cluster_seed",
            "encryptedClusterSeed",
            "cluster_public_key",
            "clusterPublicKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EncryptedClusterSeed,
            ClusterPublicKey,
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
                            "encryptedClusterSeed" | "encrypted_cluster_seed" => Ok(GeneratedField::EncryptedClusterSeed),
                            "clusterPublicKey" | "cluster_public_key" => Ok(GeneratedField::ClusterPublicKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ForwardClusterKeySyncResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.internal.v1.ForwardClusterKeySyncResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ForwardClusterKeySyncResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut encrypted_cluster_seed__ = None;
                let mut cluster_public_key__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::EncryptedClusterSeed => {
                            if encrypted_cluster_seed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("encryptedClusterSeed"));
                            }
                            encrypted_cluster_seed__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ClusterPublicKey => {
                            if cluster_public_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterPublicKey"));
                            }
                            cluster_public_key__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ForwardClusterKeySyncResponse {
                    encrypted_cluster_seed: encrypted_cluster_seed__.unwrap_or_default(),
                    cluster_public_key: cluster_public_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vaultron.internal.v1.ForwardClusterKeySyncResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InitClusterKeyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("vaultron.internal.v1.InitClusterKeyRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InitClusterKeyRequest {
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
            type Value = InitClusterKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.internal.v1.InitClusterKeyRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<InitClusterKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(InitClusterKeyRequest {
                })
            }
        }
        deserializer.deserialize_struct("vaultron.internal.v1.InitClusterKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InitClusterKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cluster_public_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.internal.v1.InitClusterKeyResponse", len)?;
        if !self.cluster_public_key.is_empty() {
            struct_ser.serialize_field("clusterPublicKey", pbjson::private::base64::encode(&self.cluster_public_key).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InitClusterKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cluster_public_key",
            "clusterPublicKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClusterPublicKey,
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
                            "clusterPublicKey" | "cluster_public_key" => Ok(GeneratedField::ClusterPublicKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InitClusterKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.internal.v1.InitClusterKeyResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<InitClusterKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cluster_public_key__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClusterPublicKey => {
                            if cluster_public_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterPublicKey"));
                            }
                            cluster_public_key__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(InitClusterKeyResponse {
                    cluster_public_key: cluster_public_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vaultron.internal.v1.InitClusterKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InitClusterKeySyncRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("vaultron.internal.v1.InitClusterKeySyncRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InitClusterKeySyncRequest {
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
            type Value = InitClusterKeySyncRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.internal.v1.InitClusterKeySyncRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<InitClusterKeySyncRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(InitClusterKeySyncRequest {
                })
            }
        }
        deserializer.deserialize_struct("vaultron.internal.v1.InitClusterKeySyncRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InitClusterKeySyncResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("vaultron.internal.v1.InitClusterKeySyncResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InitClusterKeySyncResponse {
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
            type Value = InitClusterKeySyncResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.internal.v1.InitClusterKeySyncResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<InitClusterKeySyncResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(InitClusterKeySyncResponse {
                })
            }
        }
        deserializer.deserialize_struct("vaultron.internal.v1.InitClusterKeySyncResponse", FIELDS, GeneratedVisitor)
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
        let struct_ser = serializer.serialize_struct("vaultron.internal.v1.PingRequest", len)?;
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
                formatter.write_str("struct vaultron.internal.v1.PingRequest")
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
        deserializer.deserialize_struct("vaultron.internal.v1.PingRequest", FIELDS, GeneratedVisitor)
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
        if self.enclave_data.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.internal.v1.PingResponse", len)?;
        if let Some(v) = self.enclave_data.as_ref() {
            struct_ser.serialize_field("enclaveData", v)?;
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
            "enclave_data",
            "enclaveData",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EnclaveData,
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
                            "enclaveData" | "enclave_data" => Ok(GeneratedField::EnclaveData),
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
                formatter.write_str("struct vaultron.internal.v1.PingResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PingResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut enclave_data__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::EnclaveData => {
                            if enclave_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enclaveData"));
                            }
                            enclave_data__ = map.next_value()?;
                        }
                    }
                }
                Ok(PingResponse {
                    enclave_data: enclave_data__,
                })
            }
        }
        deserializer.deserialize_struct("vaultron.internal.v1.PingResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReplyClusterKeySyncRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.responder_doc.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.internal.v1.ReplyClusterKeySyncRequest", len)?;
        if !self.responder_doc.is_empty() {
            struct_ser.serialize_field("responderDoc", pbjson::private::base64::encode(&self.responder_doc).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReplyClusterKeySyncRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "responder_doc",
            "responderDoc",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResponderDoc,
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
                            "responderDoc" | "responder_doc" => Ok(GeneratedField::ResponderDoc),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReplyClusterKeySyncRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.internal.v1.ReplyClusterKeySyncRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ReplyClusterKeySyncRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut responder_doc__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ResponderDoc => {
                            if responder_doc__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responderDoc"));
                            }
                            responder_doc__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ReplyClusterKeySyncRequest {
                    responder_doc: responder_doc__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vaultron.internal.v1.ReplyClusterKeySyncRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReplyClusterKeySyncResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("vaultron.internal.v1.ReplyClusterKeySyncResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReplyClusterKeySyncResponse {
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
            type Value = ReplyClusterKeySyncResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.internal.v1.ReplyClusterKeySyncResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ReplyClusterKeySyncResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ReplyClusterKeySyncResponse {
                })
            }
        }
        deserializer.deserialize_struct("vaultron.internal.v1.ReplyClusterKeySyncResponse", FIELDS, GeneratedVisitor)
    }
}
