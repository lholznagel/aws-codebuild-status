use crate::project::Project;

use rusoto_codebuild::{
    BatchGetBuildsInput, Build, CodeBuild, CodeBuildClient, ListBuildsInput,
};
use rusoto_core::Region;
use std::collections::HashMap;

pub struct Aws {
    codebuild_client: CodeBuildClient,
}

impl Aws {
    pub fn new() -> Self {
        let codebuild_client = CodeBuildClient::new(Region::default());

        Self {
            codebuild_client,
        }
    }

    pub fn gather_information(&mut self) -> HashMap<String, Project> {
        let mut result: HashMap<String, Project> = HashMap::new();

        for build in self.get_builds(None) {
            result.entry(build.project_name.clone().unwrap())
                .and_modify(|x| x.builds.push(build.clone()))
                .or_insert(Project {
                    builds: vec![build]
                });
        }

        result
    }

    fn get_builds(&self, next_token: Option<String>) -> Vec<Build> {
        let mut builds = Vec::new();

        let result = self.codebuild_client
            .list_builds(ListBuildsInput {
                next_token,
                ..Default::default()
            })
            .sync()
            .unwrap();

        builds.append(
            &mut self.codebuild_client
                .batch_get_builds(BatchGetBuildsInput {
                    ids: result.ids.unwrap()
                })
                .sync()
                .unwrap()
                .builds
                .unwrap()
        );

        if result.next_token.is_some() {
            builds.append(&mut self.get_builds(result.next_token));
        }

        builds
    }
}
