use codec::Encode;
use subxt::{tx::PairSigner, OnlineClient, SubstrateConfig};
use polkadex_primitives::snapshot::EnclaveSnapshot;
use polkadex_primitives::{AccountId, AssetsLimit, SnapshotAccLimit, WithdrawalLimit};
use sp_core::{H256, Pair, sr25519};
use sp_runtime::{BoundedVec,BoundedBTreeMap};

#[subxt::subxt(runtime_metadata_path = "src/metadata.scale")]
pub mod polkadex {}

#[tokio::main]
async fn main() {
    let (pair, _) = sr25519::Pair::generate();
    let api = OnlineClient::<SubstrateConfig>::new().await.unwrap();
    let snapshot: EnclaveSnapshot<AccountId,WithdrawalLimit,AssetsLimit,SnapshotAccLimit> = EnclaveSnapshot {
        snapshot_number: 10,
        merkle_root: H256::random(),
        withdrawals: BoundedBTreeMap::default(),
        fees: BoundedVec::default(),
    };

    let signature = pair.sign(&snapshot.encode());
    let txn = polkadex::tx().ocex().submit_snapshot(snapshot, signature);

    let result = api
        .tx()
        .sign_and_submit_then_watch_default(&txn, & PairSigner::new(pair))
        .await.unwrap()
        .wait_for_finalized_success()
        .await.unwrap();

}