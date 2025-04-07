// @generated
impl serde::Serialize for ClusterData {
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
        if !self.cluster_public_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.cluster.v1.ClusterData", len)?;
        if !self.enclave_pcr0.is_empty() {
            struct_ser.serialize_field("enclavePcr0", pbjson::private::base64::encode(&self.enclave_pcr0).as_str())?;
        }
        if !self.cluster_public_key.is_empty() {
            struct_ser.serialize_field("clusterPublicKey", pbjson::private::base64::encode(&self.cluster_public_key).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClusterData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "enclave_pcr0",
            "enclavePcr0",
            "cluster_public_key",
            "clusterPublicKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = ClusterData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.cluster.v1.ClusterData")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ClusterData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut enclave_pcr0__ = None;
                let mut cluster_public_key__ = None;
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
                Ok(ClusterData {
                    enclave_pcr0: enclave_pcr0__.unwrap_or_default(),
                    cluster_public_key: cluster_public_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vaultron.cluster.v1.ClusterData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateUserWalletData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.cluster_data.is_some() {
            len += 1;
        }
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
        let mut struct_ser = serializer.serialize_struct("vaultron.cluster.v1.CreateUserWalletData", len)?;
        if let Some(v) = self.cluster_data.as_ref() {
            struct_ser.serialize_field("clusterData", v)?;
        }
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
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateUserWalletData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cluster_data",
            "clusterData",
            "user_id",
            "userId",
            "user_public_key",
            "userPublicKey",
            "signature_type",
            "signatureType",
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClusterData,
            UserId,
            UserPublicKey,
            SignatureType,
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
                            "clusterData" | "cluster_data" => Ok(GeneratedField::ClusterData),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "userPublicKey" | "user_public_key" => Ok(GeneratedField::UserPublicKey),
                            "signatureType" | "signature_type" => Ok(GeneratedField::SignatureType),
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
            type Value = CreateUserWalletData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.cluster.v1.CreateUserWalletData")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CreateUserWalletData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cluster_data__ = None;
                let mut user_id__ = None;
                let mut user_public_key__ = None;
                let mut signature_type__ = None;
                let mut message__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClusterData => {
                            if cluster_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterData"));
                            }
                            cluster_data__ = map.next_value()?;
                        }
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
                    }
                }
                Ok(CreateUserWalletData {
                    cluster_data: cluster_data__,
                    user_id: user_id__.unwrap_or_default(),
                    user_public_key: user_public_key__.unwrap_or_default(),
                    signature_type: signature_type__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vaultron.cluster.v1.CreateUserWalletData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateUserWalletRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.data.is_some() {
            len += 1;
        }
        if !self.signature.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.cluster.v1.CreateUserWalletRequest", len)?;
        if let Some(v) = self.data.as_ref() {
            struct_ser.serialize_field("data", v)?;
        }
        if !self.signature.is_empty() {
            struct_ser.serialize_field("signature", pbjson::private::base64::encode(&self.signature).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateUserWalletRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "data",
            "signature",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Data,
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
                            "data" => Ok(GeneratedField::Data),
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
            type Value = CreateUserWalletRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.cluster.v1.CreateUserWalletRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CreateUserWalletRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut data__ = None;
                let mut signature__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = map.next_value()?;
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
                Ok(CreateUserWalletRequest {
                    data: data__,
                    signature: signature__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vaultron.cluster.v1.CreateUserWalletRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateUserWalletResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
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
        let mut struct_ser = serializer.serialize_struct("vaultron.cluster.v1.CreateUserWalletResponse", len)?;
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
impl<'de> serde::Deserialize<'de> for CreateUserWalletResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
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
            type Value = CreateUserWalletResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.cluster.v1.CreateUserWalletResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CreateUserWalletResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut wallet_encrypted_data__ = None;
                let mut eth_public_key__ = None;
                let mut eth_encrypted_data__ = None;
                let mut solana_public_key__ = None;
                let mut solana_encrypted_data__ = None;
                let mut sui_public_key__ = None;
                let mut sui_encrypted_data__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
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
                Ok(CreateUserWalletResponse {
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
        deserializer.deserialize_struct("vaultron.cluster.v1.CreateUserWalletResponse", FIELDS, GeneratedVisitor)
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
