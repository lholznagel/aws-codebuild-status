use aws_codebuild_status_derive::{BuildInformation, CodebuildOutput};
use colored::Colorize;
use prettytable::{cell, row, Table};

pub struct TerminalOutput;

impl CodebuildOutput for TerminalOutput {

    fn print(build_info: &[BuildInformation]) {
        let mut table = Table::new();
        table.add_row(row!["#", "Project name", "Status", "Finished"]);

        for (i, build) in build_info.iter().enumerate() {
            let status = match build.status.as_ref() {
                "SUCCEEDED" => "SUCCEEDED".green(),
                "IN_PROGRESS" => "IN_PROGRESS".yellow(),
                "FAILED" => "FAILED".red(),
                "TIMED_OUT" => "TIMED_OUT".red(),
                "STOPPED" => "STOPPED".red(),
                _ => "UNDEFINED".red(),
            };

            table.add_row(row![i, build.name, status, build.timestamp]);
        }

        table.printstd();
    }
}