use aws_codebuild_status_aws::{Aws, CodeBuildResult};
use lambda_runtime::error::HandlerError;
use lambda_runtime::lambda;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
struct Event {
    failed: Option<bool>,
    tags: Option<Vec<String>>,
}

fn main() {
    lambda!(my_handler);
}

fn my_handler(_: Event, _: lambda_runtime::Context) -> Result<Vec<CodeBuildResult>, HandlerError> {
    let aws = Aws::default();
    Ok(aws.fetch_all_builds())
}
