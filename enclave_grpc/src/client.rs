use anyhow::Result;
use enclave_protos::vaultron::service::v1::ClientOptions;
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Endpoint};

pub async fn connect(options: &ClientOptions) -> Result<Channel> {
    let uri = options.to_uri()?;
    let mut endpoint = Endpoint::from(uri);

    if let Some(connect_timeout_ms) = options.connect_timeout_ms {
        endpoint = endpoint.connect_timeout(std::time::Duration::from_millis(connect_timeout_ms));
    }

    if let Some(request_timeout_ms) = options.request_timeout_ms {
        endpoint = endpoint.timeout(std::time::Duration::from_millis(request_timeout_ms));
    }

    if let Some(keep_alive_while_idle) = options.keep_alive_while_idle {
        endpoint = endpoint.keep_alive_while_idle(keep_alive_while_idle);
    }

    if let Some(keep_alive_timeout_ms) = options.keep_alive_timeout_ms {
        endpoint = endpoint.keep_alive_timeout(std::time::Duration::from_millis(keep_alive_timeout_ms));
    }

    let channel = if let Some(tls) = tls_config(options).await? {
        endpoint.tls_config(tls)?.connect_lazy()
    } else {
        endpoint.connect_lazy()
    };
    Ok(channel)
}

async fn tls_config(options: &ClientOptions) -> Result<Option<ClientTlsConfig>, std::io::Error> {
    if options.is_ssl() {
        let mut cert: Option<Certificate> = None;
        if let Some(ssl_cert) = options.ssl_cert.clone() {
            cert = Some(Certificate::from_pem(ssl_cert));
        } else {
            #[cfg(feature = "fs")]
            if let Some(ssl_cert_path) = options.ssl_cert_path.clone() {
                cert = Some(Certificate::from_pem(tokio::fs::read(ssl_cert_path).await?));
            }
        }
        if let Some(cert) = cert {
            let tls = if let Some(domain_name) = &options.ssl_server_name {
                ClientTlsConfig::new().ca_certificate(cert).domain_name(domain_name)
            } else {
                ClientTlsConfig::new()
                    .ca_certificate(cert)
                    .domain_name(options.get_host())
            };
            return Ok(Some(tls));
        }
    }
    Ok(None)
}
