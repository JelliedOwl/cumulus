#!/bin/bash

# import common functions
source "$(dirname "$0")"/bridges_rococo_wococo.sh "import"

# Address: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
# AccountId: [212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125]
ASSET_HUB_KUSAMA_ACCOUNT_SEED_FOR_LOCAL="//Alice"
# Address: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
# AccountId: [212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125]
ASSET_HUB_POLKADOT_ACCOUNT_ADDRESS_FOR_LOCAL="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"

# Expected sovereign accounts.
#
# Generated by:
#
#        #[test]
#        fn generate_sovereign_accounts() {
#		use sp_core::crypto::Ss58Codec;
#
#		parameter_types! {
#			pub UniversalLocationAHK: InteriorMultiLocation = X2(GlobalConsensus(Kusama), Parachain(1000));
#			pub UniversalLocationAHP: InteriorMultiLocation = X2(GlobalConsensus(Polkadot), Parachain(1000));
#		}
#
#		// SS58=0
#		println!("GLOBAL_CONSENSUS_KUSAMA_SOVEREIGN_ACCOUNT=\"{}\"",
#				 frame_support::sp_runtime::AccountId32::new(
#					 GlobalConsensusConvertsFor::<UniversalLocationAHP, [u8; 32]>::convert_location(
#						 &MultiLocation { parents: 2, interior: X1(GlobalConsensus(Kusama)) }).unwrap()
#				 ).to_ss58check_with_version(0_u16.into())
#		);
#		println!("GLOBAL_CONSENSUS_KUSAMA_ASSET_HUB_KUSAMA_1000_SOVEREIGN_ACCOUNT=\"{}\"",
#				 frame_support::sp_runtime::AccountId32::new(
#					 GlobalConsensusParachainConvertsFor::<UniversalLocationAHP, [u8; 32]>::convert_location(
#						 &MultiLocation { parents: 2, interior: X2(GlobalConsensus(Kusama), Parachain(1000)) }).unwrap()
#				 ).to_ss58check_with_version(0_u16.into())
#		);
#
#		// SS58=2
#		println!("GLOBAL_CONSENSUS_POLKADOT_SOVEREIGN_ACCOUNT=\"{}\"",
#				 frame_support::sp_runtime::AccountId32::new(
#					 GlobalConsensusConvertsFor::<UniversalLocationAHK, [u8; 32]>::convert_location(
#						 &MultiLocation { parents: 2, interior: X1(GlobalConsensus(Polkadot)) }).unwrap()
#				 ).to_ss58check_with_version(2_u16.into())
#		);
#		println!("GLOBAL_CONSENSUS_POLKADOT_ASSET_HUB_POLKADOT_1000_SOVEREIGN_ACCOUNT=\"{}\"",
#				 frame_support::sp_runtime::AccountId32::new(
#					 GlobalConsensusParachainConvertsFor::<UniversalLocationAHK, [u8; 32]>::convert_location(
#						 &MultiLocation { parents: 2, interior: X2(GlobalConsensus(Polkadot), Parachain(1000)) }).unwrap()
#				 ).to_ss58check_with_version(2_u16.into())
#		);
#	}
GLOBAL_CONSENSUS_KUSAMA_SOVEREIGN_ACCOUNT="14zcUAhP5XypiFQWA3b1AnGKrhZqR4XWUo4deWkwuN5y983G"
GLOBAL_CONSENSUS_KUSAMA_ASSET_HUB_KUSAMA_1000_SOVEREIGN_ACCOUNT="12GvRkNCmXFuaaziTJ2ZKAfa7MArKfLT2HYvLjQuepP3JuHf"
GLOBAL_CONSENSUS_POLKADOT_SOVEREIGN_ACCOUNT="FxqimVubBRPqJ8kTwb3wL7G4q645hEkBEnXPyttLsTrFc5Q"
GLOBAL_CONSENSUS_POLKADOT_ASSET_HUB_POLKADOT_1000_SOVEREIGN_ACCOUNT="FwGjEp7GXJXT9NjH8r4sqdyd8XZVogbxSs3iFakx4wFwJ5Y"

function init_ksm_dot() {
    ensure_relayer

    RUST_LOG=runtime=trace,rpc=trace,bridge=trace \
        ~/local_bridge_testing/bin/substrate-relay init-bridge kusama-to-bridge-hub-polkadot \
	--source-host localhost \
	--source-port 9942 \
	--source-version-mode Auto \
	--target-host localhost \
	--target-port 8945 \
	--target-version-mode Auto \
	--target-signer //Bob
}

function init_dot_ksm() {
    ensure_relayer

    RUST_LOG=runtime=trace,rpc=trace,bridge=trace \
        ~/local_bridge_testing/bin/substrate-relay init-bridge polkadot-to-bridge-hub-kusama \
        --source-host localhost \
        --source-port 9945 \
        --source-version-mode Auto \
        --target-host localhost \
        --target-port 8943 \
        --target-version-mode Auto \
        --target-signer //Bob
}

function run_relay() {
    ensure_relayer

RUST_LOG=runtime=trace,rpc=trace,bridge=trace \
        ~/local_bridge_testing/bin/substrate-relay relay-headers-and-messages bridge-hub-kusama-bridge-hub-polkadot \
        --kusama-host localhost \
        --kusama-port 9942 \
        --kusama-version-mode Auto \
        --bridge-hub-kusama-host localhost \
        --bridge-hub-kusama-port 8943 \
        --bridge-hub-kusama-version-mode Auto \
        --bridge-hub-kusama-signer //Charlie \
        --polkadot-headers-to-bridge-hub-kusama-signer //Bob \
        --polkadot-parachains-to-bridge-hub-kusama-signer //Bob \
        --bridge-hub-kusama-transactions-mortality 4 \
        --polkadot-host localhost \
        --polkadot-port 9945 \
        --polkadot-version-mode Auto \
        --bridge-hub-polkadot-host localhost \
        --bridge-hub-polkadot-port 8945 \
        --bridge-hub-polkadot-version-mode Auto \
        --bridge-hub-polkadot-signer //Charlie \
        --kusama-headers-to-bridge-hub-polkadot-signer //Bob \
        --kusama-parachains-to-bridge-hub-polkadot-signer //Bob \
        --bridge-hub-polkadot-transactions-mortality 4 \
        --lane 00000000
}

case "$1" in
  run-relay)
    init_ksm_dot
    init_dot_ksm
    run_relay
    ;;
  init-asset-hub-kusama-local)
      # create foreign assets for native Polkadot token (governance call on Kusama)
      force_create_foreign_asset \
          "ws://127.0.0.1:9942" \
          "//Alice" \
          1000 \
          "ws://127.0.0.1:9910" \
          "$(jq --null-input '{ "parents": 2, "interior": { "X1": { "GlobalConsensus": "Polkadot" } } }')" \
          "$GLOBAL_CONSENSUS_POLKADOT_SOVEREIGN_ACCOUNT" \
          1000000000 \
          true
      # drip SA which holds reserves
      transfer_balance \
          "ws://127.0.0.1:9910" \
          "//Alice" \
          "$GLOBAL_CONSENSUS_POLKADOT_ASSET_HUB_POLKADOT_1000_SOVEREIGN_ACCOUNT" \
          $((1000000000 + 50000000000 * 20))
      ;;
  init-asset-hub-polkadot-local)
      # create foreign assets for native Polkadot token (governance call on Kusama)
            force_create_foreign_asset \
                "ws://127.0.0.1:9945" \
                "//Alice" \
                1000 \
                "ws://127.0.0.1:9010" \
                "$(jq --null-input '{ "parents": 2, "interior": { "X1": { "GlobalConsensus": "Kusama" } } }')" \
                "$GLOBAL_CONSENSUS_KUSAMA_SOVEREIGN_ACCOUNT" \
                1000000000 \
                true
      # drip SA which holds reserves
      transfer_balance \
          "ws://127.0.0.1:9010" \
          "//Alice" \
          "$GLOBAL_CONSENSUS_KUSAMA_ASSET_HUB_KUSAMA_1000_SOVEREIGN_ACCOUNT" \
          $((1000000000 + 50000000000 * 20))
      ;;
  reserve-transfer-assets-from-asset-hub-kusama-local)
      ensure_polkadot_js_api
      # send KSMs to Alice account on AHP
      limited_reserve_transfer_assets \
          "ws://127.0.0.1:9910" \
          "//Alice" \
          "$(jq --null-input '{ "V3": { "parents": 2, "interior": { "X2": [ { "GlobalConsensus": "Polkadot" }, { "Parachain": 1000 } ] } } }')" \
          "$(jq --null-input '{ "V3": { "parents": 0, "interior": { "X1": { "AccountId32": { "id": [212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125] } } } } }')" \
          "$(jq --null-input '{ "V3": [ { "id": { "Concrete": { "parents": 1, "interior": "Here" } }, "fun": { "Fungible": 1000000000000 } } ] }')" \
          0 \
          "Unlimited"
      ;;
  reserve-transfer-assets-from-asset-hub-polkadot-local)
      ensure_polkadot_js_api
      # send DOTs to Alice account on AHH
      limited_reserve_transfer_assets \
          "ws://127.0.0.1:9010" \
          "//Alice" \
          "$(jq --null-input '{ "V3": { "parents": 2, "interior": { "X2": [ { "GlobalConsensus": "Kusama" }, { "Parachain": 1000 } ] } } }')" \
          "$(jq --null-input '{ "V3": { "parents": 0, "interior": { "X1": { "AccountId32": { "id": [212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125] } } } } }')" \
          "$(jq --null-input '{ "V3": [ { "id": { "Concrete": { "parents": 1, "interior": "Here" } }, "fun": { "Fungible": 2000000000000 } } ] }')" \
          0 \
          "Unlimited"
      ;;
  stop)
    pkill -f polkadot
    pkill -f parachain
    ;;
  *)
    echo "A command is require. Supported commands for:
    Local (zombienet) run:
          - run-relay
          - init-asset-hub-kusama-local
          - init-asset-hub-polkadot-local
          - reserve-transfer-assets-from-asset-hub-kusama-local
          - reserve-transfer-assets-from-asset-hub-polkadot-local";
    exit 1
    ;;
esac
