use askama::Template;
use aws_codebuild_status_aws::{BuildInformation, CodebuildOutput};
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
        let mut reduced_map = HashMap::new();
        for (key, value) in build_information {
            if value.is_empty() {
                continue;
            }

            reduced_map.insert(key, vec![value[0].clone()]);
        }

        let template = TemplateData { build_information: reduced_map };
        fs::write("/tmp/aws-codebuild-status.html", template.render().unwrap())
            .expect("Unable to write file");
        println!("Saved static website at: file:///tmp/aws-codebuild-status.html");
    }
}