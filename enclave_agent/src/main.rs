use clap::Parser;
use enclave_agent::{
    enclave_agent_service_name, enclave_name, start_enclave_agent, AgentCreateOptions, AgentServiceOptions,
    Ec2InstanceOptions, EnclaveAgentError, DEFAULT_VAULTRON_SERVICE_NAMESPACE, DEFAULT_VAULTRON_SERVICE_REGION,
    ENCLAVE_AGENT_VERSION,
};
use enclave_agent::{
    CreateOptions, EnclaveCreateOptions, DEFAULT_ENCLAVE_AGENT_GRPC_SERVER_PORT, DEFAULT_ENCLAVE_CPU_COUNT,
    DEFAULT_ENCLAVE_ELF_FILE, DEFAULT_ENCLAVE_LOG_LEVEL, DEFAULT_ENCLAVE_MEMORY_SIZE,
};
use enclave_vsock::{DEFAULT_ENCLAVE_CID, DEFAULT_ENCLAVE_VSOCK_PORT};

#[derive(Parser, Debug)]
#[command(author, version = concat!("v", env!("CARGO_PKG_VERSION"), "-", env!("VAULTRON_GIT_REVISION")), about, long_about = None)]
struct EnclaveAgentArgs {
    #[arg(long, default_value_t = DEFAULT_ENCLAVE_CID)]
    enclave_cid: u32,

    #[arg(long, default_value_t = DEFAULT_ENCLAVE_VSOCK_PORT)]
    enclave_vsock_port: u32,

    #[arg(long, default_value_t = DEFAULT_ENCLAVE_CPU_COUNT)]
    enclave_cpu_count: u32,

    #[arg(long, default_value_t = DEFAULT_ENCLAVE_MEMORY_SIZE)]
    enclave_memory_size: u32,

    #[arg(long, default_value = DEFAULT_ENCLAVE_ELF_FILE)]
    enclave_elf_file: String,

    #[arg(long, default_value_t = false)]
    enclave_cluster_init_startup: bool,

    #[arg(long, default_value_t = false)]
    enclave_debug_mode: bool,

    #[arg(long, default_value_t = DEFAULT_ENCLAVE_AGENT_GRPC_SERVER_PORT)]
    enclave_agent_grpc_server_port: u32,

    #[arg(long, default_value = DEFAULT_VAULTRON_SERVICE_REGION)]
    region: String,

    #[arg(long, default_value = DEFAULT_VAULTRON_SERVICE_NAMESPACE)]
    namespace: String,

    #[arg(long, default_value = DEFAULT_ENCLAVE_LOG_LEVEL)]
    log_level: String,
}

async fn create_options_from_args(args: EnclaveAgentArgs) -> Result<CreateOptions, EnclaveAgentError> {
    let ec2_instance_options = Ec2InstanceOptions::new().await?;
    let instance_id = ec2_instance_options.instance_id.clone();
    Ok(CreateOptions::builder()
        .ec2_instance_options(ec2_instance_options)
        .enclave_create_options(
            EnclaveCreateOptions::builder()
                .name(enclave_name(&instance_id, args.enclave_cid))
                .cid(args.enclave_cid)
                .vsock_port(args.enclave_vsock_port)
                .cpu_count(args.enclave_cpu_count)
                .memory_size(args.enclave_memory_size)
                .elf_file(args.enclave_elf_file)
                .cluster_init_startup(args.enclave_cluster_init_startup)
                .debug_mode(args.enclave_debug_mode)
                .build(),
        )
        .agent_create_options(
            AgentCreateOptions::builder()
                .service_options(
                    AgentServiceOptions::builder()
                        .region(args.region)
                        .namespace(args.namespace)
                        .service_name(enclave_agent_service_name(ENCLAVE_AGENT_VERSION))
                        .port(args.enclave_agent_grpc_server_port)
                        .build(),
                )
                .log_level(args.log_level)
                .build(),
        )
        .build())
}

#[tokio::main]
async fn main() -> Result<(), EnclaveAgentError> {
    let args = EnclaveAgentArgs::parse();
    let agent_options = create_options_from_args(args).await?;
    start_enclave_agent(agent_options).await?;
    Ok(())
}
