/* automatically generated by rust-bindgen 0.71.1 */

#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(warnings)]

#[link(name = "kmstool-enclave-lib")]
unsafe extern "C" {}

#[doc = " @brief Initialization parameters for the KMS Tool enclave\n\n This structure contains all the necessary parameters to initialize\n the KMS Tool enclave, including AWS credentials, region settings,\n and encryption parameters."]
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct kmstool_init_params {
    pub enable_logging: ::core::ffi::c_uint,
    pub proxy_port: ::core::ffi::c_uint,
    pub aws_region: *const ::core::ffi::c_char,
    pub aws_access_key_id: *const ::core::ffi::c_char,
    pub aws_secret_access_key: *const ::core::ffi::c_char,
    pub aws_session_token: *const ::core::ffi::c_char,
    pub kms_key_id: *const ::core::ffi::c_char,
    pub kms_encryption_algorithm: *const ::core::ffi::c_char,
}
#[doc = " @brief Parameters for updating AWS credentials\n\n This structure contains the parameters needed to update the AWS credentials\n for an already initialized KMS Tool enclave."]
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct kmstool_update_aws_key_params {
    pub aws_access_key_id: *const ::core::ffi::c_char,
    pub aws_secret_access_key: *const ::core::ffi::c_char,
    pub aws_session_token: *const ::core::ffi::c_char,
}
#[doc = " @brief Parameters for encryption operation\n\n This structure contains the data to be encrypted using KMS."]
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct kmstool_encrypt_params {
    pub plaintext: *const ::core::ffi::c_uchar,
    pub plaintext_len: ::core::ffi::c_uint,
}
#[doc = " @brief Parameters for decryption operation\n\n This structure contains the data to be decrypted using KMS."]
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct kmstool_decrypt_params {
    pub ciphertext: *const ::core::ffi::c_uchar,
    pub ciphertext_len: ::core::ffi::c_uint,
}
#[repr(i32)]
#[doc = " @brief Status codes for KMS Tool operations\n\n These status codes indicate the result of KMS Tool operations.\n All functions in this library return one of these values."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum KMSTOOL_STATUS {
    KMSTOOL_ERROR = -1,
    KMSTOOL_SUCCESS = 0,
}
impl Default for kmstool_init_params {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for kmstool_update_aws_key_params {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for kmstool_encrypt_params {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for kmstool_decrypt_params {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    #[doc = " @brief Initialize the KMS Tool enclave with the given parameters\n\n This function must be called before performing any KMS operations.\n It sets up the AWS credentials, KMS client, and other necessary resources.\n\n @param params Pointer to initialization parameters\n @return KMSTOOL_SUCCESS on success, KMSTOOL_ERROR on failure"]
    pub fn kmstool_enclave_init(params: *const kmstool_init_params) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    #[doc = " @brief Clean up and stop the KMS Tool enclave\n\n This function releases all resources associated with the KMS Tool enclave.\n It should be called when the enclave is no longer needed.\n\n @return KMSTOOL_SUCCESS on success, KMSTOOL_ERROR on failure"]
    pub fn kmstool_enclave_stop() -> ::core::ffi::c_int;
}
unsafe extern "C" {
    #[doc = " @brief Update AWS credentials for the KMS Tool enclave\n\n This function updates the AWS credentials used by the KMS Tool enclave.\n The enclave must be initialized before calling this function.\n\n @param params Pointer to the new AWS credentials\n @return KMSTOOL_SUCCESS on success, KMSTOOL_ERROR on failure"]
    pub fn kmstool_enclave_update_aws_key(params: *const kmstool_update_aws_key_params) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    #[doc = " @brief Encrypt data using KMS\n\n This function encrypts the provided plaintext using the configured KMS key\n and encryption algorithm.\n\n @param params Pointer to encryption parameters\n @param ciphertext_out Pointer to store the encrypted data (caller must free)\n @param ciphertext_out_len Pointer to store the length of encrypted data\n @return KMSTOOL_SUCCESS on success, KMSTOOL_ERROR on failure"]
    pub fn kmstool_enclave_encrypt(
        params: *const kmstool_encrypt_params,
        ciphertext_out: *mut *mut ::core::ffi::c_uchar,
        ciphertext_out_len: *mut ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    #[doc = " @brief Decrypt data using KMS\n\n This function decrypts the provided ciphertext using the configured KMS key\n and encryption algorithm.\n\n @param params Pointer to decryption parameters\n @param plaintext_out Pointer to store the decrypted data (caller must free)\n @param plaintext_out_len Pointer to store the length of decrypted data\n @return KMSTOOL_SUCCESS on success, KMSTOOL_ERROR on failure"]
    pub fn kmstool_enclave_decrypt(
        params: *const kmstool_decrypt_params,
        plaintext_out: *mut *mut ::core::ffi::c_uchar,
        plaintext_out_len: *mut ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
}
