mod aws_cli;

use aws_codebuild_status_derive::CodebuildOutput;
use aws_codebuild_status_terminal::TerminalOutput;
use aws_codebuild_status_web::WebOutput;
use clap::{crate_authors, crate_description, crate_version, App, Arg};
use std::collections::HashMap;

fn main() {
    let matches = App::new("AWS Codebuild status")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("web")
                .short("w")
                .long("web")
                .value_name("WEB")
                .help("Generates a static web page with additional information")
                .takes_value(false),
        )
        .get_matches();

    let mut aws = aws_cli::AWSCli::new();
    let mut infos = aws.gather_information();
    let mut map = HashMap::new();

    for (name, project) in infos.iter_mut() {
        map.insert(name.to_string(), vec![project.get_build_information()[0].clone()]);
    }

    TerminalOutput::print(map.clone());

    if matches.is_present("web") {
        WebOutput::print(map.clone());
    }
}