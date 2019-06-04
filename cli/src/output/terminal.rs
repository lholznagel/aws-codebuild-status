use aws_codebuild_status_aws::{BuildInformation, CodebuildOutput, Status};
use colored::Colorize;
use prettytable::{cell, row, Table};
use std::collections::HashMap;

pub struct TerminalOutput;

impl CodebuildOutput for TerminalOutput {

    fn print(build_info: HashMap<String, Vec<BuildInformation>>) {
        let mut table = Table::new();
        table.add_row(row!["#", "Project name", "Status", "Finished"]);

        for (i, (_, builds)) in build_info.iter().enumerate() {

            let status = match builds[0].status {
                Status::Succeeded => Status::Succeeded.to_string().green(),
                Status::InProgress => Status::InProgress.to_string().yellow(),
                Status::Failed => Status::Failed.to_string().red(),
                Status::TimedOut => Status::TimedOut.to_string().red(),
                Status::Stopped => Status::Stopped.to_string().red(),
                _ => Status::Undefined.to_string().red(),
            };

            table.add_row(row![i, builds[0].project_name, status, builds[0].timestamp]);
        }

        table.printstd();
    }
}