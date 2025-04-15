// @generated
impl serde::Serialize for DescribeEnclaveInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.enclave_name.is_empty() {
            len += 1;
        }
        if !self.enclave_id.is_empty() {
            len += 1;
        }
        if self.enclave_cid != 0 {
            len += 1;
        }
        if self.number_of_cpus != 0 {
            len += 1;
        }
        if !self.cpu_ids.is_empty() {
            len += 1;
        }
        if self.memory_mib != 0 {
            len += 1;
        }
        if !self.state.is_empty() {
            len += 1;
        }
        if !self.flags.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.agent.v1.DescribeEnclaveInfo", len)?;
        if !self.enclave_name.is_empty() {
            struct_ser.serialize_field("enclaveName", &self.enclave_name)?;
        }
        if !self.enclave_id.is_empty() {
            struct_ser.serialize_field("enclaveId", &self.enclave_id)?;
        }
        if self.enclave_cid != 0 {
            struct_ser.serialize_field("enclaveCid", &self.enclave_cid)?;
        }
        if self.number_of_cpus != 0 {
            struct_ser.serialize_field("numberOfCpus", &self.number_of_cpus)?;
        }
        if !self.cpu_ids.is_empty() {
            struct_ser.serialize_field("cpuIds", &self.cpu_ids)?;
        }
        if self.memory_mib != 0 {
            struct_ser.serialize_field("memoryMib", &self.memory_mib)?;
        }
        if !self.state.is_empty() {
            struct_ser.serialize_field("state", &self.state)?;
        }
        if !self.flags.is_empty() {
            struct_ser.serialize_field("flags", &self.flags)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DescribeEnclaveInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "enclave_name",
            "enclaveName",
            "enclave_id",
            "enclaveId",
            "enclave_cid",
            "enclaveCid",
            "number_of_cpus",
            "numberOfCpus",
            "cpu_ids",
            "cpuIds",
            "memory_mib",
            "memoryMib",
            "state",
            "flags",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EnclaveName,
            EnclaveId,
            EnclaveCid,
            NumberOfCpus,
            CpuIds,
            MemoryMib,
            State,
            Flags,
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
                            "enclaveName" | "enclave_name" => Ok(GeneratedField::EnclaveName),
                            "enclaveId" | "enclave_id" => Ok(GeneratedField::EnclaveId),
                            "enclaveCid" | "enclave_cid" => Ok(GeneratedField::EnclaveCid),
                            "numberOfCpus" | "number_of_cpus" => Ok(GeneratedField::NumberOfCpus),
                            "cpuIds" | "cpu_ids" => Ok(GeneratedField::CpuIds),
                            "memoryMib" | "memory_mib" => Ok(GeneratedField::MemoryMib),
                            "state" => Ok(GeneratedField::State),
                            "flags" => Ok(GeneratedField::Flags),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DescribeEnclaveInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.agent.v1.DescribeEnclaveInfo")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DescribeEnclaveInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut enclave_name__ = None;
                let mut enclave_id__ = None;
                let mut enclave_cid__ = None;
                let mut number_of_cpus__ = None;
                let mut cpu_ids__ = None;
                let mut memory_mib__ = None;
                let mut state__ = None;
                let mut flags__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::EnclaveName => {
                            if enclave_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enclaveName"));
                            }
                            enclave_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::EnclaveId => {
                            if enclave_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enclaveId"));
                            }
                            enclave_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::EnclaveCid => {
                            if enclave_cid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enclaveCid"));
                            }
                            enclave_cid__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NumberOfCpus => {
                            if number_of_cpus__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numberOfCpus"));
                            }
                            number_of_cpus__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CpuIds => {
                            if cpu_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cpuIds"));
                            }
                            cpu_ids__ = 
                                Some(map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::MemoryMib => {
                            if memory_mib__.is_some() {
                                return Err(serde::de::Error::duplicate_field("memoryMib"));
                            }
                            memory_mib__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map.next_value()?);
                        }
                        GeneratedField::Flags => {
                            if flags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("flags"));
                            }
                            flags__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(DescribeEnclaveInfo {
                    enclave_name: enclave_name__.unwrap_or_default(),
                    enclave_id: enclave_id__.unwrap_or_default(),
                    enclave_cid: enclave_cid__.unwrap_or_default(),
                    number_of_cpus: number_of_cpus__.unwrap_or_default(),
                    cpu_ids: cpu_ids__.unwrap_or_default(),
                    memory_mib: memory_mib__.unwrap_or_default(),
                    state: state__.unwrap_or_default(),
                    flags: flags__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("vaultron.agent.v1.DescribeEnclaveInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DescribeEnclaveRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("vaultron.agent.v1.DescribeEnclaveRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DescribeEnclaveRequest {
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
            type Value = DescribeEnclaveRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.agent.v1.DescribeEnclaveRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DescribeEnclaveRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DescribeEnclaveRequest {
                })
            }
        }
        deserializer.deserialize_struct("vaultron.agent.v1.DescribeEnclaveRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DescribeEnclaveResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.info.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.agent.v1.DescribeEnclaveResponse", len)?;
        if let Some(v) = self.info.as_ref() {
            struct_ser.serialize_field("info", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DescribeEnclaveResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "info",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Info,
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
                            "info" => Ok(GeneratedField::Info),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DescribeEnclaveResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.agent.v1.DescribeEnclaveResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DescribeEnclaveResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut info__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Info => {
                            if info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("info"));
                            }
                            info__ = map.next_value()?;
                        }
                    }
                }
                Ok(DescribeEnclaveResponse {
                    info: info__,
                })
            }
        }
        deserializer.deserialize_struct("vaultron.agent.v1.DescribeEnclaveResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EnclaveAgentRequest {
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
        let mut struct_ser = serializer.serialize_struct("vaultron.agent.v1.EnclaveAgentRequest", len)?;
        if let Some(v) = self.request.as_ref() {
            match v {
                enclave_agent_request::Request::StartRequest(v) => {
                    struct_ser.serialize_field("startRequest", v)?;
                }
                enclave_agent_request::Request::StopRequest(v) => {
                    struct_ser.serialize_field("stopRequest", v)?;
                }
                enclave_agent_request::Request::RestartRequest(v) => {
                    struct_ser.serialize_field("restartRequest", v)?;
                }
                enclave_agent_request::Request::DescribeRequest(v) => {
                    struct_ser.serialize_field("describeRequest", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EnclaveAgentRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "start_request",
            "startRequest",
            "stop_request",
            "stopRequest",
            "restart_request",
            "restartRequest",
            "describe_request",
            "describeRequest",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StartRequest,
            StopRequest,
            RestartRequest,
            DescribeRequest,
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
                            "startRequest" | "start_request" => Ok(GeneratedField::StartRequest),
                            "stopRequest" | "stop_request" => Ok(GeneratedField::StopRequest),
                            "restartRequest" | "restart_request" => Ok(GeneratedField::RestartRequest),
                            "describeRequest" | "describe_request" => Ok(GeneratedField::DescribeRequest),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EnclaveAgentRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.agent.v1.EnclaveAgentRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EnclaveAgentRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::StartRequest => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startRequest"));
                            }
                            request__ = map.next_value::<::std::option::Option<_>>()?.map(enclave_agent_request::Request::StartRequest)
;
                        }
                        GeneratedField::StopRequest => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stopRequest"));
                            }
                            request__ = map.next_value::<::std::option::Option<_>>()?.map(enclave_agent_request::Request::StopRequest)
;
                        }
                        GeneratedField::RestartRequest => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("restartRequest"));
                            }
                            request__ = map.next_value::<::std::option::Option<_>>()?.map(enclave_agent_request::Request::RestartRequest)
;
                        }
                        GeneratedField::DescribeRequest => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("describeRequest"));
                            }
                            request__ = map.next_value::<::std::option::Option<_>>()?.map(enclave_agent_request::Request::DescribeRequest)
;
                        }
                    }
                }
                Ok(EnclaveAgentRequest {
                    request: request__,
                })
            }
        }
        deserializer.deserialize_struct("vaultron.agent.v1.EnclaveAgentRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EnclaveAgentResponse {
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
        if self.response.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("vaultron.agent.v1.EnclaveAgentResponse", len)?;
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        if let Some(v) = self.response.as_ref() {
            match v {
                enclave_agent_response::Response::StartResponse(v) => {
                    struct_ser.serialize_field("startResponse", v)?;
                }
                enclave_agent_response::Response::StopResponse(v) => {
                    struct_ser.serialize_field("stopResponse", v)?;
                }
                enclave_agent_response::Response::RestartResponse(v) => {
                    struct_ser.serialize_field("restartResponse", v)?;
                }
                enclave_agent_response::Response::DescribeResponse(v) => {
                    struct_ser.serialize_field("describeResponse", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EnclaveAgentResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "start_response",
            "startResponse",
            "stop_response",
            "stopResponse",
            "restart_response",
            "restartResponse",
            "describe_response",
            "describeResponse",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            StartResponse,
            StopResponse,
            RestartResponse,
            DescribeResponse,
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
                            "startResponse" | "start_response" => Ok(GeneratedField::StartResponse),
                            "stopResponse" | "stop_response" => Ok(GeneratedField::StopResponse),
                            "restartResponse" | "restart_response" => Ok(GeneratedField::RestartResponse),
                            "describeResponse" | "describe_response" => Ok(GeneratedField::DescribeResponse),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EnclaveAgentResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.agent.v1.EnclaveAgentResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EnclaveAgentResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut response__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = map.next_value()?;
                        }
                        GeneratedField::StartResponse => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startResponse"));
                            }
                            response__ = map.next_value::<::std::option::Option<_>>()?.map(enclave_agent_response::Response::StartResponse)
;
                        }
                        GeneratedField::StopResponse => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stopResponse"));
                            }
                            response__ = map.next_value::<::std::option::Option<_>>()?.map(enclave_agent_response::Response::StopResponse)
;
                        }
                        GeneratedField::RestartResponse => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("restartResponse"));
                            }
                            response__ = map.next_value::<::std::option::Option<_>>()?.map(enclave_agent_response::Response::RestartResponse)
;
                        }
                        GeneratedField::DescribeResponse => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("describeResponse"));
                            }
                            response__ = map.next_value::<::std::option::Option<_>>()?.map(enclave_agent_response::Response::DescribeResponse)
;
                        }
                    }
                }
                Ok(EnclaveAgentResponse {
                    code: code__,
                    response: response__,
                })
            }
        }
        deserializer.deserialize_struct("vaultron.agent.v1.EnclaveAgentResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RestartEnclaveRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("vaultron.agent.v1.RestartEnclaveRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RestartEnclaveRequest {
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
            type Value = RestartEnclaveRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.agent.v1.RestartEnclaveRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RestartEnclaveRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(RestartEnclaveRequest {
                })
            }
        }
        deserializer.deserialize_struct("vaultron.agent.v1.RestartEnclaveRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RestartEnclaveResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("vaultron.agent.v1.RestartEnclaveResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RestartEnclaveResponse {
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
            type Value = RestartEnclaveResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.agent.v1.RestartEnclaveResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RestartEnclaveResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(RestartEnclaveResponse {
                })
            }
        }
        deserializer.deserialize_struct("vaultron.agent.v1.RestartEnclaveResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StartEnclaveRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("vaultron.agent.v1.StartEnclaveRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StartEnclaveRequest {
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
            type Value = StartEnclaveRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.agent.v1.StartEnclaveRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StartEnclaveRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(StartEnclaveRequest {
                })
            }
        }
        deserializer.deserialize_struct("vaultron.agent.v1.StartEnclaveRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StartEnclaveResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("vaultron.agent.v1.StartEnclaveResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StartEnclaveResponse {
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
            type Value = StartEnclaveResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.agent.v1.StartEnclaveResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StartEnclaveResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(StartEnclaveResponse {
                })
            }
        }
        deserializer.deserialize_struct("vaultron.agent.v1.StartEnclaveResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StopEnclaveRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("vaultron.agent.v1.StopEnclaveRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StopEnclaveRequest {
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
            type Value = StopEnclaveRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.agent.v1.StopEnclaveRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StopEnclaveRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(StopEnclaveRequest {
                })
            }
        }
        deserializer.deserialize_struct("vaultron.agent.v1.StopEnclaveRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StopEnclaveResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("vaultron.agent.v1.StopEnclaveResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StopEnclaveResponse {
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
            type Value = StopEnclaveResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct vaultron.agent.v1.StopEnclaveResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StopEnclaveResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(StopEnclaveResponse {
                })
            }
        }
        deserializer.deserialize_struct("vaultron.agent.v1.StopEnclaveResponse", FIELDS, GeneratedVisitor)
    }
}
