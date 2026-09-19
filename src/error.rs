#[derive(Debug)]
pub enum MiniRtErr {
    Io(std::io::Error),
    InvalidSceneFile,
    InvalidSceneFileExtension,
    InvalidNumberOfArguments,
}

impl From<std::io::Error> for MiniRtErr {
    fn from(error: std::io::Error) -> Self {
        MiniRtErr::Io(error)
    }
}
