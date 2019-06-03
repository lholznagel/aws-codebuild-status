use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug)]
pub struct BuildInformation {
    pub commit_id: String,
    pub project_name: String,
    pub repository_name: String,
    pub status: Status,
    pub timestamp: String,
    pub url: String,
    pub tags: HashMap<String, String>
}

pub trait CodebuildOutput {

    fn print(build_info: HashMap<String, Vec<BuildInformation>>);
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Status {
    Failed,
    InProgress,
    Stopped,
    Succeeded,
    TimedOut,
    Undefined,
}

impl Status {
    pub fn is_failed(&self) -> bool {
        match self {
            Status::Failed | Status::Stopped | Status::TimedOut => true,
            Status::InProgress | Status::Succeeded => false,
            _ => true,
        }
    }
}

impl From<String> for Status {
    fn from(var: String) -> Self {
        match var.as_ref() {
            "FAILED" => Status::Failed,
            "IN_PROGRESS" => Status::InProgress,
            "STOPPED" => Status::Stopped,
            "SUCCEEDED" => Status::Succeeded,
            "TIMED_OUT" => Status::TimedOut,
            _ => Status::Undefined,
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let name = match self {
            Status::Failed => "FAILED".to_string(),
            Status::InProgress => "IN_PROGRESS".to_string(),
            Status::Stopped => "STOPPED".to_string(),
            Status::Succeeded => "SUCCEEDED".to_string(),
            Status::TimedOut => "TIMED_OUT".to_string(),
            _ => "UNDEFINED".to_string(),
        };

        write!(f, "{}", name)
    }
}