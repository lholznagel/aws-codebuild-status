use crate::Status;
use chrono::{offset::TimeZone, Utc};
use rusoto_codebuild::Build;
use rusoto_core::Region;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
pub struct CodeBuildResult {
    pub commit_id: String,
    pub project_name: String,
    pub repository_name: String,
    pub status: Status,
    pub tags: HashMap<String, String>,
    pub timestamp: String,
    pub url: String,
}

impl CodeBuildResult {
    pub fn is_failed(&self) -> bool {
        self.status.is_failed()
    }

    pub fn has_tag(&self, key: &str, value: &str) -> bool {
        if self.tags.contains_key(key) {
            self.tags.get(key).unwrap() == value
        } else {
            false
        }
    }
}

impl From<Build> for CodeBuildResult {
    fn from(build: Build) -> Self {
        let build_status = build.clone().build_status.unwrap();

        let commit_id = if build.clone().resolved_source_version.is_none() {
            build.clone().source_version.unwrap()
        } else {
            build.clone().resolved_source_version.unwrap()
        };

        let mut timestamp = String::from("Unknown");
        if build.clone().end_time.is_some() {
            timestamp = Utc
                .timestamp(build.clone().end_time.unwrap() as i64, 0)
                .to_rfc2822();
        };

        let url = format!(
            "https://{}.console.aws.amazon.com/codesuite/codebuild/projects/{}/build/{}/log",
            Region::default().name(),
            build.clone().project_name.unwrap(),
            build.clone().id.unwrap().replace(':', "%3A")
        );

        let location = build
            .clone()
            .source
            .unwrap()
            .location
            .unwrap_or_else(|| String::from("Undefined"));
        let splitted = location.split('/').collect::<Vec<&str>>();
        let repository_name = splitted.last().unwrap().to_string();

        Self {
            commit_id,
            project_name: build.project_name.clone().unwrap(),
            repository_name,
            status: Status::from(build_status),
            tags: HashMap::new(),
            timestamp,
            url,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tags() {
        let mut map = HashMap::new();
        map.insert("tag".to_string(), "test".to_string());

        let result = CodeBuildResult {
            commit_id: String::new(),
            project_name: String::new(),
            repository_name: String::new(),
            status: Status::Undefined,
            tags: map,
            timestamp: String::new(),
            url: String::new(),
        };

        assert!(result.has_tag("tag", "test"));
        assert!(!result.has_tag("another", "tag"));
    }
}
