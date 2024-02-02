use std::error::Error;
use std::fmt::{Display, Formatter};

///程序内部异常
#[derive(Debug)]
pub enum  KaMuError{
    ///文件异常
    FileError(String),
    ///转存异常
    UnloadingError(String)
}

impl Display for KaMuError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"KaMuError:{}",self.to_string())
    }
}

impl Error for KaMuError {}