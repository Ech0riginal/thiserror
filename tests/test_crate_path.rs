use std::error::Error as StdError;
use std::fmt::{self, Display};

// Re-export thiserror under a different name to simulate a workspace setup
// where thiserror is not available at ::thiserror
mod my_error_lib {
    pub use thiserror::*;
}

// Test multi-segment path like common::thiserror
mod common {
    pub mod thiserror {
        pub use thiserror::*;
    }
}

#[test]
fn test_crate_path_struct() {
    #[derive(Debug, my_error_lib::Error)]
    #[thiserror(crate = "my_error_lib")]
    #[error("simple error")]
    struct SimpleError;

    let err = SimpleError;
    assert_eq!(err.to_string(), "simple error");
}

#[test]
fn test_crate_path_enum() {
    #[derive(Debug, my_error_lib::Error)]
    #[thiserror(crate = "my_error_lib")]
    enum MyError {
        #[error("variant a")]
        VariantA,
        #[error("variant b")]
        VariantB,
    }

    let err = MyError::VariantA;
    assert_eq!(err.to_string(), "variant a");
    
    let err = MyError::VariantB;
    assert_eq!(err.to_string(), "variant b");
}

#[test]
fn test_crate_path_with_source() {
    #[derive(Debug)]
    struct InnerError;

    impl Display for InnerError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "inner error")
        }
    }

    impl StdError for InnerError {}

    #[derive(Debug, my_error_lib::Error)]
    #[thiserror(crate = "my_error_lib")]
    #[error("outer error")]
    struct OuterError {
        #[source]
        source: InnerError,
    }

    let err = OuterError {
        source: InnerError,
    };
    assert_eq!(err.to_string(), "outer error");
    assert!(err.source().is_some());
}

#[test]
fn test_crate_path_with_from() {
    #[derive(Debug)]
    struct InnerError;

    impl Display for InnerError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "inner error")
        }
    }

    impl StdError for InnerError {}

    #[derive(Debug, my_error_lib::Error)]
    #[thiserror(crate = "my_error_lib")]
    #[error("outer error")]
    struct OuterError {
        #[from]
        source: InnerError,
    }

    let inner = InnerError;
    let err: OuterError = inner.into();
    assert_eq!(err.to_string(), "outer error");
    assert!(err.source().is_some());
}

#[test]
fn test_crate_path_transparent() {
    #[derive(Debug)]
    struct InnerError;

    impl Display for InnerError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "inner error")
        }
    }

    impl StdError for InnerError {}

    #[derive(Debug, my_error_lib::Error)]
    #[thiserror(crate = "my_error_lib")]
    #[error(transparent)]
    struct TransparentError(InnerError);

    let err = TransparentError(InnerError);
    assert_eq!(err.to_string(), "inner error");
}

#[test]
fn test_multi_segment_crate_path() {
    #[derive(Debug, common::thiserror::Error)]
    #[thiserror(crate = "common::thiserror")]
    enum MyError {
        #[error("variant a")]
        VariantA,
        #[error("variant b")]
        VariantB,
    }

    let err = MyError::VariantA;
    assert_eq!(err.to_string(), "variant a");

    let err = MyError::VariantB;
    assert_eq!(err.to_string(), "variant b");
}

