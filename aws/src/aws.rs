use crate::CodeBuildResult;
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
    pub fn fetch_builds(
        &self,
        next_token: Option<String>,
    ) -> (Option<String>, Vec<CodeBuildResult>) {
        let projects = self.get_projects(None);
        let fetch_result = self.get_builds(next_token);
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

        (fetch_result.0, result)
    }

    pub fn fetch_all_builds(&self) -> Vec<CodeBuildResult> {
        let mut result = self.fetch_builds(None);
        let mut builds = result.1;

        while result.0.is_some() {
            result = self.fetch_builds(result.0);
            builds.append(&mut result.1);
        }

        builds
    }

    fn get_projects(&self, next_token: Option<String>) -> Vec<Project> {
        let mut projects = Vec::new();

        let result = self
            .codebuild_client
            .list_projects(ListProjectsInput {
                next_token,
                ..Default::default()
            })
            .sync()
            .unwrap();

        projects.append(
            &mut self
                .codebuild_client
                .batch_get_projects(BatchGetProjectsInput {
                    names: result.projects.unwrap(),
                })
                .sync()
                .unwrap()
                .projects
                .unwrap(),
        );

        if result.next_token.is_some() {
            projects.append(&mut self.get_projects(result.next_token));
        }

        projects
    }

    fn get_builds(&self, next_token: Option<String>) -> (Option<String>, Vec<Build>) {
        let result = self
            .codebuild_client
            .list_builds(ListBuildsInput {
                next_token,
                ..Default::default()
            })
            .sync()
            .unwrap();

        let builds = self
            .codebuild_client
            .batch_get_builds(BatchGetBuildsInput {
                ids: result.ids.unwrap(),
            })
            .sync()
            .unwrap()
            .builds
            .unwrap();

        (result.next_token, builds)
    }
}

impl Default for Aws {
    fn default() -> Self {
        Self {
            codebuild_client: CodeBuildClient::new(Region::default()),
        }
    }
}
