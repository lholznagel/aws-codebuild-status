use aws_codebuild_status_derive::BuildInformation;
use chrono::{offset::TimeZone, Utc};
use rusoto_codebuild::{
    BatchGetBuildsInput, Build, CodeBuild, CodeBuildClient, ListBuildsForProjectInput,
    ListProjectsInput,
};
use rusoto_core::Region;

pub struct AWS {
    pub build_information: Vec<BuildInformation>,
    builds: Vec<Build>,
    build_ids: Vec<String>,
    codebuild_client: CodeBuildClient,
    projects: Vec<String>,
}

impl AWS {
    pub fn new() -> Self {
        let codebuild_client = CodeBuildClient::new(Region::default());
        Self {
            build_information: Vec::new(),
            builds: Vec::new(),
            build_ids: Vec::new(),
            codebuild_client,
            projects: Vec::new(),
        }
    }

    pub fn get_build_information(&mut self) -> Vec<BuildInformation> {
        if self.build_information.is_empty() {
            for build in self.get_builds() {
                let build_status = build.clone().build_status.unwrap();
                let timestamp = Utc.timestamp(build.clone().end_time.unwrap() as i64, 0);
                let url = format!(
                    "https://{}.console.aws.amazon.com/codesuite/codebuild/projects/{}/build/{}/log",
                    Region::default().name(),
                    build.clone().project_name.unwrap(),
                    build.clone().id.unwrap().replace(':', "%3A")
                );

                self.build_information.push(BuildInformation {
                    commit_id: build.clone().resolved_source_version.unwrap(),
                    name: build.clone().project_name.unwrap(),
                    status: build_status,
                    timestamp: timestamp.to_rfc2822(),
                    url,
                });
            }
        }

        self.build_information.to_owned()
    }

    fn get_projects(&mut self) -> Vec<String> {
        if self.projects.is_empty() {
            self.projects = self
                .codebuild_client
                .list_projects(ListProjectsInput::default())
                .sync()
                .unwrap()
                .projects
                .unwrap();
        }

        self.projects.to_owned()
    }

    fn get_project_builds(&mut self) -> Vec<String> {
        if self.build_ids.is_empty() {
            for project in self.get_projects() {
                let builds = self
                    .codebuild_client
                    .list_builds_for_project(ListBuildsForProjectInput {
                        project_name: project.clone(),
                        ..Default::default()
                    })
                    .sync()
                    .unwrap()
                    .ids
                    .unwrap();
                self.build_ids.push(builds[0].clone());
            }
        }

        self.build_ids.to_owned()
    }

    fn get_builds(&mut self) -> Vec<Build> {
        let ids = self.get_project_builds();

        if self.builds.is_empty() {
            self.builds = self
                .codebuild_client
                .batch_get_builds(BatchGetBuildsInput { ids })
                .sync()
                .unwrap()
                .builds
                .unwrap();
        }

        self.builds.to_owned()
    }
}
