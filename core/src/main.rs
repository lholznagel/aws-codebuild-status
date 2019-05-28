mod aws_cli;
mod terminal;

#[cfg(feature = "web")]
use aws_codebuild_status_web::WebOutput;

use aws_codebuild_status_derive::CodebuildOutput;

fn main() {
    let mut aws = aws_cli::AWSCli::new();
    let mut infos = aws.gather_information();
    let mut build_information = Vec::new();

    for info in infos.iter_mut() {
        for build_info in info.get_build_information() {
            if build_info.branch == "master" {
                build_information.push(build_info);
                break;
            }
        }
    }

    #[cfg(feature = "default")]
    terminal::TerminalOutput::print(&build_information);

    #[cfg(feature = "web")]
    WebOutput::print(&build_information);
}