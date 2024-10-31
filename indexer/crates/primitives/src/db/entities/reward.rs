use super::DB_VERSION;
use crate::entity_record::RewardRecord;

pub const REWARD_TABLE_NAME: &str = "reward";

/// This function is literally used to create a new reward table in the
/// database if anyone does not exist already
pub async fn create_reward_table(
    db_client: &mut tokio_postgres::Client,
) -> Result<(), anyhow::Error> {
    let executable = format!(
        "
            CREATE TABLE IF NOT EXISTS {REWARD_TABLE_NAME}{DB_VERSION} (
               id              SERIAL PRIMARY KEY,
               brand_id        VARCHAR NULL,
               reward_address        VARCHAR NULL,
               requestor_address        VARCHAR NULL,
               initial_supply        VARCHAR NULL,
               timestamp        VARCHAR NULL,
               created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
        "
    );
    db_client.batch_execute(&executable).await?;
    Ok(())
}

// function creates a new reward entity in the database
pub async fn create_reward_if_does_not_exist(
    brand_id: String,
    reward_address: String,
    requestor_address: String,
    initial_supply: String,
    timestamp: String,
    db_client: &mut tokio_postgres::Client,
) -> Result<(), anyhow::Error> {
    let executable = format!(
        "
           INSERT INTO {REWARD_TABLE_NAME}{DB_VERSION} (brand_id, reward_address, requestor_address, initial_supply, timestamp)
           VALUES ($1, $2, $3, $4, $5)
        "
    );

    if check_if_reward_exist_by_reward_address(reward_address.clone(), db_client).await? {
        return Ok(());
    }

    db_client
        .execute(
            &executable,
            &[
                &brand_id,
                &reward_address,
                &requestor_address,
                &initial_supply,
                &timestamp,
            ],
        )
        .await?;

    Ok(())
}

// This function checks if the reward exists by the reward address in the database
async fn check_if_reward_exist_by_reward_address(
    reward_address: String,
    db_client: &mut tokio_postgres::Client,
) -> Result<bool, anyhow::Error> {
    let executable = format!(
        "
            SELECT 1
            FROM {REWARD_TABLE_NAME}{DB_VERSION}
            WHERE reward_address = $1
        "
    );

    let result = db_client.query(&executable, &[&reward_address]).await?;

    Ok(!result.is_empty())
}

// This function checks if the reward exists in the database
pub async fn check_if_reward_exist_by_brand_id(
    brand_id: String,
    db_client: &mut tokio_postgres::Client,
) -> Result<bool, anyhow::Error> {
    let executable = format!(
        "
            SELECT 1
            FROM {REWARD_TABLE_NAME}{DB_VERSION}
            WHERE brand_id = $1
        "
    );

    let result = db_client.query(&executable, &[&brand_id]).await?;

    Ok(!result.is_empty())
}

// this function returns all the count of reward entities in the database
pub async fn get_total_rewards_count(
    db_client: &mut tokio_postgres::Client,
) -> Result<i64, anyhow::Error> {
    let executable = format!(
        "
            SELECT COUNT(*) as count
            FROM {REWARD_TABLE_NAME}{DB_VERSION}
        "
    );

    let result = db_client.query(&executable, &[]).await?;

    Ok(result[0].get(0))
}

pub async fn query_reward_by_brand_id(
    brand_id: String,
    db_client: &mut tokio_postgres::Client,
) -> Result<RewardRecord, anyhow::Error> {
    let executable = format!(
        "
            SELECT *
            FROM {REWARD_TABLE_NAME}{DB_VERSION}
            WHERE brand_id = $1
        "
    );

    let result = db_client.query(&executable, &[&brand_id]).await?;

    let reward_record = RewardRecord {
        brand_id: result[0].get(1),
        reward_address: result[0].get(2),
        requestor_address: result[0].get(3),
        initial_supply: result[0].get(4),
        timestamp: result[0].get(5),
        create_at: result[0].get(6),
    };

    Ok(reward_record)
}

pub async fn query_reward_by_reward_address(
    reward_address: String,
    db_client: &mut tokio_postgres::Client,
) -> Result<RewardRecord, anyhow::Error> {
    let executable = format!(
        "
            SELECT *
            FROM {REWARD_TABLE_NAME}{DB_VERSION}
            WHERE reward_address = $1
        "
    );

    let result = db_client.query(&executable, &[&reward_address]).await?;

    let reward_record = RewardRecord {
        brand_id: result[0].get(1),
        reward_address: result[0].get(2),
        requestor_address: result[0].get(3),
        initial_supply: result[0].get(4),
        timestamp: result[0].get(5),
        create_at: result[0].get(6),
    };

    Ok(reward_record)
}

pub async fn query_reward_by_requestor_address(
    requestor_address: String,
    db_client: &mut tokio_postgres::Client,
) -> Result<RewardRecord, anyhow::Error> {
    let executable = format!(
        "
            SELECT *
            FROM {REWARD_TABLE_NAME}{DB_VERSION}
            WHERE requestor_address = $1
        "
    );

    let result = db_client.query(&executable, &[&requestor_address]).await?;

    let reward_record = RewardRecord {
        brand_id: result[0].get(1),
        reward_address: result[0].get(2),
        requestor_address: result[0].get(3),
        initial_supply: result[0].get(4),
        timestamp: result[0].get(5),
        create_at: result[0].get(6),
    };

    Ok(reward_record)
}

pub async fn query_all_rewards_paginated(
    db_client: &mut tokio_postgres::Client,
    page_number: i64,
    page_size: i64,
) -> Result<Vec<RewardRecord>, anyhow::Error> {
    let executable = format!(
        "
            SELECT * 
            FROM {REWARD_TABLE_NAME}{DB_VERSION}
            ORDER BY id DESC
            LIMIT $1 OFFSET $2;
        "
    );

    let offest = (page_number - 1) * page_size;

    let result = db_client
        .query(&executable, &[&page_size, &offest])
        .await?;
    let mut reward_record_buffer = Vec::new();

    for row in result {
        let reward_record = RewardRecord {
            brand_id: row.get(1),
            reward_address: row.get(2),
            requestor_address: row.get(3),
            initial_supply: row.get(4),
            timestamp: row.get(5),
            create_at: row.get(6),
        };
        reward_record_buffer.push(reward_record);
    }

    Ok(reward_record_buffer)
}

pub async fn query_all_rewards_owned_by_a_brand(
    brand_id: String,
    db_client: &mut tokio_postgres::Client,
) -> Result<Vec<RewardRecord>, anyhow::Error> {
    let executable = format!(
        "
            SELECT * 
            FROM {REWARD_TABLE_NAME}{DB_VERSION}
            WHERE brand_id = $1
        "
    );

    let result = db_client.query(&executable, &[&brand_id]).await?;
    let mut reward_record_buffer = Vec::new();

    for row in result {
        let reward_record = RewardRecord {
            brand_id: row.get(1),
            reward_address: row.get(2),
            requestor_address: row.get(3),
            initial_supply: row.get(4),
            timestamp: row.get(5),
            create_at: row.get(6),
        };
        reward_record_buffer.push(reward_record);
    }

    Ok(reward_record_buffer)
}
