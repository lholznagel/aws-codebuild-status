use askama::Template;
use aws_codebuild_status_aws::CodeBuildResult;
use std::collections::HashMap;

#[derive(Template)]
#[template(path = "codebuild_overview.html")]
pub struct TemplateData {
    pub build_information: HashMap<String, Vec<CodeBuildResult>>,
}
