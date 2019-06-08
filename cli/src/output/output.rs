use aws_codebuild_status_aws::CodeBuildResult;
use std::collections::HashMap;

pub trait Output {

    fn print(build_info: HashMap<String, Vec<CodeBuildResult>>);
}