// @generated
pub mod vaultron {
    pub mod attestation {
        #[cfg(feature = "vaultron-attestation-v1")]
        // @@protoc_insertion_point(attribute:vaultron.attestation.v1)
        pub mod v1 {
            include!("vaultron.attestation.v1.rs");
            // @@protoc_insertion_point(vaultron.attestation.v1)
        }
    }
    pub mod cluster {
        #[cfg(feature = "vaultron-cluster-v1")]
        // @@protoc_insertion_point(attribute:vaultron.cluster.v1)
        pub mod v1 {
            include!("vaultron.cluster.v1.rs");
            // @@protoc_insertion_point(vaultron.cluster.v1)
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
    pub mod internal {
        #[cfg(feature = "vaultron-internal-v1")]
        // @@protoc_insertion_point(attribute:vaultron.internal.v1)
        pub mod v1 {
            include!("vaultron.internal.v1.rs");
            // @@protoc_insertion_point(vaultron.internal.v1)
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
    #[cfg(feature = "vaultron-v1")]
    // @@protoc_insertion_point(attribute:vaultron.v1)
    pub mod v1 {
        include!("vaultron.v1.rs");
        // @@protoc_insertion_point(vaultron.v1)
    }
}
