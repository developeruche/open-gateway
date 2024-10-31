//! This function would be used for in-chronicle persistent data management
pub const SYSTEM_TABLE_NAME: &str = "chronicle_system";
use super::{
    brand::create_brand_table, pool::create_pool_table, redemption::create_redemption_table,
    reward::create_reward_table, DB_VERSION,
};

/// This function is literally used to create a new system table in the
/// database if anyone does not exist already
pub async fn create_system_table(
    db_client: &mut tokio_postgres::Client,
) -> Result<(), anyhow::Error> {
    let executable = format!(
        "
               CREATE TABLE IF NOT EXISTS {SYSTEM_TABLE_NAME}{DB_VERSION} (
                   id              SERIAL PRIMARY KEY,
                   last_block_number    VARCHAR NULL
            )
        "
    );

    db_client.batch_execute(&executable).await?;
    create_brand_table(db_client).await?;
    create_pool_table(db_client).await?;
    create_redemption_table(db_client).await?;
    create_reward_table(db_client).await?;

    Ok(())
}

/// This function creates a new system entity in the database
/// though this would be called only once
pub async fn create_system_if_does_not_exist(
    last_block_number: String,
    db_client: &mut tokio_postgres::Client,
) -> Result<(), anyhow::Error> {
    let executable = format!(
        "
            INSERT INTO {SYSTEM_TABLE_NAME}{DB_VERSION} (id, last_block_number)
            VALUES (1, $1)
            ON CONFLICT (id) 
            DO UPDATE SET last_block_number = EXCLUDED.last_block_number;
        "
    );

    db_client
        .execute(&executable, &[&last_block_number])
        .await?;

    Ok(())
}

/// This function is used to get the last block number from the system table
/// in the database
pub async fn get_last_block_number(
    db_client: &mut tokio_postgres::Client,
) -> Result<String, anyhow::Error> {
    let executable = format!(
        "
            SELECT last_block_number FROM {SYSTEM_TABLE_NAME}{DB_VERSION} WHERE id = 1
        "
    );

    let rows = db_client.query(&executable, &[]).await?;
    let last_block_number: String = if rows.is_empty() {
        "0".to_string()
    } else {
        rows[0].get(0)
    };

    Ok(last_block_number)
}

/// This function is used to update the last block number in the system table
/// in the database
pub async fn update_last_block_number(
    last_block_number: String,
    db_client: &mut tokio_postgres::Client,
) -> Result<(), anyhow::Error> {
    let executable = format!(
        "
            UPDATE {SYSTEM_TABLE_NAME}{DB_VERSION} SET last_block_number = $1 WHERE id = 1
        "
    );

    db_client
        .execute(&executable, &[&last_block_number])
        .await?;

    Ok(())
}
