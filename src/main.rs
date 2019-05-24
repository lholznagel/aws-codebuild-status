use chrono::{offset::TimeZone, Utc};
use colored::Colorize;
use prettytable::{cell, row, Table};
use rusoto_codebuild::{
    BatchGetBuildsInput, CodeBuild, CodeBuildClient, ListBuildsForProjectInput, ListProjectsInput,
};
use rusoto_core::credential::ChainProvider;
use rusoto_core::{HttpClient, Region};

fn main() {
    let mut build_ids = Vec::new();
    let mut table = Table::new();
    table.add_row(row!["#", "Project name", "Status", "Finished"]);

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

    let builds = client
        .batch_get_builds(BatchGetBuildsInput { ids: build_ids })
        .sync()
        .unwrap();

    for (i, build) in builds.builds.unwrap().iter().enumerate() {
        let build_status = build.clone().build_status.unwrap();
        let status = match build_status.as_ref() {
            "SUCCEEDED" => "SUCCEEDED".green(),
            "IN_PROGRESS" => "IN_PROGRESS".yellow(),
            "FAILED" => "FAILED".red(),
            "TIMED_OUT" => "TIMED_OUT".red(),
            "STOPPED" => "STOPPED".red(),
            _ => "UNDEFINED".red(),
        };
        let timestamp = Utc.timestamp(build.clone().end_time.unwrap() as i64, 0);

        table.add_row(row![
            i,
            build.clone().project_name.unwrap(),
            status,
            timestamp.to_rfc2822()
        ]);
    }

    table.printstd();
}