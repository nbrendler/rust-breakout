use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct GameError(pub String);

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for GameError {}

impl From<image::ImageError> for GameError {
    fn from(image_error: image::ImageError) -> GameError {
        let msg = image_error.source().unwrap();
        GameError(msg.to_string())
    }
}
