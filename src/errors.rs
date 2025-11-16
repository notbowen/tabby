#[derive(Debug)]
pub struct FenParseError(pub String);

impl std::fmt::Display for FenParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error parsing FEN: {}", self.0)
    }
}

impl std::error::Error for FenParseError {}
