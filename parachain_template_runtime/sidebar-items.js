window.SIDEBAR_ITEMS = {"constant":[["DAYS",""],["EXISTENTIAL_DEPOSIT","The existential deposit. Set to 1/10 of the Connected Relay Chain."],["HOURS",""],["MICROUNIT",""],["MILLISECS_PER_BLOCK","This determines the average expected block time that we are targeting. Blocks will be produced at a minimum duration defined by `SLOT_DURATION`. `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked up by `pallet_aura` to implement `fn slot_duration()`."],["MILLIUNIT",""],["MINUTES",""],["SLOT_DURATION",""],["UNIT",""],["VERSION",""],["WASM_BINARY",""],["WASM_BINARY_BLOATY",""]],"enum":[["MultiAddress","A multi-format address wrapper for on-chain accounts."],["OriginCaller",""],["RuntimeCall",""],["RuntimeEvent",""],["RuntimeFreezeReason",""],["RuntimeHoldReason",""],["RuntimeLockId",""],["RuntimeSlashReason",""]],"fn":[["native_version","The version information used to identify this runtime when compiled natively."]],"mod":[["api",""],["opaque","Opaque types. These are used by the CLI to instantiate machinery that don’t need to know the specifics of the runtime. They can then be made to be agnostic over specific formats of data like extrinsics, allowing for them to continue syncing the network through upgrades to even the core data structures."],["xcm_config",""]],"struct":[["ExistentialDeposit",""],["GenesisConfig",""],["MaxCandidates",""],["MaxInvulnerables",""],["MinCandidates",""],["Offset",""],["PalletInfo","Provides an implementation of `PalletInfo` to provide information about the pallet setup in the runtime."],["Perbill","A fixed point representation of a number in the range [0, 1]."],["Period",""],["Permill","A fixed point representation of a number in the range [0, 1]."],["PotId",""],["ReservedDmpWeight",""],["ReservedXcmpWeight",""],["Runtime",""],["RuntimeApi",""],["RuntimeApiImpl","Implements all runtime apis for the client side."],["RuntimeBlockLength",""],["RuntimeBlockWeights",""],["RuntimeOrigin","The runtime origin type representing the origin of a call."],["SS58Prefix",""],["SessionKeys",""],["SessionLength",""],["StakingAdminBodyId",""],["TransactionByteFee","Relay Chain `TransactionByteFee` / 10"],["Version",""],["WeightToFee","Handles converting a weight scalar to a fee value, based on the scale and granularity of the node’s balance type."]],"trait":[["BuildStorage","Complex storage builder stuff."]],"type":[["AccountId","Some way of identifying an account on the chain. We intentionally make it equivalent to the public key of our transaction signing scheme."],["Address","The address format for describing accounts."],["AllPallets","All pallets included in the runtime as a nested tuple of types."],["AllPalletsReversedWithSystemFirst","All pallets included in the runtime as a nested tuple of types in reversed order. With the system pallet first."],["AllPalletsWithSystem","All pallets included in the runtime as a nested tuple of types."],["AllPalletsWithSystemReversed","All pallets included in the runtime as a nested tuple of types in reversed order."],["AllPalletsWithoutSystem","All pallets included in the runtime as a nested tuple of types. Excludes the System pallet."],["AllPalletsWithoutSystemReversed","All pallets included in the runtime as a nested tuple of types in reversed order. Excludes the System pallet."],["Aura",""],["AuraConfig",""],["AuraExt",""],["AuraExtConfig",""],["AuraId","An Aura authority identifier using S/R 25519 as its crypto."],["Authorship",""],["Balance","Balance of an account."],["Balances",""],["BalancesConfig",""],["Block","Block type as expected by this runtime."],["BlockId","BlockId type as expected by this runtime."],["BlockNumber","An index to a block."],["CheckedExtrinsic","Extrinsic type that has already been checked."],["CollatorSelection",""],["CollatorSelectionConfig",""],["CollatorSelectionUpdateOrigin","We allow root and the StakingAdmin to execute privileged collator selection operations."],["CumulusXcm",""],["DmpQueue",""],["Executive","Executive: handles dispatch to the various modules."],["Hash","A hash of some data used by the chain."],["Header","Block header type as expected by this runtime."],["Index","Index of a transaction in the chain."],["ParachainInfo",""],["ParachainInfoConfig",""],["ParachainSystem",""],["ParachainSystemConfig",""],["PolkadotXcm",""],["PolkadotXcmConfig",""],["Session",""],["SessionConfig",""],["Signature","Alias to 512-bit hash when used in the context of a transaction signature on the chain."],["SignedBlock","A Block signed with a Justification"],["SignedExtra","The SignedExtension to the basic transaction logic."],["System",""],["SystemConfig",""],["TemplatePallet",""],["Timestamp",""],["TransactionPayment",""],["TransactionPaymentConfig",""],["UncheckedExtrinsic","Unchecked extrinsic type as expected by this runtime."],["XcmpQueue",""]]};