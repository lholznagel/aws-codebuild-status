use aws_codebuild_status_derive::{BuildInformation, Status};
use chrono::{offset::TimeZone, Utc};
use rusoto_codebuild::{
    BatchGetBuildsInput, Build, CodeBuild, CodeBuildClient, ListBuildsInput,
};
use rusoto_core::Region;
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct CodeBuildProject {
    builds: Vec<Build>
}

impl CodeBuildProject {
    pub fn get_build_information(&mut self) -> Vec<BuildInformation> {
        let mut build_information = Vec::new();

        for build in self.builds.iter() {
            let build_status = build.clone().build_status.unwrap();

            if build.clone().resolved_source_version.is_none() {
                continue;
            }

            let commit_id = build.clone().resolved_source_version.unwrap();
            let timestamp = Utc.timestamp(build.clone().end_time.unwrap() as i64, 0);
            let url = format!(
                "https://{}.console.aws.amazon.com/codesuite/codebuild/projects/{}/build/{}/log",
                Region::default().name(),
                build.clone().project_name.unwrap(),
                build.clone().id.unwrap().replace(':', "%3A")
            );

            let location = build.clone().source.unwrap().location.unwrap_or_else(|| String::from("Undefined"));
            let splitted = location.split('/').collect::<Vec<&str>>();
            let repository_name = splitted.last().unwrap().to_string();

            build_information.push(BuildInformation {
                commit_id,
                project_name: build.project_name.clone().unwrap(),
                repository_name,
                status: Status::from(build_status),
                timestamp: timestamp.to_rfc2822(),
                url,
            });
        }

        build_information
    }
}

pub struct AWSCli {
    codebuild_client: CodeBuildClient,
}

impl AWSCli {
    pub fn new() -> Self {
        let codebuild_client = CodeBuildClient::new(Region::default());

        Self {
            codebuild_client,
        }
    }

    pub fn gather_information(&mut self) -> HashMap<String, CodeBuildProject> {
        let mut result: HashMap<String, CodeBuildProject> = HashMap::new();

        for build in self.get_builds(None) {
            result.entry(build.project_name.clone().unwrap())
                .and_modify(|x| x.builds.push(build.clone()))
                .or_insert(CodeBuildProject {
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
