use crate::{AwsError, CodeBuildResult};

use rusoto_codebuild::{
    BatchGetBuildsInput, BatchGetProjectsInput, Build, CodeBuild, CodeBuildClient, ListBuildsInput,
    ListProjectsInput, Project,
};
use rusoto_core::Region;
use std::collections::HashMap;

pub struct Aws {
    codebuild_client: CodeBuildClient,
}

impl Aws {
    pub async fn fetch_builds(
        &self,
        next_token: Option<String>,
    ) -> Result<(Option<String>, Vec<CodeBuildResult>), AwsError> {
        let projects = self.get_projects().await?;
        let fetch_result = self.get_builds(next_token).await?;
        let mut result = Vec::new();

        for build in fetch_result.1 {
            let project_name = build.project_name.clone().unwrap();

            let project_details = projects
                .iter()
                .find(|x| x.name.clone().unwrap() == project_name);

            if project_details.is_none() {
                continue;
            }

            let mut tags = HashMap::new();
            for tag in project_details.unwrap().tags.clone().unwrap() {
                tags.insert(tag.key.unwrap(), tag.value.unwrap());
            }

            let mut codebuild_result = CodeBuildResult::from(build);
            codebuild_result.tags = tags;

            result.push(codebuild_result);
        }

        Ok((fetch_result.0, result))
    }

    pub async fn fetch_all_builds(&self) -> Result<Vec<CodeBuildResult>, AwsError> {
        let mut result = self.fetch_builds(None).await?;
        let mut builds = result.1;

        while result.0.is_some() {
            result = self.fetch_builds(result.0).await?;
            builds.append(&mut result.1);
        }

        Ok(builds)
    }

    async fn get_projects(&self) -> Result<Vec<Project>, AwsError> {
        let mut projects = Vec::new();
        let mut result = self.collect_projects(None).await?;

        while result.0.is_some() {
            projects.append(&mut result.1.clone());
            result = self.collect_projects(result.0).await?;
        }

        Ok(projects)
    }

    async fn collect_projects(&self, next_token: Option<String>) -> Result<(Option<String>, Vec<Project>), AwsError> {
        let mut projects = Vec::new();

        let result = self
            .codebuild_client
            .list_projects(ListProjectsInput {
                next_token,
                ..Default::default()
            })
            .await
            .map_err(AwsError::ErrorListingProjects)?;

        projects.append(
            &mut self
                .codebuild_client
                .batch_get_projects(BatchGetProjectsInput {
                    names: result.projects.unwrap(),
                })
                .await
                .map_err(AwsError::ErrorGettingProjects)?
                .projects
                .unwrap(),
        );

        Ok((result.next_token, projects))
    }

    async fn get_builds(
        &self,
        next_token: Option<String>,
    ) -> Result<(Option<String>, Vec<Build>), AwsError> {
        let result = self
            .codebuild_client
            .list_builds(ListBuildsInput {
                next_token,
                ..Default::default()
            })
            .await
            .map_err(AwsError::ErrorListingBuilds)?;

        let builds = self
            .codebuild_client
            .batch_get_builds(BatchGetBuildsInput {
                ids: result.ids.unwrap(),
            })
            .await
            .map_err(AwsError::ErrorGettingBuilds)?
            .builds
            .unwrap_or_default();

        Ok((result.next_token, builds))
    }
}

impl Default for Aws {
    fn default() -> Self {
        Self {
            codebuild_client: CodeBuildClient::new(Region::default()),
        }
    }
}
