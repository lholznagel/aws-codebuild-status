mod output {
    mod terminal;
    mod web;

    pub use self::terminal::*;
    pub use self::web::*;
}

use aws_codebuild_status_aws::{Aws, BuildInformation, CodebuildOutput};

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

    let mut aws = Aws::default();
    let mut infos = aws.gather_information();
    let mut map: HashMap<String, Vec<BuildInformation>> = HashMap::new();

    for (name, project) in infos.iter_mut() {
        for build_info in project.get_build_information() {
            if matches.is_present("failed") && !build_info.status.is_failed() {
                continue;
            }
            let mut tag_matches = true;
            for user_tag in matches.values_of("tag").unwrap_or_default() {
                if user_tag.contains(':') {
                    let splitted = user_tag.split(':').collect::<Vec<_>>();

                    if !project.tags.contains_key(splitted[0]) {
                        tag_matches = false;
                        continue;
                    }

                    if let Some(value) = project.tags.get(splitted[0]) {
                        if value != splitted[1] {
                            tag_matches = false;
                            continue;
                        }
                    }
                } else {
                    continue;
                }

                if !tag_matches {
                    continue;
                }

            }

            if !tag_matches {
                continue;
            }


            map.entry(name.to_string())
                .and_modify(|x| x.push(build_info.clone()))
                .or_insert_with(|| vec![build_info]);
        }
    }

    TerminalOutput::print(map.clone());

    if matches.is_present("web") {
        WebOutput::print(map.clone());
    }
}