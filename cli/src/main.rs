mod output {
    mod terminal;
    mod web;

    pub use self::terminal::*;
    pub use self::web::*;
}

use aws_codebuild_status_aws::{Aws, BuildInformation, CodebuildOutput, Filter};

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

    let start = std::time::Instant::now();
    let mut aws = Aws::default();
    let mut infos = aws.gather_information();
    let mut map: HashMap<String, Vec<BuildInformation>> = HashMap::new();

    for (name, project) in infos.iter_mut() {
        let tags: Vec<String> = matches
            .values_of("tag")
            .unwrap_or_default()
            .map(|x| x.to_string())
            .collect();

        let build_info = project.get_build_information_with_filter(Filter {
            failed: Some(matches.is_present("failed")),
            tags: Some(tags)
        });

        map.insert(name.to_string(), build_info);
    }

    TerminalOutput::print(map.clone());

    if matches.is_present("web") {
        WebOutput::print(map.clone());
    }
    dbg!(start.elapsed().as_millis());
}