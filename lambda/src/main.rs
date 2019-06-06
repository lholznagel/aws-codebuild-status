use aws_codebuild_status_aws::{Aws, BuildInformation, Filter};
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
    let mut map: HashMap<String, Vec<BuildInformation>> = HashMap::new();

    for (name, project) in infos.iter_mut() {
        let tags: Vec<String> = e
            .tags
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(|x| x.to_string())
            .collect();

        let build_info = project.get_build_information_with_filter(Filter {
            failed: Some(e.failed.is_some()),
            tags: Some(tags)
        });

        map.insert(name.to_string(), build_info);
    }

    Ok(map)
}