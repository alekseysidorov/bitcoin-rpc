#[macro_use]
extern crate jsonrpc_v1;
extern crate strason;
extern crate serde;

use jsonrpc_v1::client::Client as RpcClient;
use jsonrpc_v1::Error as RpcError;
use strason::Json;

macro_rules! rpc_method {
    ($method_name:ident<$return_type:ty>, $rpc_name:expr) => {
        pub fn $method_name(&self) -> Result<$return_type, RpcError> {
            let request = self.client.build_request(String::from($rpc_name), vec![]);

            match self.client.send_request(&request).and_then(|res| res.into_result::<$return_type>()) {
                Ok(res) => return Ok(res),
                Err(e) => return Err(e),
            }
        }
    };
    ($method_name:ident<$return_type:ty>, $rpc_name:expr, { $($param:ident : $param_ty:ty),* }) => {
        pub fn $method_name(&self, $($param : $param_ty),*) -> Result<$return_type, RpcError> {
            let mut params: Vec<Json> = Vec::new();

            $(
                params.push(Json::from($param));
            )*

            let request = self.client.build_request(String::from($rpc_name), params);

            match self.client.send_request(&request).and_then(|res| res.into_result::<$return_type>()) {
                Ok(res) => return Ok(res),
                Err(e) => return Err(e),
            }
        }
    }
}

/// A Handle to a Bitcoin Rpc connection
pub struct BitcoinRpc {
    client: RpcClient,
}

pub struct SerializedBlock {
    pub result: String,
}

pub struct Block {
    pub hash: String,
    pub confirmations: i64,
    pub size: i64,
    pub height: i64,
    pub version: i64,
    pub merkleroot: String,
    pub tx: Vec<Json>,
    pub txid: String,
    pub time: i64,
    pub nonce: i64,
    pub bits: String,
    pub chainwork: String,
    pub previousblockhash: Option<String>,
    pub nextblockhash: Option<String>,
}

pub enum GetBlockReply {
    True(Block),
    False(SerializedBlock),
}

serde_struct_enum_impl!(GetBlockReply,
                        True, Block, hash, confirmations, size, height, version, merkleroot, tx, txid <- "TXID", time, nonce, bits,  chainwork, previousblockhash, nextblockhash;
                        False, SerializedBlock, result
);

pub struct BlockChainInfo {
    chain: String,
    blocks: u64,
    headers: u64,
    bestblockhash: String,
    difficulty: f64,
    verificationprogress: f64,
    chainwork: String,
}

serde_struct_impl!(BlockChainInfo, chain, blocks, headers, bestblockhash, difficulty, verificationprogress, chainwork);

impl BitcoinRpc {
    /// Creates a connection to a bitcoin rpc server
    pub fn new(url: &str, user: Option<String>, pass: Option<String>) -> Self {
        // Check that if we have a password, we have a username; other way around is ok
        debug_assert!(pass.is_none() || user.is_some());

        BitcoinRpc {
            client: RpcClient::new(String::from(url), user, pass),
        }
    }

    rpc_method!(getbestblockhash<String>, "getbestblockhash");

    rpc_method!(getblock<GetBlockReply>, "getblock", {
        header_hash: String,
        format: bool
    });

    rpc_method!(getblockchaininfo<BlockChainInfo>, "getblockchaininfo");
}

