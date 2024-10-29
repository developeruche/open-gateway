use alloy::primitives::{address, b256, Address, B256};

use crate::{Config, IndexerConfig, ServerConfig};
// =====================================
// EVENT SIGNATURES
// =====================================

/// This event is emmitted when a cross brand redemption is done
/// event-name [`crossBrandRedemption`]
/// event-signature [`rewardMadeConversationWithOtherReward(address,address,uint256,uint256,bool,address)`]
/// event-sinature-hashed [`0xfed787e0d30655f3b4541940cf654e513554b6ceed26d2341cfacd02d1dd882c`]
pub const EVENT_ONE_SIGNATURE: B256 =
    b256!("fed787e0d30655f3b4541940cf654e513554b6ceed26d2341cfacd02d1dd882c");

/// This is for the current state of the pools
/// this event is emitted when a pool interaction is
/// done
/// event-name [`currentPoolsState`]
/// event-signature [`currentPoolsState(address,uint256,uint256,uint256,uint256,uint256,address,uint256,uint256,uint256,uint256,uint256)`]
/// event-sinature-hashed [`0x0f763139e3cee82c571ca295b0b83227dcb2b398bdbcd7b2f002b21d3fd64d078`]
pub const EVENT_TWO_SIGNATURE: B256 =
    b256!("efe08965798e655edd8c4067c3e33db2d678750da0c9f84cd2c38d4fa02faf3b");

/// This is for creation of new Brands
/// This event is emitted when a new brand is created in the admin
/// event-name [`registerBrand`]
/// event-signature [`registerBrand(string,string,address,bytes10,address)`]
pub const EVENT_THREE_SIGNATURE: B256 =
    b256!("0f5a0d63eb86b0478f8c56652b268eddd83c313e58058b1a65951958ed19074c");

/// This is for the creation of new rewards
/// This event is emitted when a new reward is created
/// event-name ['fungibleRewardsCreatedSuccessfully']
/// event-signature ['fungibleRewardsCreatedSuccessfully(bytes10,address,address,uint256,uint256)']
pub const EVENT_FOUR_SIGNATURE: B256 =
    b256!("8ae268bd07c1784370b1d77d72548763ada0729264a966a234f895e03ae4c33c");

// =====================================
// CONTRACT ADDRESSES
// =====================================
pub const OPEN_REWARD_DIAMOND: Address = address!("2C123047B23809DbCCDA2d34bB5158D2563221E3");

// =====================================
// OTHER CONSTANTS CONFIGS
// =====================================
pub fn return_me_indexer_config() -> Config {
    Config {
        name: Some("Chronicle".to_string()),
        indexer: vec![
            IndexerConfig {
                event_name: "On Crossbrand Redeption".to_string(),
                state_machine: "EVM".to_string(),
                rpc_url: get_rpc_from_evm_rpc_from_env(),
                address: OPEN_REWARD_DIAMOND.to_string(),
                event_signature: EVENT_ONE_SIGNATURE.to_string(),
                block_number: get_start_block_from_env(),
            },
            IndexerConfig {
                event_name: "On Current Pool State".to_string(),
                state_machine: "EVM".to_string(),
                rpc_url: get_rpc_from_evm_rpc_from_env(),
                address: OPEN_REWARD_DIAMOND.to_string(),
                event_signature: EVENT_TWO_SIGNATURE.to_string(),
                block_number: get_start_block_from_env(),
            },
            IndexerConfig {
                event_name: "Register Brand".to_string(),
                state_machine: "EVM".to_string(),
                rpc_url: get_rpc_from_evm_rpc_from_env(),
                address: OPEN_REWARD_DIAMOND.to_string(),
                event_signature: EVENT_THREE_SIGNATURE.to_string(),
                block_number: get_start_block_from_env(),
            },
            IndexerConfig {
                event_name: "On Fungible Reward Created Successfully".to_string(),
                state_machine: "EVM".to_string(),
                rpc_url: get_rpc_from_evm_rpc_from_env(),
                address: OPEN_REWARD_DIAMOND.to_string(),
                event_signature: EVENT_FOUR_SIGNATURE.to_string(),
                block_number: get_start_block_from_env(),
            },
        ],
        server: ServerConfig {
            server_url: get_server_url_from_env(),
        },
        db_url: get_db_url_from_env(),
    }
}

fn get_rpc_from_evm_rpc_from_env() -> String {
    std::env::var("JSON_RPC").expect("JSON_RPC must be set")
}

pub fn get_db_url_from_env() -> String {
    std::env::var("DB_URL_PROD").expect("RUNTIME_ADMIN_ADDRESS must be set")
}

fn get_server_url_from_env() -> String {
    std::env::var("HOST_N_PORT")
        .expect("HOST_N_PORT must be set")
        .to_string()
}

fn get_start_block_from_env() -> u64 {
    std::env::var("START_BLOCK")
        .expect("START_BLOCK must be set")
        .parse::<u64>()
        .expect("START_BLOCK must be a number")
}
