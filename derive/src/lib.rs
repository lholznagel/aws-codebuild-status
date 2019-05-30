mod status;

pub use crate::status::Status;

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct BuildInformation {
    pub branch: String,
    pub commit_id: String,
    pub project_name: String,
    pub repository_name: String,
    pub status: Status,
    pub timestamp: String,
    pub url: String,
}

pub trait CodebuildOutput {

    fn print(build_info: HashMap<String, Vec<BuildInformation>>);
}