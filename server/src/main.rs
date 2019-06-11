mod build;
mod template;

use actix_web::{web, App, HttpServer};
use aws_codebuild_status_aws::Aws;
use loggify::LogBuilder;
use std::io;
use std::sync::{Arc, Mutex};

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
            .service(web::resource("/").route(web::get().to(crate::build::get_builds)))
    })
    .bind("127.0.0.1:8081")?
    .start();

    sys.run()
}
