use thiserror::Error;
use web_sys::Element;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Window not found")]
    WindowNotFound,

    #[error("Document not found")]
    DocumentNotFound,

    #[error("Location not found")]
    LocationNotFound,

    #[error("Specified selectors `{0}` is invalid")]
    InvalidSelectors(String),

    #[error("Could not found element by `{0}`")]
    ElementNotFound(String),

    #[error("Object is not an Element")]
    IsNotAnElement,

    #[error("Could not cast element {0:?}")]
    ElementNotCast(Element),
}
