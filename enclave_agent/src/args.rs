use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct EnclaveArgs {
    pub cid: u32,
    pub port: u32,
}
