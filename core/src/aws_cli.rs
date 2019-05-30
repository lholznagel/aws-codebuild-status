use aws_codebuild_status_derive::{BuildInformation, Status};
use chrono::{offset::TimeZone, Utc};
use rusoto_codebuild::{
    BatchGetBuildsInput, BatchGetProjectsInput, Build, CodeBuild, CodeBuildClient,
    ListBuildsForProjectInput, ListProjectsInput,
};
use rusoto_codecommit::{
    BranchInfo, CodeCommit, CodeCommitClient, GetBranchInput, ListBranchesInput,
};
use rusoto_core::Region;

#[derive(Clone, Debug, Default)]
pub struct AWSBuildProject {
    build_ids: Vec<String>,
    builds: Vec<Build>,
    branches: Vec<BranchInfo>,
    pub project_name: String,
    pub repository_name: String
}

impl AWSBuildProject {
    pub fn get_build_information(&mut self) -> Vec<BuildInformation> {
        let mut build_information = Vec::new();

        for build in self.builds.iter() {
            let build_status = build.clone().build_status.unwrap();

            if build.clone().resolved_source_version.is_none() {
                continue;
            }

            let commit_id = build.clone().resolved_source_version.unwrap();
            let timestamp = Utc.timestamp(build.clone().end_time.unwrap() as i64, 0);
            let url = format!(
                "https://{}.console.aws.amazon.com/codesuite/codebuild/projects/{}/build/{}/log",
                Region::default().name(),
                build.clone().project_name.unwrap(),
                build.clone().id.unwrap().replace(':', "%3A")
            );
            let mut branch = String::from("");

            for branch_info in self.branches.clone() {
                if branch_info.commit_id.unwrap() == commit_id {
                    branch = branch_info.branch_name.unwrap();
                    break;
                }
            }

            build_information.push(BuildInformation {
                branch,
                commit_id,
                project_name: self.project_name.clone(),
                repository_name: self.repository_name.clone(),
                status: Status::from(build_status),
                timestamp: timestamp.to_rfc2822(),
                url,
            });
        }

        build_information
    }
}

pub struct AWSCli {
    codebuild_client: CodeBuildClient,
    codecommit_client: CodeCommitClient,
}

impl AWSCli {
    pub fn new() -> Self {
        let codebuild_client = CodeBuildClient::new(Region::default());
        let codecommit_client = CodeCommitClient::new(Region::default());

        Self {
            codebuild_client,
            codecommit_client,
        }
    }

    pub fn gather_information(&mut self) -> Vec<AWSBuildProject> {
        let mut info: Vec<AWSBuildProject> = Vec::new();

        for project in self.get_build_projects() {
            let mut current = AWSBuildProject::default();
            current.project_name = project.clone();
            current.repository_name = self.get_project_source(project.clone());
            current.build_ids = self.get_project_builds(project.clone());
            current.builds = self.get_builds(current.build_ids.clone());

            for branch in self.get_branches(current.repository_name.clone()) {
                current
                    .branches
                    .push(self.get_branch_info(branch, current.repository_name.clone()));
            }

            info.push(current);
        }

        info
    }

    fn get_build_projects(&self) -> Vec<String> {
        self.codebuild_client
            .list_projects(ListProjectsInput::default())
            .sync()
            .unwrap()
            .projects
            .unwrap()
    }

    fn get_project_source(&self, build_project: String) -> String {
        let projects = self
            .codebuild_client
            .batch_get_projects(BatchGetProjectsInput {
                names: vec![build_project],
            })
            .sync()
            .unwrap()
            .projects
            .unwrap();

        let location = projects[0].clone().source.unwrap().location.unwrap();
        let splitted = location.split('/').collect::<Vec<&str>>();
        splitted.last().unwrap().to_string()
    }

    fn get_project_builds(&self, project: String) -> Vec<String> {
        self.codebuild_client
            .list_builds_for_project(ListBuildsForProjectInput {
                project_name: project,
                ..Default::default()
            })
            .sync()
            .unwrap()
            .ids
            .unwrap()
    }

    fn get_builds(&self, build_ids: Vec<String>) -> Vec<Build> {
        self.codebuild_client
            .batch_get_builds(BatchGetBuildsInput { ids: build_ids })
            .sync()
            .unwrap()
            .builds
            .unwrap()
    }

    fn get_branches(&self, project: String) -> Vec<String> {
        self.codecommit_client
            .list_branches(ListBranchesInput {
                repository_name: project,
                ..Default::default()
            })
            .sync()
            .unwrap()
            .branches
            .unwrap()
    }

    fn get_branch_info(&self, branch: String, project: String) -> BranchInfo {
        self.codecommit_client
            .get_branch(GetBranchInput {
                branch_name: Some(branch),
                repository_name: Some(project.clone()),
            })
            .sync()
            .unwrap()
            .branch
            .unwrap_or_default()
    }
}
