use sea_orm::{Database, DatabaseConnection};
use std::error::Error;

pub async fn init(database_url: &str) -> Result<DatabaseConnection, Box<dyn Error>> {
    let db = Database::connect(database_url).await?;
    Ok(db)
}
