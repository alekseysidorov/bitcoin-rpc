extern crate bitcoinrpc;

use bitcoinrpc::BitcoinRpc;

#[test]
fn test_blockchain_methods() {
    let client = BitcoinRpc::new("http://localhost:18332", Some(String::from("user")), Some(String::from("pass")));

    client.getbestblockhash().unwrap();
    client.getblockchaininfo().unwrap();
    client.getblockcount().unwrap();
    client.getchaintips().unwrap();
    client.getdifficulty().unwrap();
    client.getmempoolinfo().unwrap();
    client.getrawmempool(false).unwrap();
    client.gettxout(String::from(""), 0, false).unwrap();
    client.gettxoutsetinfo().unwrap();
}
