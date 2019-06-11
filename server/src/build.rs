use crate::template::TemplateData;
use actix_web::{web, HttpResponse, Result};
use askama::Template;
use aws_codebuild_status_aws::{Aws, CodeBuildResult};
use log::debug;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub fn get_builds(state: web::Data<Arc<Mutex<Vec<CodeBuildResult>>>>) -> Result<HttpResponse> {
    update_internal_state(&state, None);

    let mut project_build: HashMap<String, Vec<CodeBuildResult>> = HashMap::new();
    for build in state.lock().unwrap().clone() {
        project_build
            .entry(build.project_name.clone())
            .and_modify(|x| x.push(build.clone()))
            .or_insert_with(|| vec![build]);
    }

    let mut reduced_map = HashMap::new();
    for (key, value) in project_build {
        if value.is_empty() {
            continue;
        }

        reduced_map.insert(key, vec![value[0].clone()]);
    }

    let template = TemplateData {
        build_information: reduced_map,
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
