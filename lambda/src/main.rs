use aws_codebuild_status_aws::{Aws, BuildInformation};
use lambda_runtime::error::HandlerError;
use lambda_runtime::lambda;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Clone)]
struct Event {
    failed: Option<bool>,
    tags: Option<Vec<String>>
}

fn main() {
    lambda!(my_handler);
}

fn my_handler(e: Event, _: lambda_runtime::Context) -> Result<HashMap<String, Vec<BuildInformation>>, HandlerError> {
    let mut aws = Aws::default();
    let mut infos = aws.gather_information();
    let mut map = HashMap::new();

    for (name, project) in infos.iter_mut() {
        let project_build_info = project.get_build_information()[0].clone();

        if e.failed.unwrap_or(false) && !project_build_info.status.is_failed() {
            continue;
        }

        let mut tag_matches = true;
        for user_tag in e.tags.clone().unwrap_or_default() {
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

        map.insert(name.to_string(), vec![project_build_info]);
    }

    Ok(map)
}