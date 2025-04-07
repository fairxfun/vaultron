// @generated
impl serde::Serialize for AttestationDocument {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.module_id.is_empty() {
            len += 1;
        }
        if self.digest != 0 {
            len += 1;
        }
        if self.timestamp != 0 {
            len += 1;
        }
        if !self.pcrs.is_empty() {
            len += 1;
        }
        if !self.certificate.is_empty() {
            len += 1;
        }
        if !self.cabundle.is_empty() {
            len += 1;
        }
        if !self.public_key.is_empty() {
            len += 1;
        }
        if !self.nonce.is_empty() {
            len += 1;
        }
        if self.user_data.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.attestation.v1.AttestationDocument", len)?;
        if !self.module_id.is_empty() {
            struct_ser.serialize_field("moduleId", &self.module_id)?;
        }
        if self.digest != 0 {
            let v = Digest::from_i32(self.digest)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.digest)))?;
            struct_ser.serialize_field("digest", &v)?;
        }
        if self.timestamp != 0 {
            struct_ser.serialize_field("timestamp", ToString::to_string(&self.timestamp).as_str())?;
        }
        if !self.pcrs.is_empty() {
            let v: std::collections::HashMap<_, _> = self.pcrs.iter()
                .map(|(k, v)| (k, pbjson::private::base64::encode(v))).collect();
            struct_ser.serialize_field("pcrs", &v)?;
        }
        if !self.certificate.is_empty() {
            struct_ser.serialize_field("certificate", pbjson::private::base64::encode(&self.certificate).as_str())?;
        }
        if !self.cabundle.is_empty() {
            struct_ser.serialize_field("cabundle", &self.cabundle.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        if !self.public_key.is_empty() {
            struct_ser.serialize_field("publicKey", pbjson::private::base64::encode(&self.public_key).as_str())?;
        }
        if !self.nonce.is_empty() {
            struct_ser.serialize_field("nonce", pbjson::private::base64::encode(&self.nonce).as_str())?;
        }
        if let Some(v) = self.user_data.as_ref() {
            struct_ser.serialize_field("userData", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AttestationDocument {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "module_id",
            "moduleId",
            "digest",
            "timestamp",
            "pcrs",
            "certificate",
            "cabundle",
            "public_key",
            "publicKey",
            "nonce",
            "user_data",
            "userData",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ModuleId,
            Digest,
            Timestamp,
            Pcrs,
            Certificate,
            Cabundle,
            PublicKey,
            Nonce,
            UserData,
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
                            "moduleId" | "module_id" => Ok(GeneratedField::ModuleId),
                            "digest" => Ok(GeneratedField::Digest),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "pcrs" => Ok(GeneratedField::Pcrs),
                            "certificate" => Ok(GeneratedField::Certificate),
                            "cabundle" => Ok(GeneratedField::Cabundle),
                            "publicKey" | "public_key" => Ok(GeneratedField::PublicKey),
                            "nonce" => Ok(GeneratedField::Nonce),
                            "userData" | "user_data" => Ok(GeneratedField::UserData),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AttestationDocument;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.attestation.v1.AttestationDocument")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AttestationDocument, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut module_id__ = None;
                let mut digest__ = None;
                let mut timestamp__ = None;
                let mut pcrs__ = None;
                let mut certificate__ = None;
                let mut cabundle__ = None;
                let mut public_key__ = None;
                let mut nonce__ = None;
                let mut user_data__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ModuleId => {
                            if module_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("moduleId"));
                            }
                            module_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Digest => {
                            if digest__.is_some() {
                                return Err(serde::de::Error::duplicate_field("digest"));
                            }
                            digest__ = Some(map.next_value::<Digest>()? as i32);
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Pcrs => {
                            if pcrs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pcrs"));
                            }
                            pcrs__ = Some(
                                map.next_value::<std::collections::HashMap<::pbjson::private::NumberDeserialize<u32>, ::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|(k,v)| (k.0, v.0)).collect()
                            );
                        }
                        GeneratedField::Certificate => {
                            if certificate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("certificate"));
                            }
                            certificate__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Cabundle => {
                            if cabundle__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cabundle"));
                            }
                            cabundle__ = 
                                Some(map.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::PublicKey => {
                            if public_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicKey"));
                            }
                            public_key__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Nonce => {
                            if nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonce"));
                            }
                            nonce__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::UserData => {
                            if user_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userData"));
                            }
                            user_data__ = map.next_value()?;
                        }
                    }
                }
                Ok(AttestationDocument {
                    module_id: module_id__.unwrap_or_default(),
                    digest: digest__.unwrap_or_default(),
                    timestamp: timestamp__.unwrap_or_default(),
                    pcrs: pcrs__.unwrap_or_default(),
                    certificate: certificate__.unwrap_or_default(),
                    cabundle: cabundle__.unwrap_or_default(),
                    public_key: public_key__.unwrap_or_default(),
                    nonce: nonce__.unwrap_or_default(),
                    user_data: user_data__,
                })
            }
        }
        deserializer.deserialize_struct("vaultron.attestation.v1.AttestationDocument", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AttestationDocumentUserData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.request.is_empty() {
            len += 1;
        }
        if !self.response.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.attestation.v1.AttestationDocumentUserData", len)?;
        if !self.request.is_empty() {
            struct_ser.serialize_field("request", pbjson::private::base64::encode(&self.request).as_str())?;
        }
        if !self.response.is_empty() {
            struct_ser.serialize_field("response", pbjson::private::base64::encode(&self.response).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AttestationDocumentUserData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request",
            "response",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Request,
            Response,
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
                            "request" => Ok(GeneratedField::Request),
                            "response" => Ok(GeneratedField::Response),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AttestationDocumentUserData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.attestation.v1.AttestationDocumentUserData")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AttestationDocumentUserData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request__ = None;
                let mut response__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Request => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("request"));
                            }
                            request__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Response => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("response"));
                            }
                            response__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(AttestationDocumentUserData {
                    request: request__.unwrap_or_default(),
                    response: response__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vaultron.attestation.v1.AttestationDocumentUserData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Digest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "DIGEST_UNSPECIFIED",
            Self::Sha256 => "DIGEST_SHA256",
            Self::Sha384 => "DIGEST_SHA384",
            Self::Sha512 => "DIGEST_SHA512",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Digest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DIGEST_UNSPECIFIED",
            "DIGEST_SHA256",
            "DIGEST_SHA384",
            "DIGEST_SHA512",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Digest;

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
                    .and_then(Digest::from_i32)
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
                    .and_then(Digest::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "DIGEST_UNSPECIFIED" => Ok(Digest::Unspecified),
                    "DIGEST_SHA256" => Ok(Digest::Sha256),
                    "DIGEST_SHA384" => Ok(Digest::Sha384),
                    "DIGEST_SHA512" => Ok(Digest::Sha512),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
