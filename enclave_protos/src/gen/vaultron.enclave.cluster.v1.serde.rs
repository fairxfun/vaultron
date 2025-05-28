// @generated
impl serde::Serialize for ChainEndpoint {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.chain_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.cluster.v1.ChainEndpoint", len)?;
        if self.chain_id != 0 {
            struct_ser.serialize_field("chainId", &self.chain_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ChainEndpoint {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
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
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ChainEndpoint;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.enclave.cluster.v1.ChainEndpoint")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ChainEndpoint, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ChainEndpoint {
                    chain_id: chain_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vaultron.enclave.cluster.v1.ChainEndpoint", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClusterAttributes {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.enclave_version.is_empty() {
            len += 1;
        }
        if !self.enclave_pcr0.is_empty() {
            len += 1;
        }
        if !self.cluster_public_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.cluster.v1.ClusterAttributes", len)?;
        if !self.enclave_version.is_empty() {
            struct_ser.serialize_field("enclaveVersion", &self.enclave_version)?;
        }
        if !self.enclave_pcr0.is_empty() {
            struct_ser.serialize_field("enclavePcr0", pbjson::private::base64::encode(&self.enclave_pcr0).as_str())?;
        }
        if !self.cluster_public_key.is_empty() {
            struct_ser.serialize_field("clusterPublicKey", pbjson::private::base64::encode(&self.cluster_public_key).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClusterAttributes {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "enclave_version",
            "enclaveVersion",
            "enclave_pcr0",
            "enclavePcr0",
            "cluster_public_key",
            "clusterPublicKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EnclaveVersion,
            EnclavePcr0,
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
                            "enclaveVersion" | "enclave_version" => Ok(GeneratedField::EnclaveVersion),
                            "enclavePcr0" | "enclave_pcr0" => Ok(GeneratedField::EnclavePcr0),
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
            type Value = ClusterAttributes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.enclave.cluster.v1.ClusterAttributes")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ClusterAttributes, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut enclave_version__ = None;
                let mut enclave_pcr0__ = None;
                let mut cluster_public_key__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::EnclaveVersion => {
                            if enclave_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enclaveVersion"));
                            }
                            enclave_version__ = Some(map.next_value()?);
                        }
                        GeneratedField::EnclavePcr0 => {
                            if enclave_pcr0__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enclavePcr0"));
                            }
                            enclave_pcr0__ = 
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
                Ok(ClusterAttributes {
                    enclave_version: enclave_version__.unwrap_or_default(),
                    enclave_pcr0: enclave_pcr0__.unwrap_or_default(),
                    cluster_public_key: cluster_public_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vaultron.enclave.cluster.v1.ClusterAttributes", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateEnclaveVaultronRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.attributes.is_some() {
            len += 1;
        }
        if !self.signature.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.cluster.v1.CreateEnclaveVaultronRequest", len)?;
        if let Some(v) = self.attributes.as_ref() {
            struct_ser.serialize_field("attributes", v)?;
        }
        if !self.signature.is_empty() {
            struct_ser.serialize_field("signature", pbjson::private::base64::encode(&self.signature).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateEnclaveVaultronRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "attributes",
            "signature",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Attributes,
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
                            "attributes" => Ok(GeneratedField::Attributes),
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
            type Value = CreateEnclaveVaultronRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.enclave.cluster.v1.CreateEnclaveVaultronRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CreateEnclaveVaultronRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut attributes__ = None;
                let mut signature__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Attributes => {
                            if attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributes"));
                            }
                            attributes__ = map.next_value()?;
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
                Ok(CreateEnclaveVaultronRequest {
                    attributes: attributes__,
                    signature: signature__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vaultron.enclave.cluster.v1.CreateEnclaveVaultronRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateEnclaveVaultronResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.encrypted_attributes.is_empty() {
            len += 1;
        }
        if !self.encrypted_seed.is_empty() {
            len += 1;
        }
        if !self.eth_public_key.is_empty() {
            len += 1;
        }
        if !self.solana_public_key.is_empty() {
            len += 1;
        }
        if !self.sui_public_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.cluster.v1.CreateEnclaveVaultronResponse", len)?;
        if !self.encrypted_attributes.is_empty() {
            struct_ser.serialize_field("encryptedAttributes", pbjson::private::base64::encode(&self.encrypted_attributes).as_str())?;
        }
        if !self.encrypted_seed.is_empty() {
            struct_ser.serialize_field("encryptedSeed", pbjson::private::base64::encode(&self.encrypted_seed).as_str())?;
        }
        if !self.eth_public_key.is_empty() {
            struct_ser.serialize_field("ethPublicKey", pbjson::private::base64::encode(&self.eth_public_key).as_str())?;
        }
        if !self.solana_public_key.is_empty() {
            struct_ser.serialize_field("solanaPublicKey", pbjson::private::base64::encode(&self.solana_public_key).as_str())?;
        }
        if !self.sui_public_key.is_empty() {
            struct_ser.serialize_field("suiPublicKey", pbjson::private::base64::encode(&self.sui_public_key).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateEnclaveVaultronResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "encrypted_attributes",
            "encryptedAttributes",
            "encrypted_seed",
            "encryptedSeed",
            "eth_public_key",
            "ethPublicKey",
            "solana_public_key",
            "solanaPublicKey",
            "sui_public_key",
            "suiPublicKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EncryptedAttributes,
            EncryptedSeed,
            EthPublicKey,
            SolanaPublicKey,
            SuiPublicKey,
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
                            "encryptedAttributes" | "encrypted_attributes" => Ok(GeneratedField::EncryptedAttributes),
                            "encryptedSeed" | "encrypted_seed" => Ok(GeneratedField::EncryptedSeed),
                            "ethPublicKey" | "eth_public_key" => Ok(GeneratedField::EthPublicKey),
                            "solanaPublicKey" | "solana_public_key" => Ok(GeneratedField::SolanaPublicKey),
                            "suiPublicKey" | "sui_public_key" => Ok(GeneratedField::SuiPublicKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateEnclaveVaultronResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.enclave.cluster.v1.CreateEnclaveVaultronResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CreateEnclaveVaultronResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut encrypted_attributes__ = None;
                let mut encrypted_seed__ = None;
                let mut eth_public_key__ = None;
                let mut solana_public_key__ = None;
                let mut sui_public_key__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::EncryptedAttributes => {
                            if encrypted_attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("encryptedAttributes"));
                            }
                            encrypted_attributes__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EncryptedSeed => {
                            if encrypted_seed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("encryptedSeed"));
                            }
                            encrypted_seed__ = 
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
                        GeneratedField::SolanaPublicKey => {
                            if solana_public_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("solanaPublicKey"));
                            }
                            solana_public_key__ = 
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
                    }
                }
                Ok(CreateEnclaveVaultronResponse {
                    encrypted_attributes: encrypted_attributes__.unwrap_or_default(),
                    encrypted_seed: encrypted_seed__.unwrap_or_default(),
                    eth_public_key: eth_public_key__.unwrap_or_default(),
                    solana_public_key: solana_public_key__.unwrap_or_default(),
                    sui_public_key: sui_public_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vaultron.enclave.cluster.v1.CreateEnclaveVaultronResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DexEndpoint {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.url != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.cluster.v1.DexEndpoint", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.url != 0 {
            struct_ser.serialize_field("url", &self.url)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DexEndpoint {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "url",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Url,
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
                            "name" => Ok(GeneratedField::Name),
                            "url" => Ok(GeneratedField::Url),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DexEndpoint;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.enclave.cluster.v1.DexEndpoint")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DexEndpoint, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut url__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Url => {
                            if url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            url__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(DexEndpoint {
                    name: name__.unwrap_or_default(),
                    url: url__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vaultron.enclave.cluster.v1.DexEndpoint", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Endpoint {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.endpoint.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.cluster.v1.Endpoint", len)?;
        if let Some(v) = self.endpoint.as_ref() {
            match v {
                endpoint::Endpoint::Chain(v) => {
                    struct_ser.serialize_field("chain", v)?;
                }
                endpoint::Endpoint::Dex(v) => {
                    struct_ser.serialize_field("dex", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Endpoint {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain",
            "dex",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Chain,
            Dex,
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
                            "chain" => Ok(GeneratedField::Chain),
                            "dex" => Ok(GeneratedField::Dex),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Endpoint;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.enclave.cluster.v1.Endpoint")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Endpoint, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut endpoint__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Chain => {
                            if endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chain"));
                            }
                            endpoint__ = map.next_value::<::std::option::Option<_>>()?.map(endpoint::Endpoint::Chain)
;
                        }
                        GeneratedField::Dex => {
                            if endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dex"));
                            }
                            endpoint__ = map.next_value::<::std::option::Option<_>>()?.map(endpoint::Endpoint::Dex)
;
                        }
                    }
                }
                Ok(Endpoint {
                    endpoint: endpoint__,
                })
            }
        }
        deserializer.deserialize_struct("vaultron.enclave.cluster.v1.Endpoint", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SignVaultronTaskTransactionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.vaultron_encrypted_attributes.is_empty() {
            len += 1;
        }
        if self.creation_attributes.is_some() {
            len += 1;
        }
        if !self.creation_attributes_signature.is_empty() {
            len += 1;
        }
        if !self.action_transaction.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.cluster.v1.SignVaultronTaskTransactionRequest", len)?;
        if !self.vaultron_encrypted_attributes.is_empty() {
            struct_ser.serialize_field("vaultronEncryptedAttributes", pbjson::private::base64::encode(&self.vaultron_encrypted_attributes).as_str())?;
        }
        if let Some(v) = self.creation_attributes.as_ref() {
            struct_ser.serialize_field("creationAttributes", v)?;
        }
        if !self.creation_attributes_signature.is_empty() {
            struct_ser.serialize_field("creationAttributesSignature", pbjson::private::base64::encode(&self.creation_attributes_signature).as_str())?;
        }
        if !self.action_transaction.is_empty() {
            struct_ser.serialize_field("actionTransaction", pbjson::private::base64::encode(&self.action_transaction).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SignVaultronTaskTransactionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "vaultron_encrypted_attributes",
            "vaultronEncryptedAttributes",
            "creation_attributes",
            "creationAttributes",
            "creation_attributes_signature",
            "creationAttributesSignature",
            "action_transaction",
            "actionTransaction",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VaultronEncryptedAttributes,
            CreationAttributes,
            CreationAttributesSignature,
            ActionTransaction,
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
                            "vaultronEncryptedAttributes" | "vaultron_encrypted_attributes" => Ok(GeneratedField::VaultronEncryptedAttributes),
                            "creationAttributes" | "creation_attributes" => Ok(GeneratedField::CreationAttributes),
                            "creationAttributesSignature" | "creation_attributes_signature" => Ok(GeneratedField::CreationAttributesSignature),
                            "actionTransaction" | "action_transaction" => Ok(GeneratedField::ActionTransaction),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SignVaultronTaskTransactionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.enclave.cluster.v1.SignVaultronTaskTransactionRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SignVaultronTaskTransactionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut vaultron_encrypted_attributes__ = None;
                let mut creation_attributes__ = None;
                let mut creation_attributes_signature__ = None;
                let mut action_transaction__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::VaultronEncryptedAttributes => {
                            if vaultron_encrypted_attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vaultronEncryptedAttributes"));
                            }
                            vaultron_encrypted_attributes__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CreationAttributes => {
                            if creation_attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creationAttributes"));
                            }
                            creation_attributes__ = map.next_value()?;
                        }
                        GeneratedField::CreationAttributesSignature => {
                            if creation_attributes_signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creationAttributesSignature"));
                            }
                            creation_attributes_signature__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ActionTransaction => {
                            if action_transaction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionTransaction"));
                            }
                            action_transaction__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(SignVaultronTaskTransactionRequest {
                    vaultron_encrypted_attributes: vaultron_encrypted_attributes__.unwrap_or_default(),
                    creation_attributes: creation_attributes__,
                    creation_attributes_signature: creation_attributes_signature__.unwrap_or_default(),
                    action_transaction: action_transaction__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vaultron.enclave.cluster.v1.SignVaultronTaskTransactionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SignVaultronTaskTransactionResponse {
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
        if self.action_transaction_signature.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.cluster.v1.SignVaultronTaskTransactionResponse", len)?;
        if self.success {
            struct_ser.serialize_field("success", &self.success)?;
        }
        if let Some(v) = self.error_message.as_ref() {
            struct_ser.serialize_field("errorMessage", v)?;
        }
        if let Some(v) = self.action_transaction_signature.as_ref() {
            struct_ser.serialize_field("actionTransactionSignature", pbjson::private::base64::encode(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SignVaultronTaskTransactionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "success",
            "error_message",
            "errorMessage",
            "action_transaction_signature",
            "actionTransactionSignature",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Success,
            ErrorMessage,
            ActionTransactionSignature,
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
                            "actionTransactionSignature" | "action_transaction_signature" => Ok(GeneratedField::ActionTransactionSignature),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SignVaultronTaskTransactionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.enclave.cluster.v1.SignVaultronTaskTransactionResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SignVaultronTaskTransactionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut success__ = None;
                let mut error_message__ = None;
                let mut action_transaction_signature__ = None;
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
                        GeneratedField::ActionTransactionSignature => {
                            if action_transaction_signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionTransactionSignature"));
                            }
                            action_transaction_signature__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(SignVaultronTaskTransactionResponse {
                    success: success__.unwrap_or_default(),
                    error_message: error_message__,
                    action_transaction_signature: action_transaction_signature__,
                })
            }
        }
        deserializer.deserialize_struct("vaultron.enclave.cluster.v1.SignVaultronTaskTransactionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TaskCreationAttributes {
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
        if !self.task_name.is_empty() {
            len += 1;
        }
        if !self.task_description.is_empty() {
            len += 1;
        }
        if !self.trigger_event_id.is_empty() {
            len += 1;
        }
        if self.action.is_some() {
            len += 1;
        }
        if self.expiry_time != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.cluster.v1.TaskCreationAttributes", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.task_name.is_empty() {
            struct_ser.serialize_field("taskName", &self.task_name)?;
        }
        if !self.task_description.is_empty() {
            struct_ser.serialize_field("taskDescription", &self.task_description)?;
        }
        if !self.trigger_event_id.is_empty() {
            struct_ser.serialize_field("triggerEventId", &self.trigger_event_id)?;
        }
        if let Some(v) = self.action.as_ref() {
            struct_ser.serialize_field("action", v)?;
        }
        if self.expiry_time != 0 {
            struct_ser.serialize_field("expiryTime", ToString::to_string(&self.expiry_time).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TaskCreationAttributes {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "task_name",
            "taskName",
            "task_description",
            "taskDescription",
            "trigger_event_id",
            "triggerEventId",
            "action",
            "expiry_time",
            "expiryTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            TaskName,
            TaskDescription,
            TriggerEventId,
            Action,
            ExpiryTime,
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
                            "taskName" | "task_name" => Ok(GeneratedField::TaskName),
                            "taskDescription" | "task_description" => Ok(GeneratedField::TaskDescription),
                            "triggerEventId" | "trigger_event_id" => Ok(GeneratedField::TriggerEventId),
                            "action" => Ok(GeneratedField::Action),
                            "expiryTime" | "expiry_time" => Ok(GeneratedField::ExpiryTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TaskCreationAttributes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.enclave.cluster.v1.TaskCreationAttributes")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TaskCreationAttributes, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut task_name__ = None;
                let mut task_description__ = None;
                let mut trigger_event_id__ = None;
                let mut action__ = None;
                let mut expiry_time__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::TaskName => {
                            if task_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taskName"));
                            }
                            task_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::TaskDescription => {
                            if task_description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("taskDescription"));
                            }
                            task_description__ = Some(map.next_value()?);
                        }
                        GeneratedField::TriggerEventId => {
                            if trigger_event_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("triggerEventId"));
                            }
                            trigger_event_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Action => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            action__ = map.next_value()?;
                        }
                        GeneratedField::ExpiryTime => {
                            if expiry_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expiryTime"));
                            }
                            expiry_time__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(TaskCreationAttributes {
                    user_id: user_id__.unwrap_or_default(),
                    task_name: task_name__.unwrap_or_default(),
                    task_description: task_description__.unwrap_or_default(),
                    trigger_event_id: trigger_event_id__.unwrap_or_default(),
                    action: action__,
                    expiry_time: expiry_time__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vaultron.enclave.cluster.v1.TaskCreationAttributes", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VaultronActionAttributes {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.attributes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.cluster.v1.VaultronActionAttributes", len)?;
        if let Some(v) = self.attributes.as_ref() {
            match v {
                vaultron_action_attributes::Attributes::Transfer(v) => {
                    struct_ser.serialize_field("transfer", v)?;
                }
                vaultron_action_attributes::Attributes::Buy(v) => {
                    struct_ser.serialize_field("buy", v)?;
                }
                vaultron_action_attributes::Attributes::Sell(v) => {
                    struct_ser.serialize_field("sell", v)?;
                }
                vaultron_action_attributes::Attributes::Stake(v) => {
                    struct_ser.serialize_field("stake", v)?;
                }
                vaultron_action_attributes::Attributes::Unstake(v) => {
                    struct_ser.serialize_field("unstake", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VaultronActionAttributes {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "transfer",
            "buy",
            "sell",
            "stake",
            "unstake",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Transfer,
            Buy,
            Sell,
            Stake,
            Unstake,
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
                            "transfer" => Ok(GeneratedField::Transfer),
                            "buy" => Ok(GeneratedField::Buy),
                            "sell" => Ok(GeneratedField::Sell),
                            "stake" => Ok(GeneratedField::Stake),
                            "unstake" => Ok(GeneratedField::Unstake),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VaultronActionAttributes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.enclave.cluster.v1.VaultronActionAttributes")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VaultronActionAttributes, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut attributes__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Transfer => {
                            if attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transfer"));
                            }
                            attributes__ = map.next_value::<::std::option::Option<_>>()?.map(vaultron_action_attributes::Attributes::Transfer)
;
                        }
                        GeneratedField::Buy => {
                            if attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("buy"));
                            }
                            attributes__ = map.next_value::<::std::option::Option<_>>()?.map(vaultron_action_attributes::Attributes::Buy)
;
                        }
                        GeneratedField::Sell => {
                            if attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sell"));
                            }
                            attributes__ = map.next_value::<::std::option::Option<_>>()?.map(vaultron_action_attributes::Attributes::Sell)
;
                        }
                        GeneratedField::Stake => {
                            if attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stake"));
                            }
                            attributes__ = map.next_value::<::std::option::Option<_>>()?.map(vaultron_action_attributes::Attributes::Stake)
;
                        }
                        GeneratedField::Unstake => {
                            if attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unstake"));
                            }
                            attributes__ = map.next_value::<::std::option::Option<_>>()?.map(vaultron_action_attributes::Attributes::Unstake)
;
                        }
                    }
                }
                Ok(VaultronActionAttributes {
                    attributes: attributes__,
                })
            }
        }
        deserializer.deserialize_struct("vaultron.enclave.cluster.v1.VaultronActionAttributes", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VaultronActionBuyAttributes {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pair.is_some() {
            len += 1;
        }
        if !self.protocol_address.is_empty() {
            len += 1;
        }
        if !self.base_amount.is_empty() {
            len += 1;
        }
        if !self.max_trading_price.is_empty() {
            len += 1;
        }
        if !self.slippage_tolerance.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.cluster.v1.VaultronActionBuyAttributes", len)?;
        if let Some(v) = self.pair.as_ref() {
            struct_ser.serialize_field("pair", v)?;
        }
        if !self.protocol_address.is_empty() {
            struct_ser.serialize_field("protocolAddress", &self.protocol_address)?;
        }
        if !self.base_amount.is_empty() {
            struct_ser.serialize_field("baseAmount", &self.base_amount)?;
        }
        if !self.max_trading_price.is_empty() {
            struct_ser.serialize_field("maxTradingPrice", &self.max_trading_price)?;
        }
        if !self.slippage_tolerance.is_empty() {
            struct_ser.serialize_field("slippageTolerance", &self.slippage_tolerance)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VaultronActionBuyAttributes {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pair",
            "protocol_address",
            "protocolAddress",
            "base_amount",
            "baseAmount",
            "max_trading_price",
            "maxTradingPrice",
            "slippage_tolerance",
            "slippageTolerance",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pair,
            ProtocolAddress,
            BaseAmount,
            MaxTradingPrice,
            SlippageTolerance,
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
                            "pair" => Ok(GeneratedField::Pair),
                            "protocolAddress" | "protocol_address" => Ok(GeneratedField::ProtocolAddress),
                            "baseAmount" | "base_amount" => Ok(GeneratedField::BaseAmount),
                            "maxTradingPrice" | "max_trading_price" => Ok(GeneratedField::MaxTradingPrice),
                            "slippageTolerance" | "slippage_tolerance" => Ok(GeneratedField::SlippageTolerance),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VaultronActionBuyAttributes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.enclave.cluster.v1.VaultronActionBuyAttributes")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VaultronActionBuyAttributes, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pair__ = None;
                let mut protocol_address__ = None;
                let mut base_amount__ = None;
                let mut max_trading_price__ = None;
                let mut slippage_tolerance__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Pair => {
                            if pair__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pair"));
                            }
                            pair__ = map.next_value()?;
                        }
                        GeneratedField::ProtocolAddress => {
                            if protocol_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocolAddress"));
                            }
                            protocol_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::BaseAmount => {
                            if base_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseAmount"));
                            }
                            base_amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxTradingPrice => {
                            if max_trading_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxTradingPrice"));
                            }
                            max_trading_price__ = Some(map.next_value()?);
                        }
                        GeneratedField::SlippageTolerance => {
                            if slippage_tolerance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slippageTolerance"));
                            }
                            slippage_tolerance__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(VaultronActionBuyAttributes {
                    pair: pair__,
                    protocol_address: protocol_address__.unwrap_or_default(),
                    base_amount: base_amount__.unwrap_or_default(),
                    max_trading_price: max_trading_price__.unwrap_or_default(),
                    slippage_tolerance: slippage_tolerance__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vaultron.enclave.cluster.v1.VaultronActionBuyAttributes", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VaultronActionSellAttributes {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pair.is_some() {
            len += 1;
        }
        if !self.protocol_address.is_empty() {
            len += 1;
        }
        if !self.trading_amount.is_empty() {
            len += 1;
        }
        if !self.min_trading_price.is_empty() {
            len += 1;
        }
        if !self.slippage_tolerance.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.cluster.v1.VaultronActionSellAttributes", len)?;
        if let Some(v) = self.pair.as_ref() {
            struct_ser.serialize_field("pair", v)?;
        }
        if !self.protocol_address.is_empty() {
            struct_ser.serialize_field("protocolAddress", pbjson::private::base64::encode(&self.protocol_address).as_str())?;
        }
        if !self.trading_amount.is_empty() {
            struct_ser.serialize_field("tradingAmount", &self.trading_amount)?;
        }
        if !self.min_trading_price.is_empty() {
            struct_ser.serialize_field("minTradingPrice", &self.min_trading_price)?;
        }
        if !self.slippage_tolerance.is_empty() {
            struct_ser.serialize_field("slippageTolerance", &self.slippage_tolerance)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VaultronActionSellAttributes {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pair",
            "protocol_address",
            "protocolAddress",
            "trading_amount",
            "tradingAmount",
            "min_trading_price",
            "minTradingPrice",
            "slippage_tolerance",
            "slippageTolerance",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pair,
            ProtocolAddress,
            TradingAmount,
            MinTradingPrice,
            SlippageTolerance,
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
                            "pair" => Ok(GeneratedField::Pair),
                            "protocolAddress" | "protocol_address" => Ok(GeneratedField::ProtocolAddress),
                            "tradingAmount" | "trading_amount" => Ok(GeneratedField::TradingAmount),
                            "minTradingPrice" | "min_trading_price" => Ok(GeneratedField::MinTradingPrice),
                            "slippageTolerance" | "slippage_tolerance" => Ok(GeneratedField::SlippageTolerance),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VaultronActionSellAttributes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.enclave.cluster.v1.VaultronActionSellAttributes")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VaultronActionSellAttributes, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pair__ = None;
                let mut protocol_address__ = None;
                let mut trading_amount__ = None;
                let mut min_trading_price__ = None;
                let mut slippage_tolerance__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Pair => {
                            if pair__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pair"));
                            }
                            pair__ = map.next_value()?;
                        }
                        GeneratedField::ProtocolAddress => {
                            if protocol_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocolAddress"));
                            }
                            protocol_address__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TradingAmount => {
                            if trading_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tradingAmount"));
                            }
                            trading_amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::MinTradingPrice => {
                            if min_trading_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minTradingPrice"));
                            }
                            min_trading_price__ = Some(map.next_value()?);
                        }
                        GeneratedField::SlippageTolerance => {
                            if slippage_tolerance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slippageTolerance"));
                            }
                            slippage_tolerance__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(VaultronActionSellAttributes {
                    pair: pair__,
                    protocol_address: protocol_address__.unwrap_or_default(),
                    trading_amount: trading_amount__.unwrap_or_default(),
                    min_trading_price: min_trading_price__.unwrap_or_default(),
                    slippage_tolerance: slippage_tolerance__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vaultron.enclave.cluster.v1.VaultronActionSellAttributes", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VaultronActionStakeAttributes {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.token.is_some() {
            len += 1;
        }
        if !self.protocol_address.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        if !self.max_gas_fee.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.cluster.v1.VaultronActionStakeAttributes", len)?;
        if let Some(v) = self.token.as_ref() {
            struct_ser.serialize_field("token", v)?;
        }
        if !self.protocol_address.is_empty() {
            struct_ser.serialize_field("protocolAddress", pbjson::private::base64::encode(&self.protocol_address).as_str())?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if !self.max_gas_fee.is_empty() {
            struct_ser.serialize_field("maxGasFee", &self.max_gas_fee)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VaultronActionStakeAttributes {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "token",
            "protocol_address",
            "protocolAddress",
            "amount",
            "max_gas_fee",
            "maxGasFee",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Token,
            ProtocolAddress,
            Amount,
            MaxGasFee,
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
                            "token" => Ok(GeneratedField::Token),
                            "protocolAddress" | "protocol_address" => Ok(GeneratedField::ProtocolAddress),
                            "amount" => Ok(GeneratedField::Amount),
                            "maxGasFee" | "max_gas_fee" => Ok(GeneratedField::MaxGasFee),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VaultronActionStakeAttributes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.enclave.cluster.v1.VaultronActionStakeAttributes")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VaultronActionStakeAttributes, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut token__ = None;
                let mut protocol_address__ = None;
                let mut amount__ = None;
                let mut max_gas_fee__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Token => {
                            if token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("token"));
                            }
                            token__ = map.next_value()?;
                        }
                        GeneratedField::ProtocolAddress => {
                            if protocol_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocolAddress"));
                            }
                            protocol_address__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxGasFee => {
                            if max_gas_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxGasFee"));
                            }
                            max_gas_fee__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(VaultronActionStakeAttributes {
                    token: token__,
                    protocol_address: protocol_address__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    max_gas_fee: max_gas_fee__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vaultron.enclave.cluster.v1.VaultronActionStakeAttributes", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VaultronActionToken {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.endpoint.is_some() {
            len += 1;
        }
        if !self.symbol.is_empty() {
            len += 1;
        }
        if !self.token_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.cluster.v1.VaultronActionToken", len)?;
        if let Some(v) = self.endpoint.as_ref() {
            struct_ser.serialize_field("endpoint", v)?;
        }
        if !self.symbol.is_empty() {
            struct_ser.serialize_field("symbol", &self.symbol)?;
        }
        if !self.token_address.is_empty() {
            struct_ser.serialize_field("tokenAddress", &self.token_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VaultronActionToken {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "endpoint",
            "symbol",
            "token_address",
            "tokenAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Endpoint,
            Symbol,
            TokenAddress,
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
                            "endpoint" => Ok(GeneratedField::Endpoint),
                            "symbol" => Ok(GeneratedField::Symbol),
                            "tokenAddress" | "token_address" => Ok(GeneratedField::TokenAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VaultronActionToken;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.enclave.cluster.v1.VaultronActionToken")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VaultronActionToken, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut endpoint__ = None;
                let mut symbol__ = None;
                let mut token_address__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Endpoint => {
                            if endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpoint"));
                            }
                            endpoint__ = map.next_value()?;
                        }
                        GeneratedField::Symbol => {
                            if symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("symbol"));
                            }
                            symbol__ = Some(map.next_value()?);
                        }
                        GeneratedField::TokenAddress => {
                            if token_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenAddress"));
                            }
                            token_address__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(VaultronActionToken {
                    endpoint: endpoint__,
                    symbol: symbol__.unwrap_or_default(),
                    token_address: token_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vaultron.enclave.cluster.v1.VaultronActionToken", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VaultronActionTokenPair {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.endpoint.is_some() {
            len += 1;
        }
        if !self.base_token_symbol.is_empty() {
            len += 1;
        }
        if !self.trading_token_symbol.is_empty() {
            len += 1;
        }
        if !self.base_token_address.is_empty() {
            len += 1;
        }
        if !self.trading_token_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.cluster.v1.VaultronActionTokenPair", len)?;
        if let Some(v) = self.endpoint.as_ref() {
            struct_ser.serialize_field("endpoint", v)?;
        }
        if !self.base_token_symbol.is_empty() {
            struct_ser.serialize_field("baseTokenSymbol", &self.base_token_symbol)?;
        }
        if !self.trading_token_symbol.is_empty() {
            struct_ser.serialize_field("tradingTokenSymbol", &self.trading_token_symbol)?;
        }
        if !self.base_token_address.is_empty() {
            struct_ser.serialize_field("baseTokenAddress", &self.base_token_address)?;
        }
        if !self.trading_token_address.is_empty() {
            struct_ser.serialize_field("tradingTokenAddress", &self.trading_token_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VaultronActionTokenPair {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "endpoint",
            "base_token_symbol",
            "baseTokenSymbol",
            "trading_token_symbol",
            "tradingTokenSymbol",
            "base_token_address",
            "baseTokenAddress",
            "trading_token_address",
            "tradingTokenAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Endpoint,
            BaseTokenSymbol,
            TradingTokenSymbol,
            BaseTokenAddress,
            TradingTokenAddress,
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
                            "endpoint" => Ok(GeneratedField::Endpoint),
                            "baseTokenSymbol" | "base_token_symbol" => Ok(GeneratedField::BaseTokenSymbol),
                            "tradingTokenSymbol" | "trading_token_symbol" => Ok(GeneratedField::TradingTokenSymbol),
                            "baseTokenAddress" | "base_token_address" => Ok(GeneratedField::BaseTokenAddress),
                            "tradingTokenAddress" | "trading_token_address" => Ok(GeneratedField::TradingTokenAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VaultronActionTokenPair;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.enclave.cluster.v1.VaultronActionTokenPair")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VaultronActionTokenPair, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut endpoint__ = None;
                let mut base_token_symbol__ = None;
                let mut trading_token_symbol__ = None;
                let mut base_token_address__ = None;
                let mut trading_token_address__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Endpoint => {
                            if endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpoint"));
                            }
                            endpoint__ = map.next_value()?;
                        }
                        GeneratedField::BaseTokenSymbol => {
                            if base_token_symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseTokenSymbol"));
                            }
                            base_token_symbol__ = Some(map.next_value()?);
                        }
                        GeneratedField::TradingTokenSymbol => {
                            if trading_token_symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tradingTokenSymbol"));
                            }
                            trading_token_symbol__ = Some(map.next_value()?);
                        }
                        GeneratedField::BaseTokenAddress => {
                            if base_token_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseTokenAddress"));
                            }
                            base_token_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::TradingTokenAddress => {
                            if trading_token_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tradingTokenAddress"));
                            }
                            trading_token_address__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(VaultronActionTokenPair {
                    endpoint: endpoint__,
                    base_token_symbol: base_token_symbol__.unwrap_or_default(),
                    trading_token_symbol: trading_token_symbol__.unwrap_or_default(),
                    base_token_address: base_token_address__.unwrap_or_default(),
                    trading_token_address: trading_token_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vaultron.enclave.cluster.v1.VaultronActionTokenPair", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VaultronActionTransferAttributes {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.token.is_some() {
            len += 1;
        }
        if !self.to.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        if self.max_gas_fee.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.cluster.v1.VaultronActionTransferAttributes", len)?;
        if let Some(v) = self.token.as_ref() {
            struct_ser.serialize_field("token", v)?;
        }
        if !self.to.is_empty() {
            struct_ser.serialize_field("to", pbjson::private::base64::encode(&self.to).as_str())?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if let Some(v) = self.max_gas_fee.as_ref() {
            struct_ser.serialize_field("maxGasFee", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VaultronActionTransferAttributes {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "token",
            "to",
            "amount",
            "max_gas_fee",
            "maxGasFee",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Token,
            To,
            Amount,
            MaxGasFee,
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
                            "token" => Ok(GeneratedField::Token),
                            "to" => Ok(GeneratedField::To),
                            "amount" => Ok(GeneratedField::Amount),
                            "maxGasFee" | "max_gas_fee" => Ok(GeneratedField::MaxGasFee),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VaultronActionTransferAttributes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.enclave.cluster.v1.VaultronActionTransferAttributes")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VaultronActionTransferAttributes, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut token__ = None;
                let mut to__ = None;
                let mut amount__ = None;
                let mut max_gas_fee__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Token => {
                            if token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("token"));
                            }
                            token__ = map.next_value()?;
                        }
                        GeneratedField::To => {
                            if to__.is_some() {
                                return Err(serde::de::Error::duplicate_field("to"));
                            }
                            to__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxGasFee => {
                            if max_gas_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxGasFee"));
                            }
                            max_gas_fee__ = map.next_value()?;
                        }
                    }
                }
                Ok(VaultronActionTransferAttributes {
                    token: token__,
                    to: to__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    max_gas_fee: max_gas_fee__,
                })
            }
        }
        deserializer.deserialize_struct("vaultron.enclave.cluster.v1.VaultronActionTransferAttributes", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VaultronActionUnstakeAttributes {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.token.is_some() {
            len += 1;
        }
        if !self.protocol_address.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        if !self.max_gas_fee.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.cluster.v1.VaultronActionUnstakeAttributes", len)?;
        if let Some(v) = self.token.as_ref() {
            struct_ser.serialize_field("token", v)?;
        }
        if !self.protocol_address.is_empty() {
            struct_ser.serialize_field("protocolAddress", pbjson::private::base64::encode(&self.protocol_address).as_str())?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if !self.max_gas_fee.is_empty() {
            struct_ser.serialize_field("maxGasFee", &self.max_gas_fee)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VaultronActionUnstakeAttributes {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "token",
            "protocol_address",
            "protocolAddress",
            "amount",
            "max_gas_fee",
            "maxGasFee",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Token,
            ProtocolAddress,
            Amount,
            MaxGasFee,
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
                            "token" => Ok(GeneratedField::Token),
                            "protocolAddress" | "protocol_address" => Ok(GeneratedField::ProtocolAddress),
                            "amount" => Ok(GeneratedField::Amount),
                            "maxGasFee" | "max_gas_fee" => Ok(GeneratedField::MaxGasFee),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VaultronActionUnstakeAttributes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.enclave.cluster.v1.VaultronActionUnstakeAttributes")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VaultronActionUnstakeAttributes, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut token__ = None;
                let mut protocol_address__ = None;
                let mut amount__ = None;
                let mut max_gas_fee__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Token => {
                            if token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("token"));
                            }
                            token__ = map.next_value()?;
                        }
                        GeneratedField::ProtocolAddress => {
                            if protocol_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocolAddress"));
                            }
                            protocol_address__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxGasFee => {
                            if max_gas_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxGasFee"));
                            }
                            max_gas_fee__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(VaultronActionUnstakeAttributes {
                    token: token__,
                    protocol_address: protocol_address__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    max_gas_fee: max_gas_fee__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vaultron.enclave.cluster.v1.VaultronActionUnstakeAttributes", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VaultronCreationAttributes {
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
        if !self.vaultron_name.is_empty() {
            len += 1;
        }
        if self.cluster.is_some() {
            len += 1;
        }
        if self.signature_type != 0 {
            len += 1;
        }
        if !self.public_key.is_empty() {
            len += 1;
        }
        if !self.recovery_public_key.is_empty() {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.cluster.v1.VaultronCreationAttributes", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.vaultron_name.is_empty() {
            struct_ser.serialize_field("vaultronName", &self.vaultron_name)?;
        }
        if let Some(v) = self.cluster.as_ref() {
            struct_ser.serialize_field("cluster", v)?;
        }
        if self.signature_type != 0 {
            let v = VaultronUserSignatureType::from_i32(self.signature_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.signature_type)))?;
            struct_ser.serialize_field("signatureType", &v)?;
        }
        if !self.public_key.is_empty() {
            struct_ser.serialize_field("publicKey", pbjson::private::base64::encode(&self.public_key).as_str())?;
        }
        if !self.recovery_public_key.is_empty() {
            struct_ser.serialize_field("recoveryPublicKey", pbjson::private::base64::encode(&self.recovery_public_key).as_str())?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", pbjson::private::base64::encode(&self.message).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VaultronCreationAttributes {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "vaultron_name",
            "vaultronName",
            "cluster",
            "signature_type",
            "signatureType",
            "public_key",
            "publicKey",
            "recovery_public_key",
            "recoveryPublicKey",
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            VaultronName,
            Cluster,
            SignatureType,
            PublicKey,
            RecoveryPublicKey,
            Message,
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
                            "vaultronName" | "vaultron_name" => Ok(GeneratedField::VaultronName),
                            "cluster" => Ok(GeneratedField::Cluster),
                            "signatureType" | "signature_type" => Ok(GeneratedField::SignatureType),
                            "publicKey" | "public_key" => Ok(GeneratedField::PublicKey),
                            "recoveryPublicKey" | "recovery_public_key" => Ok(GeneratedField::RecoveryPublicKey),
                            "message" => Ok(GeneratedField::Message),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VaultronCreationAttributes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.enclave.cluster.v1.VaultronCreationAttributes")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VaultronCreationAttributes, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut vaultron_name__ = None;
                let mut cluster__ = None;
                let mut signature_type__ = None;
                let mut public_key__ = None;
                let mut recovery_public_key__ = None;
                let mut message__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::VaultronName => {
                            if vaultron_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vaultronName"));
                            }
                            vaultron_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Cluster => {
                            if cluster__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cluster"));
                            }
                            cluster__ = map.next_value()?;
                        }
                        GeneratedField::SignatureType => {
                            if signature_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signatureType"));
                            }
                            signature_type__ = Some(map.next_value::<VaultronUserSignatureType>()? as i32);
                        }
                        GeneratedField::PublicKey => {
                            if public_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicKey"));
                            }
                            public_key__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RecoveryPublicKey => {
                            if recovery_public_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recoveryPublicKey"));
                            }
                            recovery_public_key__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(VaultronCreationAttributes {
                    user_id: user_id__.unwrap_or_default(),
                    vaultron_name: vaultron_name__.unwrap_or_default(),
                    cluster: cluster__,
                    signature_type: signature_type__.unwrap_or_default(),
                    public_key: public_key__.unwrap_or_default(),
                    recovery_public_key: recovery_public_key__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vaultron.enclave.cluster.v1.VaultronCreationAttributes", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VaultronEncryptedAttributes {
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
        if self.signature_type != 0 {
            len += 1;
        }
        if !self.user_public_key.is_empty() {
            len += 1;
        }
        if !self.user_wallet_seed.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.enclave.cluster.v1.VaultronEncryptedAttributes", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", pbjson::private::base64::encode(&self.user_id).as_str())?;
        }
        if self.signature_type != 0 {
            let v = VaultronUserSignatureType::from_i32(self.signature_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.signature_type)))?;
            struct_ser.serialize_field("signatureType", &v)?;
        }
        if !self.user_public_key.is_empty() {
            struct_ser.serialize_field("userPublicKey", pbjson::private::base64::encode(&self.user_public_key).as_str())?;
        }
        if !self.user_wallet_seed.is_empty() {
            struct_ser.serialize_field("userWalletSeed", pbjson::private::base64::encode(&self.user_wallet_seed).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VaultronEncryptedAttributes {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "signature_type",
            "signatureType",
            "user_public_key",
            "userPublicKey",
            "user_wallet_seed",
            "userWalletSeed",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            SignatureType,
            UserPublicKey,
            UserWalletSeed,
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
                            "signatureType" | "signature_type" => Ok(GeneratedField::SignatureType),
                            "userPublicKey" | "user_public_key" => Ok(GeneratedField::UserPublicKey),
                            "userWalletSeed" | "user_wallet_seed" => Ok(GeneratedField::UserWalletSeed),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VaultronEncryptedAttributes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.enclave.cluster.v1.VaultronEncryptedAttributes")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VaultronEncryptedAttributes, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut signature_type__ = None;
                let mut user_public_key__ = None;
                let mut user_wallet_seed__ = None;
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
                        GeneratedField::SignatureType => {
                            if signature_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signatureType"));
                            }
                            signature_type__ = Some(map.next_value::<VaultronUserSignatureType>()? as i32);
                        }
                        GeneratedField::UserPublicKey => {
                            if user_public_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userPublicKey"));
                            }
                            user_public_key__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::UserWalletSeed => {
                            if user_wallet_seed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userWalletSeed"));
                            }
                            user_wallet_seed__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(VaultronEncryptedAttributes {
                    user_id: user_id__.unwrap_or_default(),
                    signature_type: signature_type__.unwrap_or_default(),
                    user_public_key: user_public_key__.unwrap_or_default(),
                    user_wallet_seed: user_wallet_seed__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vaultron.enclave.cluster.v1.VaultronEncryptedAttributes", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VaultronUserSignatureType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "VAULTRON_USER_SIGNATURE_TYPE_UNSPECIFIED",
            Self::Fairx => "VAULTRON_USER_SIGNATURE_TYPE_FAIRX",
            Self::WalletBitcoin => "VAULTRON_USER_SIGNATURE_TYPE_WALLET_BITCOIN",
            Self::WalletEth => "VAULTRON_USER_SIGNATURE_TYPE_WALLET_ETH",
            Self::WalletSolana => "VAULTRON_USER_SIGNATURE_TYPE_WALLET_SOLANA",
            Self::WalletSui => "VAULTRON_USER_SIGNATURE_TYPE_WALLET_SUI",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for VaultronUserSignatureType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "VAULTRON_USER_SIGNATURE_TYPE_UNSPECIFIED",
            "VAULTRON_USER_SIGNATURE_TYPE_FAIRX",
            "VAULTRON_USER_SIGNATURE_TYPE_WALLET_BITCOIN",
            "VAULTRON_USER_SIGNATURE_TYPE_WALLET_ETH",
            "VAULTRON_USER_SIGNATURE_TYPE_WALLET_SOLANA",
            "VAULTRON_USER_SIGNATURE_TYPE_WALLET_SUI",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VaultronUserSignatureType;

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
                    .and_then(VaultronUserSignatureType::from_i32)
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
                    .and_then(VaultronUserSignatureType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "VAULTRON_USER_SIGNATURE_TYPE_UNSPECIFIED" => Ok(VaultronUserSignatureType::Unspecified),
                    "VAULTRON_USER_SIGNATURE_TYPE_FAIRX" => Ok(VaultronUserSignatureType::Fairx),
                    "VAULTRON_USER_SIGNATURE_TYPE_WALLET_BITCOIN" => Ok(VaultronUserSignatureType::WalletBitcoin),
                    "VAULTRON_USER_SIGNATURE_TYPE_WALLET_ETH" => Ok(VaultronUserSignatureType::WalletEth),
                    "VAULTRON_USER_SIGNATURE_TYPE_WALLET_SOLANA" => Ok(VaultronUserSignatureType::WalletSolana),
                    "VAULTRON_USER_SIGNATURE_TYPE_WALLET_SUI" => Ok(VaultronUserSignatureType::WalletSui),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
