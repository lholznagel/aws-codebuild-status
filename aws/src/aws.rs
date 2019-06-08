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
    pub fn fetch_all_builds(&self) -> Vec<CodeBuildResult> {
        let projects = self.get_projects(None);
        let mut result = Vec::new();

        for build in self.get_builds(None) {
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

        result
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

    fn get_builds(&self, next_token: Option<String>) -> Vec<Build> {
        let mut builds = Vec::new();

        let result = self
            .codebuild_client
            .list_builds(ListBuildsInput {
                next_token,
                ..Default::default()
            })
            .sync()
            .unwrap();

        builds.append(
            &mut self
                .codebuild_client
                .batch_get_builds(BatchGetBuildsInput {
                    ids: result.ids.unwrap(),
                })
                .sync()
                .unwrap()
                .builds
                .unwrap(),
        );

        if result.next_token.is_some() {
            builds.append(&mut self.get_builds(result.next_token));
        }

        builds
    }
}

impl Default for Aws {
    fn default() -> Self {
        Self {
            codebuild_client: CodeBuildClient::new(Region::default()),
        }
    }
}
