use crate::errors;
use poem::handler;

#[handler]
pub fn hello() -> Result<String, errors::AppError> {
    Err(errors::AppError {
        message: "自定义错误".to_string(),
    })
}
