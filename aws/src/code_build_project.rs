use crate::output::{BuildInformation, Status};
use chrono::{offset::TimeZone, Utc};
use rusoto_codebuild::{Build};
use rusoto_core::Region;
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct CodeBuildProject {
    pub builds: Vec<Build>,
    pub tags: HashMap<String, String>
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
                tags: self.tags.clone()
            });
        }

        build_information
    }
}