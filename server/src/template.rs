use askama::Template;
use aws_codebuild_status_aws::CodeBuildResult;

#[derive(Template)]
#[template(path = "codebuild_overview.html")]
pub struct TemplateData {
    pub codebuild: Vec<CodeBuildResult>,
}
