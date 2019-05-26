use askama::Template;
use aws_codebuild_status_derive::{BuildInformation, CodebuildOutput};
use std::fs;

#[derive(Template)]
#[template(path = "view.html")]
struct TemplateData {
    build_information: Vec<BuildInformation>,
}

pub struct WebOutput;

impl CodebuildOutput for WebOutput {
    fn print(build_information: &[BuildInformation]) {
        let template = TemplateData {
            build_information: build_information.to_vec(),
        };
        fs::write("/tmp/foo.html", template.render().unwrap()).expect("Unable to write file");
    }
}