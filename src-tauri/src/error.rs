
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("币安报错: {0}")]
    BinanceError(String),

    #[error("数据库报错: {0}")]
    DbError(String),

    #[error("URL解析错误: {0}")]
    UrlError(String),
}



