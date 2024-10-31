use super::DB_VERSION;
use crate::entity_record::RedepmtionRecord;

pub const REDEMPTION_TABLE_NAME: &str = "redemption";

/// This function is literally used to create a new redemption table in the
/// database if anyone does not exist already
pub async fn create_redemption_table(
    db_client: &mut tokio_postgres::Client,
) -> Result<(), anyhow::Error> {
    let executable = format!(
        "
               CREATE TABLE IF NOT EXISTS {REDEMPTION_TABLE_NAME}{DB_VERSION} (
                   id              SERIAL PRIMARY KEY,
                   source_token    VARCHAR NULL,
                   dest_token  VARCHAR NULL,
                   source_amount        VARCHAR NULL,
                   dest_amount    VARCHAR NULL,
                   user_address        VARCHAR NULL,
                   onchain_tx_hash        VARCHAR NULL,
                   redeemed_at VARCHAR NULL,
                   created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
        "
    );

    db_client.batch_execute(&executable).await?;
    Ok(())
}

/// This function creates a new redemption entity in the database
pub async fn create_redemption(
    source_token: String,
    dest_token: String,
    source_amount: String,
    dest_amount: String,
    user_address: String,
    onchain_tx_hash: String,
    redeemed_at: String,
    db_client: &mut tokio_postgres::Client,
) -> Result<(), anyhow::Error> {
    let executable = format!(
        "
            INSERT INTO {REDEMPTION_TABLE_NAME}{DB_VERSION} (source_token, dest_token, source_amount, dest_amount, user_address, onchain_tx_hash, redeemed_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
        "
    );

    db_client
        .execute(
            &executable,
            &[
                &source_token,
                &dest_token,
                &source_amount,
                &dest_amount,
                &user_address,
                &onchain_tx_hash,
                &redeemed_at,
            ],
        )
        .await?;

    Ok(())
}

// This function is used to the count of redemption records in the database
pub async fn get_total_redemptions_count(
    db_client: &mut tokio_postgres::Client,
) -> Result<i64, anyhow::Error> {
    let executable = format!(
        "
            SELECT COUNT(*) 
            FROM {REDEMPTION_TABLE_NAME}{DB_VERSION};
        "
    );

    let result = db_client.query_one(&executable, &[]).await?;
    let count: i64 = result.get(0);

    Ok(count)
}

pub async fn get_total_redemption_on_reward_count(
    db_client: &mut tokio_postgres::Client,
    reward_token: String,
) -> Result<i64, anyhow::Error> {
    let executable = format!(
        "
            SELECT COUNT(*) 
            FROM {REDEMPTION_TABLE_NAME}{DB_VERSION}
            WHERE source_token = $1 OR dest_token = $1;
        "
    );

    let result = db_client.query_one(&executable, &[&reward_token]).await?;
    let count: i64 = result.get(0);

    Ok(count)
}

pub async fn get_total_redemption_on_user_count(
    db_client: &mut tokio_postgres::Client,
    user_address: String,
) -> Result<i64, anyhow::Error> {
    let executable = format!(
        "
            SELECT COUNT(*) 
            FROM {REDEMPTION_TABLE_NAME}{DB_VERSION}
            WHERE user_address = $1;
        "
    );

    let result = db_client.query_one(&executable, &[&user_address]).await?;
    let count: i64 = result.get(0);

    Ok(count)
}

pub async fn query_all_redemptions_paginated(
    db_client: &mut tokio_postgres::Client,
    page_number: i64,
    page_size: i64,
) -> Result<Vec<RedepmtionRecord>, anyhow::Error> {
    let executable = format!(
        "
            SELECT * 
            FROM {REDEMPTION_TABLE_NAME}{DB_VERSION}
            ORDER BY id DESC
            LIMIT $1 OFFSET $2;
        "
    );

    let offest = (page_number - 1) * page_size;

    let result = db_client.query(&executable, &[&page_size, &offest]).await?;
    let mut redeption_buffer = Vec::new();

    for record in result {
        let redemption = RedepmtionRecord {
            source_token: record.get(1),
            dest_token: record.get(2),
            source_amount: record.get(3),
            dest_amount: record.get(4),
            user_address: record.get(5),
            onchain_tx_hash: record.get(6),
            redeemed_at: record.get(7),
            create_at: record.get(8),
        };

        redeption_buffer.push(redemption);
    }

    Ok(redeption_buffer)
}

pub async fn query_all_redemptions_paginated_by_reward_address(
    db_client: &mut tokio_postgres::Client,
    page_number: i64,
    page_size: i64,
    reward_address: String,
) -> Result<Vec<RedepmtionRecord>, anyhow::Error> {
    let executable = format!(
        "
            SELECT * 
            FROM {REDEMPTION_TABLE_NAME}{DB_VERSION}
            WHERE source_token = $1 OR dest_token = $1
            ORDER BY id DESC
            LIMIT $2 OFFSET $3;
        "
    );

    let offest = (page_number - 1) * page_size;

    let result = db_client
        .query(&executable, &[&reward_address, &page_size, &offest])
        .await?;
    let mut redeption_buffer = Vec::new();

    for record in result {
        let redemption = RedepmtionRecord {
            source_token: record.get(1),
            dest_token: record.get(2),
            source_amount: record.get(3),
            dest_amount: record.get(4),
            user_address: record.get(5),
            onchain_tx_hash: record.get(6),
            redeemed_at: record.get(7),
            create_at: record.get(8),
        };

        redeption_buffer.push(redemption);
    }

    Ok(redeption_buffer)
}

pub async fn query_all_redemptions_paginated_by_user_address(
    db_client: &mut tokio_postgres::Client,
    page_number: i64,
    page_size: i64,
    user_address: String,
) -> Result<Vec<RedepmtionRecord>, anyhow::Error> {
    let executable = format!(
        "
            SELECT * 
            FROM {REDEMPTION_TABLE_NAME}{DB_VERSION}
            WHERE user_address = $1
            ORDER BY id DESC
            LIMIT $2 OFFSET $3;
        "
    );

    let offest = (page_number - 1) * page_size;

    let result = db_client
        .query(&executable, &[&user_address, &page_size, &offest])
        .await?;
    let mut redeption_buffer = Vec::new();

    for record in result {
        let redemption = RedepmtionRecord {
            source_token: record.get(1),
            dest_token: record.get(2),
            source_amount: record.get(3),
            dest_amount: record.get(4),
            user_address: record.get(5),
            onchain_tx_hash: record.get(6),
            redeemed_at: record.get(7),
            create_at: record.get(8),
        };

        redeption_buffer.push(redemption);
    }

    Ok(redeption_buffer)
}

pub async fn query_redemption_by_onchain_tx_hash(
    db_client: &mut tokio_postgres::Client,
    onchain_tx_hash: String,
) -> Result<RedepmtionRecord, anyhow::Error> {
    let executable = format!(
        "
            SELECT * 
            FROM {REDEMPTION_TABLE_NAME}{DB_VERSION}
            WHERE onchain_tx_hash = $1;
        "
    );

    let result = db_client
        .query_one(&executable, &[&onchain_tx_hash])
        .await?;

    let redemption = RedepmtionRecord {
        source_token: result.get(1),
        dest_token: result.get(2),
        source_amount: result.get(3),
        dest_amount: result.get(4),
        user_address: result.get(5),
        onchain_tx_hash: result.get(6),
        redeemed_at: result.get(7),
        create_at: result.get(8),
    };

    Ok(redemption)
}
