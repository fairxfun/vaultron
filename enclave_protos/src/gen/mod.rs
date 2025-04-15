// @generated
pub mod vaultron {
    pub mod agent {
        #[cfg(feature = "vaultron-agent-v1")]
        // @@protoc_insertion_point(attribute:vaultron.agent.v1)
        pub mod v1 {
            include!("vaultron.agent.v1.rs");
            // @@protoc_insertion_point(vaultron.agent.v1)
        }
    }
    pub mod common {
        #[cfg(feature = "vaultron-common-v1")]
        // @@protoc_insertion_point(attribute:vaultron.common.v1)
        pub mod v1 {
            include!("vaultron.common.v1.rs");
            // @@protoc_insertion_point(vaultron.common.v1)
        }
    }
    pub mod enclave {
        pub mod attestation {
            #[cfg(feature = "vaultron-enclave-attestation-v1")]
            // @@protoc_insertion_point(attribute:vaultron.enclave.attestation.v1)
            pub mod v1 {
                include!("vaultron.enclave.attestation.v1.rs");
                // @@protoc_insertion_point(vaultron.enclave.attestation.v1)
            }
        }
        pub mod cluster {
            #[cfg(feature = "vaultron-enclave-cluster-v1")]
            // @@protoc_insertion_point(attribute:vaultron.enclave.cluster.v1)
            pub mod v1 {
                include!("vaultron.enclave.cluster.v1.rs");
                // @@protoc_insertion_point(vaultron.enclave.cluster.v1)
            }
        }
        pub mod common {
            #[cfg(feature = "vaultron-enclave-common-v1")]
            // @@protoc_insertion_point(attribute:vaultron.enclave.common.v1)
            pub mod v1 {
                include!("vaultron.enclave.common.v1.rs");
                // @@protoc_insertion_point(vaultron.enclave.common.v1)
            }
        }
        pub mod internal {
            #[cfg(feature = "vaultron-enclave-internal-v1")]
            // @@protoc_insertion_point(attribute:vaultron.enclave.internal.v1)
            pub mod v1 {
                include!("vaultron.enclave.internal.v1.rs");
                // @@protoc_insertion_point(vaultron.enclave.internal.v1)
            }
        }
        #[cfg(feature = "vaultron-enclave-v1")]
        // @@protoc_insertion_point(attribute:vaultron.enclave.v1)
        pub mod v1 {
            include!("vaultron.enclave.v1.rs");
            // @@protoc_insertion_point(vaultron.enclave.v1)
        }
    }
    pub mod service {
        #[cfg(feature = "vaultron-service-v1")]
        // @@protoc_insertion_point(attribute:vaultron.service.v1)
        pub mod v1 {
            include!("vaultron.service.v1.rs");
            // @@protoc_insertion_point(vaultron.service.v1)
        }
    }
    pub mod test {
        #[cfg(feature = "vaultron-test-v1")]
        // @@protoc_insertion_point(attribute:vaultron.test.v1)
        pub mod v1 {
            include!("vaultron.test.v1.rs");
            // @@protoc_insertion_point(vaultron.test.v1)
        }
    }
}
