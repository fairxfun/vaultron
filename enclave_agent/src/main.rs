use clap::Parser;
use enclave_agent::{
    start_enclave_agent, AgentCreateOptions, AgentServiceOptions, EnclaveAgentError,
    DEFAULT_VAULTRON_ENCLAVE_AGENT_SERVICE_NAME, DEFAULT_VAULTRON_SERVICE_NAMESPACE, DEFAULT_VAULTRON_SERVICE_REGION,
};
use enclave_agent::{
    EnclaveAgentCreateOptions, EnclaveCreateOptions, DEFAULT_ENCLAVE_AGENT_GRPC_SERVER_PORT, DEFAULT_ENCLAVE_CPU_COUNT,
    DEFAULT_ENCLAVE_ELF_FILE, DEFAULT_ENCLAVE_LOG_LEVEL, DEFAULT_ENCLAVE_MEMORY_SIZE, DEFAULT_ENCLAVE_NAME_PREFIX,
};
use enclave_vsock::{DEFAULT_ENCLAVE_CID, DEFAULT_ENCLAVE_VSOCK_PORT};

#[derive(Parser, Debug)]
#[command(author, version = concat!(env!("CARGO_PKG_VERSION"), "-", env!("VAULTRON_GIT_REVISION")), about, long_about = None)]
struct EnclaveAgentArgs {
    /// Enclave CID
    #[arg(long, default_value_t = DEFAULT_ENCLAVE_CID)]
    enclave_cid: u32,

    /// Enclave VSOCK port
    #[arg(long, default_value_t = DEFAULT_ENCLAVE_VSOCK_PORT)]
    enclave_vsock_port: u32,

    /// Enclave CPU count
    #[arg(long, default_value_t = DEFAULT_ENCLAVE_CPU_COUNT)]
    enclave_cpu_count: u32,

    /// Enclave memory size (MB)
    #[arg(long, default_value_t = DEFAULT_ENCLAVE_MEMORY_SIZE)]
    enclave_memory_size: u32,

    /// Enclave ELF file path
    #[arg(long, default_value = DEFAULT_ENCLAVE_ELF_FILE)]
    enclave_elf_file: String,

    /// Enable debug mode
    #[arg(long, default_value_t = false)]
    debug_mode: bool,

    /// Enclave agent gRPC server port
    #[arg(long, default_value_t = DEFAULT_ENCLAVE_AGENT_GRPC_SERVER_PORT)]
    enclave_agent_grpc_server_port: u32,

    /// Service region
    #[arg(long, default_value = DEFAULT_VAULTRON_SERVICE_REGION)]
    region: String,

    /// Service namespace
    #[arg(long, default_value = DEFAULT_VAULTRON_SERVICE_NAMESPACE)]
    namespace: String,

    /// Service name
    #[arg(long, default_value = DEFAULT_VAULTRON_ENCLAVE_AGENT_SERVICE_NAME)]
    service_name: String,

    /// Log level
    #[arg(long, default_value = DEFAULT_ENCLAVE_LOG_LEVEL)]
    log_level: String,
}

fn enclave_name(cid: u32) -> String {
    format!("{}_{}", DEFAULT_ENCLAVE_NAME_PREFIX, cid)
}

impl From<EnclaveAgentArgs> for EnclaveAgentCreateOptions {
    fn from(args: EnclaveAgentArgs) -> Self {
        EnclaveAgentCreateOptions::builder()
            .enclave_create_options(
                EnclaveCreateOptions::builder()
                    .enclave_name(enclave_name(args.enclave_cid))
                    .enclave_cid(args.enclave_cid)
                    .enclave_vsock_port(args.enclave_vsock_port)
                    .enclave_cpu_count(args.enclave_cpu_count)
                    .enclave_memory_size(args.enclave_memory_size)
                    .enclave_elf_file(args.enclave_elf_file)
                    .debug_mode(args.debug_mode)
                    .build(),
            )
            .agent_create_options(
                AgentCreateOptions::builder()
                    .service_options(
                        AgentServiceOptions::builder()
                            .region(args.region)
                            .namespace(args.namespace)
                            .service(args.service_name)
                            .port(args.enclave_agent_grpc_server_port)
                            .build(),
                    )
                    .log_level(args.log_level)
                    .build(),
            )
            .build()
    }
}

#[tokio::main]
async fn main() -> Result<(), EnclaveAgentError> {
    let args = EnclaveAgentArgs::parse();
    let agent_options = EnclaveAgentCreateOptions::from(args);
    start_enclave_agent(agent_options).await?;
    Ok(())
}
