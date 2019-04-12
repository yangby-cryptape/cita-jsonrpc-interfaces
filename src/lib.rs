// Copyright (C) 2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use jsonrpc_sdk_prelude::{jsonrpc_client, jsonrpc_core, serde_json, JsonRpcRequest};

pub use jsonrpc_types::rpc_types::{
    Block, BlockNumber, Boolean, CallRequest, Data, Data20, Data32, Filter, FilterChanges, Id, Log,
    MetaData, OneItemTupleTrick, PeersInfo, Quantity, Receipt, RpcTransaction, SoftwareVersion,
    TxResponse, Version,
};

jsonrpc_client!(|| {
    pub trait Cita {
        fn blockNumber() -> Quantity;
        fn peerCount() -> Quantity;
        fn sendRawTransaction(Data) -> TxResponse;
        fn sendTransaction(Data) -> TxResponse;
        fn getBlockByHash(Data32, Boolean) -> Block;
        fn getBlockByNumber(BlockNumber, Boolean) -> Block;
        fn getTransactionReceipt(Data32) -> Receipt;
        fn getLogs(Filter) -> Vec<Log>;
        fn call(CallRequest, BlockNumber) -> Data;
        fn getTransaction(Data32) -> RpcTransaction;
        fn getTransactionCount(Data20, BlockNumber) -> Quantity;
        fn getCode(Data20, BlockNumber) -> Data;
        fn getAbi(Data20, BlockNumber) -> Data;
        fn getBalance(Data20, BlockNumber) -> Quantity;
        fn newFilter(Filter) -> Quantity;
        fn newBlockFilter() -> Quantity;
        fn uninstallFilter(Quantity) -> Boolean;
        fn getFilterChanges(Quantity) -> FilterChanges;
        fn getFilterLogs(Quantity) -> Vec<Log>;
        fn getTransactionProof(Data32) -> Data;
        fn getMetaData(BlockNumber) -> MetaData;
        fn getStateProof(Data20, Data32, BlockNumber) -> Data;
        fn getBlockHeader(BlockNumber) -> Data;
        fn getStorageAt(Data20, Data32, BlockNumber) -> Data;
        fn getVersion() -> SoftwareVersion;
        fn peersInfo() -> PeersInfo;
    }
});
