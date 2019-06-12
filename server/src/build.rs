use crate::template::TemplateData;
use actix_web::{web, HttpResponse, Result};
use askama::Template;
use aws_codebuild_status_aws::{Aws, CodeBuildResult, Status};
use log::debug;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub fn get_builds(
    state: web::Data<Arc<Mutex<Vec<CodeBuildResult>>>>,
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse> {
    update_internal_state(&state, None);

    let mut builds: Vec<CodeBuildResult> = { state.lock().unwrap().clone() };
    builds.sort_by_key(|x| x.timestamp);

    let mut project_build: HashMap<String, Vec<CodeBuildResult>> = HashMap::new();
    for build in state.lock().unwrap().clone() {
        project_build
            .entry(build.project_name.clone())
            .and_modify(|x| x.push(build.clone()))
            .or_insert_with(|| vec![build]);
    }

    let mut reduces_vec = Vec::new();
    for key in project_build.keys() {
        reduces_vec.push(project_build[key][0].clone());
    }
    reduces_vec.sort_by_key(|x| x.project_name.clone());

    if query.contains_key("name") {
        let name = query.get("name").unwrap();

        if name != "" {
            reduces_vec = reduces_vec
                .into_iter()
                .filter(|x| x.project_name.contains(name))
                .collect();
        }
    }

    if query.contains_key("tag") {
        let tag = query.get("tag").unwrap();
        let splitted: Vec<&str> = tag.split(':').collect();

        if splitted.len() == 2 {
            reduces_vec = reduces_vec
                .into_iter()
                .filter(|x| {
                    x.tags.contains_key(splitted[0])
                        && x.tags.get(splitted[0]).unwrap() == splitted[1]
                })
                .collect();
        }
    }

    if query.contains_key("status") {
        let status = query.get("status").unwrap();

        if status != "ALL" {
            reduces_vec = reduces_vec
                .into_iter()
                .filter(|x| x.status == Status::from(status.to_string()))
                .collect();
        }
    }

    let template = TemplateData {
        codebuild: reduces_vec,
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap()))
}

fn update_internal_state(
    state: &web::Data<Arc<Mutex<Vec<CodeBuildResult>>>>,
    next_token: Option<String>,
) {
    let aws = Aws::default();
    let result = aws.fetch_builds(next_token);

    let old_state = { state.lock().unwrap().clone() };
    let mut added = 0;
    for build in result.1 {
        if !old_state.contains(&build) {
            added += 1;
            {
                let mut state_lock = state.lock().expect("Could not lock mutex.");
                state_lock.push(build);
            }
        } else {
            break;
        }
    }

    if added > 0 {
        debug!("Added {} new builds to internal state", added);
    } else {
        debug!("No new builds to add");
    }

    // it looks like we added 100 new builds
    // so we need to look for more
    if added == 100 {
        debug!("More than 100 new builds, loading more.");
        update_internal_state(state, result.0);
    }
}
