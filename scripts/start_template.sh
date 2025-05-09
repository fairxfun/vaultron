#!/bin/sh

dnf update -y
mkdir -p /fairx/certs
mkdir -p /fairx/logs

get_metadata() {
    local path="http://169.254.169.254/latest/meta-data/tags/instance/$1"
    local value=$(curl -s -w "%{http_code}" "$path")
    local status_code=${value: -3}
    local content=${value%???}
    
    if [ "$status_code" != "200" ]; then
        echo ""
    else
        echo "$content"
    fi
}

get_all_metadata() {
    echo "Retrieving enclave metadata..."
    ENCLAVE_VERSION=$(get_metadata "enclave-version")
    if [ "$ENCLAVE_VERSION" == "" ]; then
        echo "Failed to retrieve enclave version from metadata service"
        exit 1
    fi
    
    ENCLAVE_MEMORY_MIB=$(get_metadata "enclave-memory-mib")
    if [ "$ENCLAVE_MEMORY_MIB" == "" ]; then
        ENCLAVE_MEMORY_MIB=2048
    fi

    ENCLAVE_DEBUG_MODE=$(get_metadata "enclave-debug-mode")
    if [ "$ENCLAVE_DEBUG_MODE" == "true" ]; then
        ENCLAVE_DEBUG_MODE="--enclave-debug-mode"
    else
        ENCLAVE_DEBUG_MODE=""
    fi

    ENCLAVE_CLUSTER_INIT_STARTUP=$(get_metadata "enclave-cluster-init-startup")
    if [ "$ENCLAVE_CLUSTER_INIT_STARTUP" == "true" ]; then
        ENCLAVE_CLUSTER_INIT_STARTUP="--enclave-cluster-init-startup"
    else
        ENCLAVE_CLUSTER_INIT_STARTUP=""
    fi
}

get_aws_nitro_enclaves_root_cert() {
    curl -O https://aws-nitro-enclaves.amazonaws.com/AWS_NitroEnclaves_Root-G1.zip
    unzip AWS_NitroEnclaves_Root-G1.zip
    mv -f root.pem /fairx/certs/aws-nitro-enclaves-root.pem
    rm -f AWS_NitroEnclaves_Root-G1.zip
}

get_vaultron_enclave() {
    wget https://github.com/fairxfun/vaultron/releases/download/${ENCLAVE_VERSION}/vaultron_enclave.eif
    wget https://github.com/fairxfun/vaultron/releases/download/${ENCLAVE_VERSION}/vaultron_enclave_agent
    chmod +x vaultron_enclave_agent
    mv -f vaultron_enclave.eif /fairx/
    mv -f vaultron_enclave_agent /fairx/
}

update_enclave_service() {
    dnf install -y aws-nitro-enclaves-cli-1.3.4 aws-nitro-enclaves-cli-devel-1.3.4
    systemctl stop nitro-enclaves-allocator.service
    ALLOCATOR_YAML=/etc/nitro_enclaves/allocator.yaml
    MEM_KEY=memory_mib
    sed -r "s/^(\s*${MEM_KEY}\s*:\s*).*/\1${ENCLAVE_MEMORY_MIB}/" -i "${ALLOCATOR_YAML}"
    systemctl start nitro-enclaves-allocator.service 
    systemctl enable nitro-enclaves-allocator.service
}

setup_vaultron_service() {
    # Create service file
    cat > /etc/systemd/system/vaultron-enclave-agent.service << EOF
[Unit]
Description=Vaultron Enclave Agent Service
After=network.target nitro-enclaves-allocator.service
Requires=nitro-enclaves-allocator.service

[Service]
Type=simple
ExecStart=/fairx/vaultron_enclave_agent ${ENCLAVE_DEBUG_MODE} ${ENCLAVE_CLUSTER_INIT_STARTUP}
Restart=always
RestartSec=10
StandardOutput=append:/fairx/logs/vaultron_enclave_agent.log
StandardError=append:/fairx/logs/vaultron_enclave_agent.log

[Install]
WantedBy=multi-user.target
EOF

    # Reload systemd and start service
    systemctl daemon-reload
    systemctl enable vaultron-enclave-agent.service
    systemctl start vaultron-enclave-agent.service
}

get_all_metadata
{
    echo "=== Enclave Configuration ==="
    echo "Version: ${ENCLAVE_VERSION}"
    echo "Memory (MiB): ${ENCLAVE_MEMORY_MIB}"
    echo "Debug Mode: ${ENCLAVE_DEBUG_MODE}"
    echo "Cluster Init Startup: ${ENCLAVE_CLUSTER_INIT_STARTUP}"
    echo "==========================="
} > /fairx/logs/vaultron_enclave_install.log

get_aws_nitro_enclaves_root_cert
get_vaultron_enclave
update_enclave_service
setup_vaultron_service
