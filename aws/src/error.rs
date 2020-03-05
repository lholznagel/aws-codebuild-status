#[derive(Debug)]
pub enum AwsError {
    ErrorListingProjects(rusoto_core::RusotoError<rusoto_codebuild::ListProjectsError>),
    ErrorListingBuilds(rusoto_core::RusotoError<rusoto_codebuild::ListBuildsError>),
    ErrorGettingProjects(rusoto_core::RusotoError<rusoto_codebuild::BatchGetProjectsError>),
    ErrorGettingBuilds(rusoto_core::RusotoError<rusoto_codebuild::BatchGetBuildsError>)
}