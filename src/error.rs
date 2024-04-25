#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("$1")]
    Generic(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),
    // sdl errors
    #[error(transparent)]
    SdlWindowError(#[from] sdl2::video::WindowBuildError),
    #[error(transparent)]
    SdlCanvasError(#[from] sdl2::IntegerOrSdlError),
}

impl std::convert::From<String> for Error {
    fn from(st: String) -> Self {
        Self::Generic(st.to_string())
    }
}
