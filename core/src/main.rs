mod aws_cli;

use aws_codebuild_status_derive::CodebuildOutput;
use aws_codebuild_status_terminal::TerminalOutput;
use aws_codebuild_status_web::WebOutput;
use clap::{App, Arg, crate_authors, crate_description, crate_version};
use std::collections::HashMap;

fn main() {
    let matches = App::new("AWS Codebuild status")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("branch")
                .short("b")
                .long("branch")
                .value_name("BRANCH")
                .help("Only shows builds from that branch")
                .default_value("all")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("web")
                .short("w")
                .long("web")
                .value_name("WEB")
                .help("Generates a static web page with additional information")
                .takes_value(false),
        )
        .get_matches();

    let branch = matches.value_of("branch").unwrap();
    let mut aws = aws_cli::AWSCli::new();
    let mut infos = aws.gather_information();
    let mut map = HashMap::new();

    for info in infos.iter_mut() {
        let mut build_information = Vec::new();

        for build_info in info.get_build_information() {
            if branch == "all" || build_info.branch == branch {
                build_information.push(build_info);
                break;
            }
        }

        map.insert(info.name.clone(), build_information);
    }

    TerminalOutput::print(map.clone());

    if matches.is_present("web") {
        WebOutput::print(map.clone());
    }
}