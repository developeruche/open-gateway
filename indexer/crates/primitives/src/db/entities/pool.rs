use alloy::primitives::U256;

use super::DB_VERSION;
use crate::{db::PRECISION, entity_record::PoolRecord};

pub const POOL_TABLE_NAME: &str = "pool";

/// This function is literally used to create a nnew pool table in the
/// database if anyone does not exist already
pub async fn create_pool_table(
    db_client: &mut tokio_postgres::Client,
) -> Result<(), anyhow::Error> {
    let executable = format!(
        "
            CREATE TABLE IF NOT EXISTS {POOL_TABLE_NAME}{DB_VERSION} (
               id              SERIAL PRIMARY KEY,
               pool_address    VARCHAR NULL,
               reward_token  VARCHAR UNIQUE,
               me_token        VARCHAR NULL,
               current_amount_of_reward_tokens    VARCHAR NULL,
               current_amount_of_me_tokens        VARCHAR NULL,
               r_optimal        VARCHAR NULL,
               created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
        "
    );

    db_client.batch_execute(&executable).await?;
    Ok(())
}

/// This function creates a new pool entity in the database
pub async fn create_pool_if_does_not_exist(
    pool_address: String,
    reward_address: String,
    me_address: String,
    current_amount_of_reward_tokens: String,
    current_amount_of_me_tokens: String,
    r_optimal: String,
    db_client: &mut tokio_postgres::Client,
) -> Result<(), anyhow::Error> {
    let executable = format!(
        "
            INSERT INTO {POOL_TABLE_NAME}{DB_VERSION} (
                pool_address,
                reward_token,
                me_token,
                current_amount_of_reward_tokens,
                current_amount_of_me_tokens,
                r_optimal
            ) VALUES ($1, $2, $3, $4, $5, $6)
        "
    );

    db_client
        .execute(
            &executable,
            &[
                &pool_address,
                &reward_address,
                &me_address,
                &current_amount_of_reward_tokens,
                &current_amount_of_me_tokens,
                &r_optimal,
            ],
        )
        .await?;

    Ok(())
}

/// This function is used to get all the count of pools in the database
pub async fn get_pool_total_count(
    db_client: &mut tokio_postgres::Client,
) -> Result<i64, anyhow::Error> {
    let executable = format!(
        "
            SELECT COUNT(*)
            FROM {POOL_TABLE_NAME}{DB_VERSION}
        "
    );

    let result = db_client.query(&executable, &[]).await?;

    Ok(result[0].get(0))
}

pub async fn check_pool_exist_by_pool_address(
    pool_address: String,
    db_client: &mut tokio_postgres::Client,
) -> Result<bool, anyhow::Error> {
    let executable = format!(
        "
            SELECT 1
            FROM {POOL_TABLE_NAME}{DB_VERSION}
            WHERE pool_address = $1
        "
    );

    let result = db_client.query(&executable, &[&pool_address]).await?;

    Ok(!result.is_empty())
}

pub async fn check_pool_exist_by_reward_address(
    reward_address: String,
    db_client: &mut tokio_postgres::Client,
) -> Result<bool, anyhow::Error> {
    let executable = format!(
        "
            SELECT 1
            FROM {POOL_TABLE_NAME}{DB_VERSION}
            WHERE reward_token = $1
        "
    );

    let result = db_client.query(&executable, &[&reward_address]).await?;

    Ok(!result.is_empty())
}

pub async fn update_pool_reward_and_me_amount(
    reward_address: String,
    me_token_amount: String,
    reward_amount: String,
    db_client: &mut tokio_postgres::Client,
) -> Result<(), anyhow::Error> {
    let executable = format!(
        "
            INSERT INTO {POOL_TABLE_NAME}{DB_VERSION} (reward_token, current_amount_of_reward_tokens, current_amount_of_me_tokens)
            VALUES ($3, $1, $2)
            ON CONFLICT (reward_token)
            DO UPDATE SET
                current_amount_of_reward_tokens = EXCLUDED.current_amount_of_reward_tokens,
                current_amount_of_me_tokens = EXCLUDED.current_amount_of_me_tokens;
        "
    );

    db_client
        .execute(
            &executable,
            &[&reward_amount, &me_token_amount, &reward_address],
        )
        .await?;

    Ok(())
}

pub async fn query_pool_by_reward_address(
    reward_address: String,
    db_client: &mut tokio_postgres::Client,
) -> Result<PoolRecord, anyhow::Error> {
    let executable = format!(
        "
            SELECT * 
            FROM {POOL_TABLE_NAME}{DB_VERSION} 
            WHERE reward_token = $1;
        "
    );

    let result = db_client.query_one(&executable, &[&reward_address]).await?;

    let pool_record = PoolRecord {
        pool_address: result.get(1),
        reward_token: result.get(2),
        me_token: result.get(3),
        current_amount_of_reward_tokens: result.get(4),
        current_amount_of_me_tokens: result.get(5),
        r_optimal: result.get(6),
        r: ((U256::from_str_radix(result.get(4), 10).unwrap() * U256::from(PRECISION))
            / U256::from_str_radix(result.get(5), 10).unwrap())
        .to_string(),
        create_at: result.get(7),
    };

    Ok(pool_record)
}

pub async fn query_all_pools_paginated(
    db_client: &mut tokio_postgres::Client,
    page_number: i64,
    page_size: i64,
) -> Result<Vec<PoolRecord>, anyhow::Error> {
    let executable = format!(
        "
            SELECT * 
            FROM {POOL_TABLE_NAME}{DB_VERSION}
            ORDER BY id DESC
            LIMIT $1 OFFSET $2;
        "
    );

    let offest = (page_number - 1) * page_size;

    let result = db_client.query(&executable, &[&page_size, &offest]).await?;
    let mut pool_record_buffer = Vec::new();

    for record in result {
        let pool_record = PoolRecord {
            pool_address: record.get(1),
            reward_token: record.get(2),
            me_token: record.get(3),
            current_amount_of_reward_tokens: record.get(4),
            current_amount_of_me_tokens: record.get(5),
            r_optimal: record.get(6),
            r: ((U256::from_str_radix(record.get(4), 10).unwrap() * U256::from(PRECISION))
                / U256::from_str_radix(record.get(5), 10).unwrap())
            .to_string(),
            create_at: record.get(6),
        };

        pool_record_buffer.push(pool_record);
    }

    Ok(pool_record_buffer)
}

// tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::get_db_url_from_env, db::raw_chronicle_event::create_db_instance};

    #[tokio::test]
    #[ignore]
    async fn test_pool_count() {
        let url = get_db_url_from_env();
        let mut db_client = create_db_instance(&url)
            .await
            .expect("Could not create db instance");
        let pool_count = get_pool_total_count(&mut db_client).await.unwrap();

        println!("This is the pool count: {:?}", pool_count);
    }
}
