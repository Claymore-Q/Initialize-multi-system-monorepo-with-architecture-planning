//! Tests for shared_core error handling

use shared_core::error::SystemError;

#[test]
fn test_error_creation() {
    let err = SystemError::validation("field", "Invalid format", Some("value".to_string()));
    assert!(err.to_string().contains("field"));
}

#[test]
fn test_error_serialization() {
    let err = SystemError::config("Missing config".to_string(), Some("key".to_string()));
    let json = serde_json::to_string(&err).unwrap();
    let deserialized: SystemError = serde_json::from_str(&json).unwrap();

    match deserialized {
        SystemError::Config { message, key } => {
            assert_eq!(message, "Missing config");
            assert_eq!(key, Some("key".to_string()));
        }
        _ => panic!("Wrong error type deserialized"),
    }
}

#[test]
fn test_io_error_conversion() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
    let sys_err: SystemError = io_err.into();

    match sys_err {
        SystemError::Io { message, context } => {
            assert!(message.contains("File not found"));
            assert!(context.contains("I/O operation failed"));
        }
        _ => panic!("Wrong error type"),
    }
}
