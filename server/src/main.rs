use actix_web::{web, App, HttpResponse, HttpServer, Result};
use askama::Template;
use aws_codebuild_status_aws::{Aws, BuildInformation};
use loggify::Loggify;
use std::collections::HashMap;
use std::io;

#[derive(Template)]
#[template(path = "codebuild_overview.html")]
struct TemplateData {
    build_information: HashMap<String, Vec<BuildInformation>>,
}

fn index() -> Result<HttpResponse> {
    let mut aws = Aws::default();
    let mut infos = aws.gather_information();
    let mut map: HashMap<String, Vec<BuildInformation>> = HashMap::new();

    for (name, project) in infos.iter_mut() {
        map.insert(name.to_string(), project.get_build_information());
    }

    let mut reduced_map = HashMap::new();
    for (key, value) in map {
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
    Loggify::init().unwrap();
    let sys = actix_rt::System::new("basic-example");

    HttpServer::new(|| App::new().service(web::resource("/").route(web::get().to(index))))
        .bind("127.0.0.1:8081")?
        .start();

    sys.run()
}
