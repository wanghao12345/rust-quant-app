use sqlx::SqlitePool;
use tracing::info;

use crate::error::ServiceError;




pub struct MarketDataStore {
    pool: SqlitePool,
}

impl MarketDataStore {
    pub async fn new(db_path: &str) -> Result<Self, ServiceError> {
        let pool = SqlitePool::connect(&format!("sqlite://{}?mode=rwc", db_path)).await.map_err(|e| ServiceError::DbError(e.to_string()))?;
        // 创建表
        sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS klines (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                symbol TEXT NOT NULL,
                interval TEXT NOT NULL,
                open_time INTEGER NOT NULL,
                open REAL NOT NULL,
                high REAL NOT NULL,
                low REAL NOT NULL,
                close REAL NOT NULL,
                volume REAL NOT NULL,
                close_time INTEGER NOT NULL,
                quote_asset_volume REAL DEFAULT 0,
                number_of_trades INTEGER DEFAULT 0,
                taker_buy_base_asset_volume REAL DEFAULT 0,
                taker_buy_quote_asset_volume REAL DEFAULT 0,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                UNIQUE (symbol, interval, open_time)
            )"#
        ).execute(&pool).await.map_err(|e| ServiceError::DbError(e.to_string()))?;
        
        info!("数据库连接成功: {}", db_path);
        Ok(Self { pool })
    }
}