use askama::Template;
use aws_codebuild_status_derive::{BuildInformation, CodebuildOutput};
use std::collections::HashMap;
use std::fs;

#[derive(Template)]
#[template(path = "codebuild.html")]
struct TemplateData {
    build_information: HashMap<String, Vec<BuildInformation>>,
}

pub struct WebOutput;

impl CodebuildOutput for WebOutput {
    fn print(build_information: HashMap<String, Vec<BuildInformation>>) {
        let template = TemplateData {
            build_information,
        };
        fs::write("/tmp/aws-codebuild-status.html", template.render().unwrap()).expect("Unable to write file");
        println!("Saved static website at: file:///tmp/aws-codebuild-status.html");
    }
}