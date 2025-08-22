pub trait OptionExt<T> {
    fn unpack(self) -> Result<T, OptionError>;
    fn unpack_for(self, param_name: &'static str) -> Result<T, OptionError>;
}

impl<T> OptionExt<T> for Option<T> {
    fn unpack(self) -> Result<T, OptionError> {
        if let Some(v) = self {
            Ok(v)
        } else {
            Err(OptionError::CanNotBeNull)
        }
    }
    fn unpack_for(self, param_name: &'static str) -> Result<T, OptionError> {
        if let Some(v) = self {
            Ok(v)
        } else {
            Err(OptionError::CanNotBeNullFor(param_name))
        }
    }
}

use std::fmt;
use std::error::Error;
 
// 1. 定义错误类型
#[derive(Debug, Clone)]
pub enum OptionError {
    CanNotBeNull,
    CanNotBeNullFor(&'static str),
}
 
// 2. 实现 Display 格式化输出
impl fmt::Display for OptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OptionError::CanNotBeNull => write!(f, "param can not be null"),
            OptionError::CanNotBeNullFor(msg) => write!(f, "{} can not be null", msg),
        }
    }
}

// 3. 实现 Error trait（自动获得 source() 方法）
impl Error for OptionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None // 简单错误无需追溯源头 
    }
}