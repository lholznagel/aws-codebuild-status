mod aws;
mod project;
mod output {
    mod output;
    mod terminal;
    mod web;

    pub use self::output::*;
    pub use self::terminal::*;
    pub use self::web::*;
}

use aws::Aws;
use output::{CodebuildOutput, TerminalOutput, WebOutput};
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

    let mut aws = Aws::new();
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