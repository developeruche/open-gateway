use super::DB_VERSION;
use crate::entity_record::BrandRecord;

pub const BRAND_TABLE_NAME: &str = "brand";

/// This function is used to create a new brand table in the database
/// if one doesn not exist already
pub async fn create_brand_table(
    db_client: &mut tokio_postgres::Client,
) -> Result<(), anyhow::Error> {
    let executable = format!(
        "
            CREATE TABLE IF NOT EXISTS {BRAND_TABLE_NAME}{DB_VERSION} (
                id              SERIAL PRIMARY KEY,
                name         VARCHAR NULL,
                main_account    VARCHAR NULL,
                online_presence VARCHAR NULL,
                brand_protocol_id          VARCHAR NULL,
                onboarding_manager          VARCHAR NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
        "
    );
    db_client.batch_execute(&executable).await?;
    Ok(())
}

/// This function create a new brand enity in the database
pub async fn create_brand_if_does_not_exist(
    name: String,
    main_account: String,
    online_presence: String,
    brand_protocol_id: String,
    onboarding_manager: String,
    db_client: &mut tokio_postgres::Client,
) -> Result<(), anyhow::Error> {
    let executable = format!(
        "
            INSERT INTO {BRAND_TABLE_NAME}{DB_VERSION} (name, main_account, online_presence, brand_protocol_id, onboarding_manager)
            VALUES ($1, $2, $3, $4, $5)
        "
    );

    if check_brand_exists_by_brand_id(brand_protocol_id.clone(), db_client).await? {
        return Ok(());
    }

    db_client
        .execute(
            &executable,
            &[
                &name,
                &main_account,
                &online_presence,
                &brand_protocol_id,
                &onboarding_manager,
            ],
        )
        .await?;

    Ok(())
}

// This function returns all the count of brand entities in the database
pub async fn get_total_count_brands(
    db_client: &mut tokio_postgres::Client,
) -> Result<i64, anyhow::Error> {
    let executable = format!(
        "
            SELECT COUNT(*)
            FROM {BRAND_TABLE_NAME}{DB_VERSION}
        "
    );

    let result = db_client.query(&executable, &[]).await?;

    Ok(result[0].get(0))
}

async fn check_brand_exists_by_brand_id(
    brand_protocol_id: String,
    db_client: &mut tokio_postgres::Client,
) -> Result<bool, anyhow::Error> {
    let executable = format!(
        "
            SELECT 1
            FROM {BRAND_TABLE_NAME}{DB_VERSION}
            WHERE brand_protocol_id = $1
        "
    );

    let result = db_client.query(&executable, &[&brand_protocol_id]).await?;

    Ok(!result.is_empty())
}

pub async fn check_brand_exists_by_main_account(
    main_account: String,
    db_client: &mut tokio_postgres::Client,
) -> Result<bool, anyhow::Error> {
    let executable = format!(
        "
            SELECT 1
            FROM {BRAND_TABLE_NAME}{DB_VERSION}
            WHERE main_account = $1
        "
    );

    let result = db_client.query(&executable, &[&main_account]).await?;

    Ok(!result.is_empty())
}

pub async fn query_brand_by_id(
    brand_id: String,
    db_client: &mut tokio_postgres::Client,
) -> Result<BrandRecord, anyhow::Error> {
    let executable = format!(
        "
            SELECT * 
            FROM {BRAND_TABLE_NAME}{DB_VERSION} 
            WHERE brand_protocol_id = $1;
        "
    );

    let result = db_client.query_one(&executable, &[&brand_id]).await?;

    let brand_record = BrandRecord {
        brand_name: result.get(1),
        main_account: result.get(2),
        online_presence: result.get(3),
        brand_protocol_id: result.get(4),
        onboarding_manager: result.get(5),
        create_at: result.get(6),
    };

    Ok(brand_record)
}

pub async fn query_brand_by_main_account(
    main_account: String,
    db_client: &mut tokio_postgres::Client,
) -> Result<BrandRecord, anyhow::Error> {
    let executable = format!(
        "
            SELECT * 
            FROM {BRAND_TABLE_NAME}{DB_VERSION} 
            WHERE main_account = $1;
        "
    );

    let result = db_client.query_one(&executable, &[&main_account]).await?;

    let brand_record = BrandRecord {
        brand_name: result.get(1),
        main_account: result.get(2),
        online_presence: result.get(3),
        brand_protocol_id: result.get(4),
        onboarding_manager: result.get(5),
        create_at: result.get(6),
    };

    Ok(brand_record)
}

pub async fn query_brand_by_name(
    brand_name: String,
    db_client: &mut tokio_postgres::Client,
) -> Result<BrandRecord, anyhow::Error> {
    let executable = format!(
        "
            SELECT *
            FROM {BRAND_TABLE_NAME}{DB_VERSION} 
            WHERE name = $1;
        "
    );

    let result = db_client.query_one(&executable, &[&brand_name]).await?;

    let brand_record = BrandRecord {
        brand_name: result.get(1),
        main_account: result.get(2),
        online_presence: result.get(3),
        brand_protocol_id: result.get(4),
        onboarding_manager: result.get(5),
        create_at: result.get(6),
    };

    Ok(brand_record)
}

pub async fn query_all_brands_paginated(
    db_client: &mut tokio_postgres::Client,
    page_number: i64,
    page_size: i64,
) -> Result<Vec<BrandRecord>, anyhow::Error> {
    let executable = format!(
        "
            SELECT * 
            FROM {BRAND_TABLE_NAME}{DB_VERSION}
            ORDER BY id DESC
            LIMIT $1 OFFSET $2;
        "
    );

    let offest = (page_number - 1) * page_size;

    let result = db_client.query(&executable, &[&page_size, &offest]).await?;
    let mut brand_record_buffer = Vec::new();

    for row in result {
        let brand_record = BrandRecord {
            brand_name: row.get(1),
            main_account: row.get(2),
            online_presence: row.get(3),
            brand_protocol_id: row.get(4),
            onboarding_manager: row.get(5),
            create_at: row.get(6),
        };
        brand_record_buffer.push(brand_record);
    }

    Ok(brand_record_buffer)
}
