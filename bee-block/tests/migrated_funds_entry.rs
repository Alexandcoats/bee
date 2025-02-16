// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use bee_block::{
    address::{Address, Ed25519Address},
    constant::TOKEN_SUPPLY,
    payload::milestone::option::MigratedFundsEntry,
    rand::receipt::rand_tail_transaction_hash,
    Error,
};
use packable::{bounded::InvalidBoundedU64, error::UnpackError, PackableExt};

const ED25519_ADDRESS: &str = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649";

#[test]
fn new_valid() {
    let tth = rand_tail_transaction_hash();
    let address = Address::from(Ed25519Address::from_str(ED25519_ADDRESS).unwrap());
    let amount = 42424242;
    let mfe = MigratedFundsEntry::new(tth.clone(), address, amount).unwrap();

    assert_eq!(mfe.tail_transaction_hash(), &tth);
    assert_eq!(*mfe.address(), address);
    assert_eq!(mfe.amount(), amount);
}

#[test]
fn new_invalid_amount() {
    assert!(matches!(
        MigratedFundsEntry::new(
            rand_tail_transaction_hash(),
            Address::from(Ed25519Address::from_str(ED25519_ADDRESS).unwrap()),
            TOKEN_SUPPLY + 1
        ),
        Err(Error::InvalidMigratedFundsEntryAmount(InvalidBoundedU64(n))) if n == TOKEN_SUPPLY + 1
    ));
}

#[test]
fn packed_len() {
    let mge = MigratedFundsEntry::new(
        rand_tail_transaction_hash(),
        Address::from(Ed25519Address::from_str(ED25519_ADDRESS).unwrap()),
        42424242,
    )
    .unwrap();

    assert_eq!(mge.packed_len(), 49 + 1 + 32 + 8);
    assert_eq!(mge.pack_to_vec().len(), 49 + 1 + 32 + 8);
}

#[test]
fn pack_unpack_valid() {
    let address = Address::from(Ed25519Address::from_str(ED25519_ADDRESS).unwrap());
    let amount = 42424242;
    let mfe_1 = MigratedFundsEntry::new(rand_tail_transaction_hash(), address, amount).unwrap();
    let mfe_2 = MigratedFundsEntry::unpack_verified(&mut mfe_1.pack_to_vec().as_slice(), &()).unwrap();

    assert_eq!(mfe_1.tail_transaction_hash(), mfe_2.tail_transaction_hash());
    assert_eq!(*mfe_1.address(), *mfe_2.address());
    assert_eq!(mfe_1.amount(), mfe_2.amount());
}

#[test]
fn pack_unpack_invalid_amount() {
    let bytes = vec![
        42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
        42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 0, 82, 253, 252, 7, 33,
        130, 101, 79, 22, 63, 95, 15, 154, 98, 29, 114, 149, 102, 199, 77, 16, 3, 124, 77, 123, 187, 4, 7, 209, 226,
        198, 73, 42, 42, 42, 42, 42, 42, 42, 42,
    ];

    assert!(matches!(
        MigratedFundsEntry::unpack_verified(&mut bytes.as_slice(), &()),
        Err(UnpackError::Packable(Error::InvalidMigratedFundsEntryAmount(
            InvalidBoundedU64(3_038_287_259_199_220_266)
        )))
    ));
}
