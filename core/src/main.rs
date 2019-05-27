mod aws;
mod terminal;

#[cfg(feature = "web")]
use aws_codebuild_status_web::WebOutput;

use aws_codebuild_status_derive::CodebuildOutput;

fn main() {
    let mut aws = aws::AWS::new();
    let build_information = aws.get_build_information();

    #[cfg(feature = "default")]
    terminal::TerminalOutput::print(&build_information);

    #[cfg(feature = "web")]
    WebOutput::print(&build_information);
}