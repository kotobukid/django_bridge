use std::fmt;

#[derive(Debug)]
pub enum CacherError {
    Io(std::io::Error),
    Request(reqwest::Error),
    UnknownProductType(String),
    CacheNotFound,
    ParseError(String),
}

impl fmt::Display for CacherError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CacherError::Io(e) => write!(f, "IO error: {}", e),
            CacherError::Request(e) => write!(f, "Request error: {}", e),
            CacherError::UnknownProductType(t) => write!(f, "Unknown product type: {}", t),
            CacherError::CacheNotFound => write!(f, "Cache not found"),
            CacherError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for CacherError {}

impl From<std::io::Error> for CacherError {
    fn from(error: std::io::Error) -> Self {
        CacherError::Io(error)
    }
}

impl From<reqwest::Error> for CacherError {
    fn from(error: reqwest::Error) -> Self {
        CacherError::Request(error)
    }
}

pub type Result<T> = std::result::Result<T, CacherError>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_cacher_error_display() {
        let io_error = CacherError::Io(io::Error::new(io::ErrorKind::NotFound, "file not found"));
        assert!(format!("{}", io_error).contains("IO error"));
        assert!(format!("{}", io_error).contains("file not found"));

        let unknown_type_error = CacherError::UnknownProductType("invalid".to_string());
        assert_eq!(
            format!("{}", unknown_type_error),
            "Unknown product type: invalid"
        );

        let cache_not_found_error = CacherError::CacheNotFound;
        assert_eq!(format!("{}", cache_not_found_error), "Cache not found");

        let parse_error = CacherError::ParseError("invalid format".to_string());
        assert_eq!(format!("{}", parse_error), "Parse error: invalid format");
    }

    #[test]
    fn test_from_io_error() {
        let io_error = io::Error::new(io::ErrorKind::PermissionDenied, "permission denied");
        let cacher_error: CacherError = io_error.into();

        match cacher_error {
            CacherError::Io(_) => (), // 期待される結果
            _ => panic!("Expected Io error variant"),
        }
    }

    #[test]
    fn test_error_trait() {
        let error = CacherError::UnknownProductType("test".to_string());

        // std::error::Errorトレイトが実装されていることを確認
        let _: &dyn std::error::Error = &error;
    }

    #[test]
    fn test_debug_trait() {
        let error = CacherError::CacheNotFound;
        let debug_str = format!("{:?}", error);
        assert_eq!(debug_str, "CacheNotFound");
    }

    #[test]
    fn test_result_type_alias() {
        fn returns_result() -> Result<String> {
            Ok("success".to_string())
        }

        fn returns_error() -> Result<String> {
            Err(CacherError::CacheNotFound)
        }

        assert!(returns_result().is_ok());
        assert!(returns_error().is_err());
    }
}
