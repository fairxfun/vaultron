use thiserror::Error;

#[derive(Debug, Error)]
pub enum VaultronServiceDiscoveryError {
    #[error("Invalid attributes")]
    InvalidAttributes,
    #[error("Instance not found")]
    InstanceNotFound,
    #[error("Instance status unknown")]
    InstanceStatusUnknown,
    #[error("Namespace not found")]
    NamespaceNotFound,
    #[error("Service not found")]
    ServiceNotFound,
    #[error("Service ID or ARN not found")]
    ServiceIdOrArnNotFound,
    #[error("Service tag not found")]
    ServiceTagNotFound,
    #[error("Service tag value not found")]
    ServiceTagValueNotFound,
    #[error(transparent)]
    FromHexError(#[from] hex::FromHexError),
    #[error("AWS error: {0}")]
    AwsError(#[from] Box<aws_sdk_servicediscovery::Error>),
    #[error("AWS builder error: {0}")]
    AwsBuilderError(#[from] aws_sdk_servicediscovery::error::BuildError),
    #[error("AWS list namespaces error: {0}")]
    AwsListNamespacesError(#[from] Box<aws_sdk_servicediscovery::operation::list_namespaces::ListNamespacesError>),
    #[error("AWS list services error: {0}")]
    AwsListServicesError(#[from] Box<aws_sdk_servicediscovery::operation::list_services::ListServicesError>),
    #[error("AWS tag resource error: {0}")]
    AwsTagResourceError(#[from] Box<aws_sdk_servicediscovery::operation::tag_resource::TagResourceError>),
    #[error("AWS list tags for resource error: {0}")]
    AwsListTagsForResourceError(
        #[from] Box<aws_sdk_servicediscovery::operation::list_tags_for_resource::ListTagsForResourceError>,
    ),
    #[error("AWS list instances error: {0}")]
    AwsListInstancesError(#[from] Box<aws_sdk_servicediscovery::operation::list_instances::ListInstancesError>),
    #[error("AWS get instance error: {0}")]
    AwsGetInstanceError(#[from] Box<aws_sdk_servicediscovery::operation::get_instance::GetInstanceError>),
    #[error("AWS register instance error: {0}")]
    AwsRegisterInstanceError(
        #[from] Box<aws_sdk_servicediscovery::operation::register_instance::RegisterInstanceError>,
    ),
    #[error("AWS deregister instance error: {0}")]
    AwsDeregisterInstanceError(
        #[from] Box<aws_sdk_servicediscovery::operation::deregister_instance::DeregisterInstanceError>,
    ),
}

impl
    From<
        aws_smithy_runtime_api::client::result::SdkError<
            aws_sdk_servicediscovery::operation::list_namespaces::ListNamespacesError,
            aws_smithy_runtime_api::http::Response,
        >,
    > for VaultronServiceDiscoveryError
{
    fn from(
        err: aws_smithy_runtime_api::client::result::SdkError<
            aws_sdk_servicediscovery::operation::list_namespaces::ListNamespacesError,
            aws_smithy_runtime_api::http::Response,
        >,
    ) -> Self {
        VaultronServiceDiscoveryError::AwsListNamespacesError(Box::new(err.into_service_error()))
    }
}

impl
    From<
        aws_smithy_runtime_api::client::result::SdkError<
            aws_sdk_servicediscovery::operation::list_services::ListServicesError,
            aws_smithy_runtime_api::http::Response,
        >,
    > for VaultronServiceDiscoveryError
{
    fn from(
        err: aws_smithy_runtime_api::client::result::SdkError<
            aws_sdk_servicediscovery::operation::list_services::ListServicesError,
            aws_smithy_runtime_api::http::Response,
        >,
    ) -> Self {
        VaultronServiceDiscoveryError::AwsListServicesError(Box::new(err.into_service_error()))
    }
}

impl
    From<
        aws_smithy_runtime_api::client::result::SdkError<
            aws_sdk_servicediscovery::operation::tag_resource::TagResourceError,
            aws_smithy_runtime_api::http::Response,
        >,
    > for VaultronServiceDiscoveryError
{
    fn from(
        err: aws_smithy_runtime_api::client::result::SdkError<
            aws_sdk_servicediscovery::operation::tag_resource::TagResourceError,
            aws_smithy_runtime_api::http::Response,
        >,
    ) -> Self {
        VaultronServiceDiscoveryError::AwsTagResourceError(Box::new(err.into_service_error()))
    }
}

impl
    From<
        aws_smithy_runtime_api::client::result::SdkError<
            aws_sdk_servicediscovery::operation::list_tags_for_resource::ListTagsForResourceError,
            aws_smithy_runtime_api::http::Response,
        >,
    > for VaultronServiceDiscoveryError
{
    fn from(
        err: aws_smithy_runtime_api::client::result::SdkError<
            aws_sdk_servicediscovery::operation::list_tags_for_resource::ListTagsForResourceError,
            aws_smithy_runtime_api::http::Response,
        >,
    ) -> Self {
        VaultronServiceDiscoveryError::AwsListTagsForResourceError(Box::new(err.into_service_error()))
    }
}

impl
    From<
        aws_smithy_runtime_api::client::result::SdkError<
            aws_sdk_servicediscovery::operation::list_instances::ListInstancesError,
            aws_smithy_runtime_api::http::Response,
        >,
    > for VaultronServiceDiscoveryError
{
    fn from(
        err: aws_smithy_runtime_api::client::result::SdkError<
            aws_sdk_servicediscovery::operation::list_instances::ListInstancesError,
            aws_smithy_runtime_api::http::Response,
        >,
    ) -> Self {
        VaultronServiceDiscoveryError::AwsListInstancesError(Box::new(err.into_service_error()))
    }
}

impl
    From<
        aws_smithy_runtime_api::client::result::SdkError<
            aws_sdk_servicediscovery::operation::get_instance::GetInstanceError,
            aws_smithy_runtime_api::http::Response,
        >,
    > for VaultronServiceDiscoveryError
{
    fn from(
        err: aws_smithy_runtime_api::client::result::SdkError<
            aws_sdk_servicediscovery::operation::get_instance::GetInstanceError,
            aws_smithy_runtime_api::http::Response,
        >,
    ) -> Self {
        VaultronServiceDiscoveryError::AwsGetInstanceError(Box::new(err.into_service_error()))
    }
}

impl
    From<
        aws_smithy_runtime_api::client::result::SdkError<
            aws_sdk_servicediscovery::operation::register_instance::RegisterInstanceError,
            aws_smithy_runtime_api::http::Response,
        >,
    > for VaultronServiceDiscoveryError
{
    fn from(
        err: aws_smithy_runtime_api::client::result::SdkError<
            aws_sdk_servicediscovery::operation::register_instance::RegisterInstanceError,
            aws_smithy_runtime_api::http::Response,
        >,
    ) -> Self {
        VaultronServiceDiscoveryError::AwsRegisterInstanceError(Box::new(err.into_service_error()))
    }
}

impl
    From<
        aws_smithy_runtime_api::client::result::SdkError<
            aws_sdk_servicediscovery::operation::deregister_instance::DeregisterInstanceError,
            aws_smithy_runtime_api::http::Response,
        >,
    > for VaultronServiceDiscoveryError
{
    fn from(
        err: aws_smithy_runtime_api::client::result::SdkError<
            aws_sdk_servicediscovery::operation::deregister_instance::DeregisterInstanceError,
            aws_smithy_runtime_api::http::Response,
        >,
    ) -> Self {
        VaultronServiceDiscoveryError::AwsDeregisterInstanceError(Box::new(err.into_service_error()))
    }
}
