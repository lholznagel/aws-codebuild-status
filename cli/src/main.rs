mod output {
    mod output;
    mod terminal;
    mod web;

    pub use self::output::Output;
    pub use self::terminal::*;
    pub use self::web::*;
}

use crate::output::Output;
use aws_codebuild_status_aws::{Aws, CodeBuildResult};
use clap::{crate_authors, crate_description, crate_version, App, Arg};
use output::{TerminalOutput, WebOutput};
use std::collections::HashMap;

fn main() {
    let matches = App::new("AWS Codebuild status")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("failed")
                .short("f")
                .long("failed")
                .value_name("FAILED")
                .help("Only shows builds that failed")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("web")
                .short("w")
                .long("web")
                .value_name("WEB")
                .help("Generates a static web page with additional information")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("tag")
                .short("t")
                .long("tag")
                .value_name("TAG")
                .help("Filters by tag name. Format: Key:Value")
                .takes_value(true)
                .multiple(true),
        )
        .get_matches();

    let aws = Aws::default();
    let builds: Vec<CodeBuildResult> = aws
        .fetch_all_builds()
        .into_iter()
        .filter(|x| {
            if matches.is_present("failed") {
                return x.is_failed();
            }

            true
        })
        .filter(|x| {
            if !matches.is_present("tag") {
                return true;
            }

            let mut result = true;
            for tag in matches.values_of("tag").unwrap() {
                let splitted = tag.split(':').collect::<Vec<_>>();
                if splitted.len() != 2 {
                    return false;
                }

                if !x.tags.contains_key(splitted[0]) {
                    result = false;
                    continue;
                }

                if let Some(value) = x.tags.get(splitted[0]) {
                    if value != splitted[1] {
                        result = false;
                        continue;
                    }
                }
            }

            result
        })
        .collect();

    let mut project_build: HashMap<String, Vec<CodeBuildResult>> = HashMap::new();

    for build in builds {
        project_build
            .entry(build.project_name.clone())
            .and_modify(|x| x.push(build.clone()))
            .or_insert_with(|| vec![build]);
    }

    TerminalOutput::print(project_build.clone());

    if matches.is_present("web") {
        WebOutput::print(project_build.clone());
    }
}
