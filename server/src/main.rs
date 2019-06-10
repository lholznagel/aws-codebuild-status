mod build;

use actix_web::{web, App, HttpResponse, HttpServer, Result};
use askama::Template;
use aws_codebuild_status_aws::{Aws, CodeBuildResult};
use loggify::LogBuilder;
use std::collections::HashMap;
use std::io;
use std::sync::{Arc, Mutex};

#[derive(Template)]
#[template(path = "codebuild_overview.html")]
pub struct TemplateData {
    build_information: HashMap<String, Vec<CodeBuildResult>>,
}

fn index(state: web::Data<Mutex<Vec<CodeBuildResult>>>) -> Result<HttpResponse> {
    let mut project_build: HashMap<String, Vec<CodeBuildResult>> = HashMap::new();
    for build in state.lock().unwrap().clone() {
        project_build
            .entry(build.project_name.clone())
            .and_modify(|x| x.push(build.clone()))
            .or_insert_with(|| vec![build]);
    }

    let template = TemplateData {
        build_information: project_build,
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap()))
}

fn main() -> io::Result<()> {
    LogBuilder::new()
        .add_exclude("hyper".to_string())
        .add_exclude("tokio_reactor".to_string())
        .add_exclude("rusoto".to_string())
        .set_level(log::Level::Debug)
        .build()
        .unwrap();

    let sys = actix_rt::System::new("aws-codebuild-status");

    let aws = Aws::default();
    let builds = aws.fetch_all_builds();

    HttpServer::new(move || {
        App::new()
            .register_data(web::Data::new(Arc::new(Mutex::new(builds.clone()))))
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/builds").route(web::get().to(crate::build::get_builds)))
    })
    .bind("127.0.0.1:8081")?
    .start();

    sys.run()
}
