use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceDiscoveryError {
    #[error("Invalid attributes")]
    InvalidAttributes,
    #[error("Instance not found")]
    InstanceNotFound,
    #[error("Instance status unknown")]
    InstanceStatusUnknown,
    #[error("AWS error: {0}")]
    AwsError(#[from] Box<aws_sdk_servicediscovery::Error>),
    #[error("AWS register instance error: {0}")]
    AwsRegisterInstanceError(
        #[from] Box<aws_sdk_servicediscovery::operation::register_instance::RegisterInstanceError>,
    ),
    #[error("AWS deregister instance error: {0}")]
    AwsDeregisterInstanceError(
        #[from] Box<aws_sdk_servicediscovery::operation::deregister_instance::DeregisterInstanceError>,
    ),
    #[error("AWS list instances error: {0}")]
    AwsListInstancesError(#[from] Box<aws_sdk_servicediscovery::operation::list_instances::ListInstancesError>),
    #[error("AWS get instance error: {0}")]
    AwsGetInstanceError(#[from] Box<aws_sdk_servicediscovery::operation::get_instance::GetInstanceError>),
    #[error("AWS get instances health status error: {0}")]
    AwsGetInstancesHealthStatusError(
        #[from] Box<aws_sdk_servicediscovery::operation::get_instances_health_status::GetInstancesHealthStatusError>,
    ),
}

impl
    From<
        aws_smithy_runtime_api::client::result::SdkError<
            aws_sdk_servicediscovery::operation::register_instance::RegisterInstanceError,
            aws_smithy_runtime_api::http::Response,
        >,
    > for ServiceDiscoveryError
{
    fn from(
        err: aws_smithy_runtime_api::client::result::SdkError<
            aws_sdk_servicediscovery::operation::register_instance::RegisterInstanceError,
            aws_smithy_runtime_api::http::Response,
        >,
    ) -> Self {
        ServiceDiscoveryError::AwsRegisterInstanceError(Box::new(err.into_service_error()))
    }
}

impl
    From<
        aws_smithy_runtime_api::client::result::SdkError<
            aws_sdk_servicediscovery::operation::deregister_instance::DeregisterInstanceError,
            aws_smithy_runtime_api::http::Response,
        >,
    > for ServiceDiscoveryError
{
    fn from(
        err: aws_smithy_runtime_api::client::result::SdkError<
            aws_sdk_servicediscovery::operation::deregister_instance::DeregisterInstanceError,
            aws_smithy_runtime_api::http::Response,
        >,
    ) -> Self {
        ServiceDiscoveryError::AwsDeregisterInstanceError(Box::new(err.into_service_error()))
    }
}

impl
    From<
        aws_smithy_runtime_api::client::result::SdkError<
            aws_sdk_servicediscovery::operation::list_instances::ListInstancesError,
            aws_smithy_runtime_api::http::Response,
        >,
    > for ServiceDiscoveryError
{
    fn from(
        err: aws_smithy_runtime_api::client::result::SdkError<
            aws_sdk_servicediscovery::operation::list_instances::ListInstancesError,
            aws_smithy_runtime_api::http::Response,
        >,
    ) -> Self {
        ServiceDiscoveryError::AwsListInstancesError(Box::new(err.into_service_error()))
    }
}

impl
    From<
        aws_smithy_runtime_api::client::result::SdkError<
            aws_sdk_servicediscovery::operation::get_instance::GetInstanceError,
            aws_smithy_runtime_api::http::Response,
        >,
    > for ServiceDiscoveryError
{
    fn from(
        err: aws_smithy_runtime_api::client::result::SdkError<
            aws_sdk_servicediscovery::operation::get_instance::GetInstanceError,
            aws_smithy_runtime_api::http::Response,
        >,
    ) -> Self {
        ServiceDiscoveryError::AwsGetInstanceError(Box::new(err.into_service_error()))
    }
}

impl
    From<
        aws_smithy_runtime_api::client::result::SdkError<
            aws_sdk_servicediscovery::operation::get_instances_health_status::GetInstancesHealthStatusError,
            aws_smithy_runtime_api::http::Response,
        >,
    > for ServiceDiscoveryError
{
    fn from(
        err: aws_smithy_runtime_api::client::result::SdkError<
            aws_sdk_servicediscovery::operation::get_instances_health_status::GetInstancesHealthStatusError,
            aws_smithy_runtime_api::http::Response,
        >,
    ) -> Self {
        ServiceDiscoveryError::AwsGetInstancesHealthStatusError(Box::new(err.into_service_error()))
    }
}
