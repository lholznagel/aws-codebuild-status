use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Status {
    Failed,
    Fault,
    InProgress,
    Stopped,
    Succeeded,
    TimedOut,
    Undefined,
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