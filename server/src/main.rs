use actix_web::{web, App, HttpResponse, HttpServer, Result};
use askama::Template;
use aws_codebuild_status_aws::{Aws, CodeBuildResult};
use loggify::LogBuilder;
use std::collections::HashMap;
use std::io;
use std::sync::Mutex;

#[derive(Template)]
#[template(path = "codebuild_overview.html")]
struct TemplateData {
    build_information: HashMap<String, Vec<CodeBuildResult>>,
}

fn index() -> Result<HttpResponse> {
    let aws = Aws::default();
    let builds = aws.fetch_all_builds();
    let mut project_build: HashMap<String, Vec<CodeBuildResult>> = HashMap::new();
    for build in builds {
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
            .register_data(web::Data::new(Mutex::new(builds.clone())))
            .service(web::resource("/").route(web::get().to(index)))
    })
    .bind("127.0.0.1:8081")?
    .start();

    sys.run()
}
