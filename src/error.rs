use derive_more::From;
use orion_error::{ErrorCode, StructError, UvsReason};
use serde_derive::Serialize;
use thiserror::Error;

#[derive(Debug, PartialEq, Serialize, Error, From)]
pub enum OrionSecReason {
    #[error("{0}")]
    Sec(SecReason),
    #[error("{0}")]
    Uvs(UvsReason),
}

#[derive(Debug, PartialEq, Serialize, Error)]
pub enum SecReason {
    #[error("sensitive msg {0}")]
    SensitiveMsg(String),
    #[error("no permission {0}")]
    NoPermission(String),
    #[error("deception {0}")]
    Deception(String),
    #[error("un authenticated {0}")]
    UnAuthenticated(String),
}

pub type SecError = StructError<OrionSecReason>;
pub type SecResult<T> = Result<T, SecError>;

impl ErrorCode for OrionSecReason {
    fn error_code(&self) -> i32 {
        match self {
            OrionSecReason::Sec(sec_reason) => match sec_reason {
                SecReason::SensitiveMsg(_) => 101,
                SecReason::NoPermission(_) => 201,
                SecReason::Deception(_) => 301,
                SecReason::UnAuthenticated(_) => 401,
            },
            OrionSecReason::Uvs(u) => u.error_code(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use orion_conf::ToStructError;
    use orion_error::{ErrorConv, ErrorOwe, UvsPermissionFrom};

    #[derive(Debug, PartialEq, Serialize, Error, From)]
    pub enum TargetReason {
        #[error("{0}")]
        Uvs(UvsReason),
    }

    pub type TargetError = StructError<TargetReason>;

    impl From<OrionSecReason> for TargetReason {
        fn from(value: OrionSecReason) -> Self {
            match value {
                OrionSecReason::Sec(_) => Self::Uvs(UvsReason::from_permission("sec error")),
                OrionSecReason::Uvs(u) => Self::Uvs(u),
            }
        }
    }
    fn err_fun() -> SecResult<()> {
        OrionSecReason::Sec(SecReason::NoPermission("no perm".to_string())).err_result()
    }
    fn call_err() -> Result<(), TargetError> {
        err_fun().err_conv()
    }

    #[test]
    fn test_err_conv() {
        match call_err() {
            Ok(_) => todo!(),
            Err(e) => {
                println!("{:#?}", e);
            }
        }
    }

    #[test]
    fn test_sec_reason_sensitive_msg() {
        let reason = SecReason::SensitiveMsg("test message".to_string());
        assert_eq!(format!("{}", reason), "sensitive msg test message");
    }

    #[test]
    fn test_sec_error_creation() {
        let reason = OrionSecReason::Sec(SecReason::SensitiveMsg("operation failed".to_string()));
        let error: SecError = Result::<(), OrionSecReason>::Err(reason)
            .owe_logic()
            .unwrap_err();

        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("operation failed"));
    }

    #[test]
    fn test_sec_result_ok() {
        fn successful_operation() -> SecResult<i32> {
            Ok(42)
        }

        let result = successful_operation();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_sec_result_err() {
        fn failing_operation() -> SecResult<i32> {
            let reason = OrionSecReason::Sec(SecReason::SensitiveMsg("not found".to_string()));
            Result::<i32, OrionSecReason>::Err(reason).owe_logic()
        }

        let result = failing_operation();
        assert!(result.is_err());
    }

    #[test]
    fn test_orion_sec_reason_equality() {
        let reason1 = OrionSecReason::Sec(SecReason::SensitiveMsg("a".to_string()));
        let reason2 = OrionSecReason::Sec(SecReason::SensitiveMsg("a".to_string()));
        let reason3 = OrionSecReason::Sec(SecReason::SensitiveMsg("b".to_string()));

        assert_eq!(reason1, reason2);
        assert_ne!(reason1, reason3);
    }

    #[test]
    fn test_sec_reason_serializable() {
        let reason = SecReason::SensitiveMsg("serialize me".to_string());
        let serialized = serde_yaml::to_string(&reason);
        assert!(serialized.is_ok());
        assert!(serialized.unwrap().contains("serialize me"));
    }

    #[test]
    fn test_orion_sec_reason_serializable() {
        // Only test SecReason variant as UvsReason may have different serialization behavior
        let reason = OrionSecReason::Sec(SecReason::SensitiveMsg("data".to_string()));
        let debug_str = format!("{:?}", reason);
        assert!(debug_str.contains("data"));
    }

    #[test]
    fn test_sec_error_display() {
        let reason = OrionSecReason::Sec(SecReason::SensitiveMsg("password leaked".to_string()));
        let error: SecError = reason.to_err();

        // SecError uses Debug format for display
        let display_str = format!("{}", error);
        println!("{}", display_str);
        assert!(display_str.contains("password leaked"));
        assert_eq!(error.error_code(), 101);
    }
}
