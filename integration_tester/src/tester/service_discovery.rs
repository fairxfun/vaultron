use crate::error::EnclaveTesterError;
use anyhow::Result;
// use service_discovery::{
//     VaultronServiceQuerier, VaultronServiceQuerierConfig, VaultronServiceRegister, VaultronServiceRegisterConfig,
// };
// use std::{collections::HashMap, time::Duration};
// use tokio::time::sleep;

pub async fn service_discovery_test() -> Result<(), EnclaveTesterError> {
    Ok(())
}

// async fn test_service_discovery() {
//     let service_id = "srv-33eez4kuqhljwa2o".to_string();
//     let instance_id = "test-enclave-agent".to_string();
//     let region = "".to_string();
//     let mut attributes = HashMap::new();
//     attributes.insert("version".to_string(), "1.0.0".to_string());
//     let register_config = VaultronServiceRegisterConfig::builder()
//         .region(region.clone())
//         .service_id(service_id.clone())
//         .instance_id(instance_id.clone())
//         .attributes(attributes)
//         .build();
//     let register = VaultronServiceRegister::new(register_config).await;
//     register.register_service().await.unwrap();

//     sleep(Duration::from_secs(5)).await;

//     let querier_config = VaultronServiceQuerierConfig::builder().region(region.clone()).build();
//     let querier = VaultronServiceQuerier::new(querier_config).await;

//     let instances = querier.list_instances(service_id.clone()).await.unwrap();
//     println!("instances: {:?}", instances);
//     let instance = querier
//         .get_instance(service_id.clone(), instance_id.clone())
//         .await
//         .unwrap();
//     println!("instance: {:?}", instance);
//     let instance_ids = instances.iter().map(|i| i.id.clone()).collect();
//     let instance_health_status = querier
//         .get_instance_health_status(service_id.clone(), instance_ids)
//         .await
//         .unwrap();
//     println!("instance_health_status: {:?}", instance_health_status);
// }
