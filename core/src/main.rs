mod terminal;

#[cfg(feature = "web")]
use aws_codebuild_status_web::WebOutput;

use aws_codebuild_status_derive::{BuildInformation, CodebuildOutput};
use chrono::{offset::TimeZone, Utc};
use rusoto_codebuild::{
    BatchGetBuildsInput, CodeBuild, CodeBuildClient, ListBuildsForProjectInput, ListProjectsInput,
};
use rusoto_core::credential::ChainProvider;
use rusoto_core::{HttpClient, Region};

fn main() {
    let mut build_information = Vec::new();
    let mut build_ids = Vec::new();

    let client = CodeBuildClient::new_with(
        HttpClient::new().unwrap(),
        ChainProvider::default(),
        Region::default(),
    );
    let projects = client
        .list_projects(ListProjectsInput {
            next_token: None,
            sort_by: None,
            sort_order: None,
        })
        .sync()
        .unwrap();

    for project in projects.projects.unwrap() {
        let builds = client
            .list_builds_for_project(ListBuildsForProjectInput {
                next_token: None,
                sort_order: None,
                project_name: project,
            })
            .sync()
            .unwrap();
        build_ids.push(builds.ids.unwrap()[0].clone());
    }

    let builds_response = client
        .batch_get_builds(BatchGetBuildsInput { ids: build_ids })
        .sync()
        .unwrap();

    for build in builds_response.builds.unwrap() {
        let build_status = build.clone().build_status.unwrap();
        let timestamp = Utc.timestamp(build.clone().end_time.unwrap() as i64, 0);
        let url = format!(
            "https://{}.console.aws.amazon.com/codesuite/codebuild/projects/{}/build/{}/log",
            Region::default().name(),
            build.clone().project_name.unwrap(),
            build.clone().id.unwrap().replace(':', "%3A")
        );

        build_information.push(BuildInformation {
            name: build.clone().project_name.unwrap(),
            status: build_status,
            timestamp: timestamp.to_rfc2822(),
            url,
        });
    }

    #[cfg(feature = "default")]
    terminal::TerminalOutput::print(&build_information);

    #[cfg(feature = "web")]
    WebOutput::print(&build_information);
}