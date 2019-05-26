#[derive(Clone, Debug)]
pub struct BuildInformation {
    pub name: String,
    pub status: String,
    pub timestamp: String,
    pub url: String,
}

pub trait CodebuildOutput {

    fn print(build_info: &[BuildInformation]);
}