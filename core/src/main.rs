mod aws_cli;

use aws_codebuild_status_terminal::TerminalOutput;
#[cfg(feature = "web")]
use aws_codebuild_status_web::WebOutput;
use aws_codebuild_status_derive::CodebuildOutput;
use clap::{App, Arg};
use std::collections::HashMap;

fn main() { 
    let matches = App::new("AWS Codebuild status")
        .version("")
        .author("Lars Holznagel <contact@lholznagel.info>")
        .arg(Arg::with_name("branch")
            .short("b")
            .long("branch")
            .value_name("BRANCH")
            .help("Only shows builds from that branch")
            .default_value("all")
            .takes_value(true))
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

    #[cfg(feature = "default")]
    TerminalOutput::print(map.clone());

    #[cfg(feature = "web")]
    WebOutput::print(map.clone());
}