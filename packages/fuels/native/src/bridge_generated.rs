#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.82.6.

use crate::api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::rust2dart::IntoIntoDart;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

use crate::model::transaction::TransactionCost;

// Section: wire functions

fn wire_new_random__static_method__WalletUnlocked_impl(
    port_: MessagePort,
    provider: impl Wire2Api<Provider> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, WalletUnlocked, _>(
        WrapInfo {
            debug_name: "new_random__static_method__WalletUnlocked",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_provider = provider.wire2api();
            move |task_callback| Result::<_, ()>::Ok(WalletUnlocked::new_random(api_provider))
        },
    )
}
fn wire_new_from_private_key__static_method__WalletUnlocked_impl(
    port_: MessagePort,
    private_key: impl Wire2Api<String> + UnwindSafe,
    provider: impl Wire2Api<Provider> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, WalletUnlocked, _>(
        WrapInfo {
            debug_name: "new_from_private_key__static_method__WalletUnlocked",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_private_key = private_key.wire2api();
            let api_provider = provider.wire2api();
            move |task_callback| {
                Result::<_, ()>::Ok(WalletUnlocked::new_from_private_key(
                    api_private_key,
                    api_provider,
                ))
            }
        },
    )
}
fn wire_new_from_mnemonic_phrase__static_method__WalletUnlocked_impl(
    port_: MessagePort,
    phrase: impl Wire2Api<String> + UnwindSafe,
    provider: impl Wire2Api<Provider> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, WalletUnlocked, _>(
        WrapInfo {
            debug_name: "new_from_mnemonic_phrase__static_method__WalletUnlocked",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_phrase = phrase.wire2api();
            let api_provider = provider.wire2api();
            move |task_callback| {
                Result::<_, ()>::Ok(WalletUnlocked::new_from_mnemonic_phrase(
                    api_phrase,
                    api_provider,
                ))
            }
        },
    )
}
fn wire_new_from_mnemonic_phrase_with_path__static_method__WalletUnlocked_impl(
    port_: MessagePort,
    phrase: impl Wire2Api<String> + UnwindSafe,
    path: impl Wire2Api<String> + UnwindSafe,
    provider: impl Wire2Api<Provider> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, WalletUnlocked, _>(
        WrapInfo {
            debug_name: "new_from_mnemonic_phrase_with_path__static_method__WalletUnlocked",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_phrase = phrase.wire2api();
            let api_path = path.wire2api();
            let api_provider = provider.wire2api();
            move |task_callback| {
                Result::<_, ()>::Ok(WalletUnlocked::new_from_mnemonic_phrase_with_path(
                    api_phrase,
                    api_path,
                    api_provider,
                ))
            }
        },
    )
}
fn wire_gen_transfer_tx_request__method__WalletUnlocked_impl(
    port_: MessagePort,
    that: impl Wire2Api<WalletUnlocked> + UnwindSafe,
    to: impl Wire2Api<Bech32Address> + UnwindSafe,
    amount: impl Wire2Api<u64> + UnwindSafe,
    asset: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, (Vec<u8>, Vec<u8>), _>(
        WrapInfo {
            debug_name: "gen_transfer_tx_request__method__WalletUnlocked",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_to = to.wire2api();
            let api_amount = amount.wire2api();
            let api_asset = asset.wire2api();
            move |task_callback| {
                Result::<_, ()>::Ok(WalletUnlocked::gen_transfer_tx_request(
                    &api_that, api_to, api_amount, api_asset,
                ))
            }
        },
    )
}
fn wire_send_transaction__method__WalletUnlocked_impl(
    port_: MessagePort,
    that: impl Wire2Api<WalletUnlocked> + UnwindSafe,
    encoded_tx: impl Wire2Api<Vec<u8>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String, _>(
        WrapInfo {
            debug_name: "send_transaction__method__WalletUnlocked",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_encoded_tx = encoded_tx.wire2api();
            move |task_callback| {
                Result::<_, ()>::Ok(WalletUnlocked::send_transaction(&api_that, api_encoded_tx))
            }
        },
    )
}
fn wire_estimate_transaction_cost__method__WalletUnlocked_impl(
    port_: MessagePort,
    that: impl Wire2Api<WalletUnlocked> + UnwindSafe,
    encoded_tx: impl Wire2Api<Vec<u8>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, TransactionCost, _>(
        WrapInfo {
            debug_name: "estimate_transaction_cost__method__WalletUnlocked",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_encoded_tx = encoded_tx.wire2api();
            move |task_callback| {
                Result::<_, ()>::Ok(WalletUnlocked::estimate_transaction_cost(
                    &api_that,
                    api_encoded_tx,
                ))
            }
        },
    )
}
fn wire_sign_message__method__WalletUnlocked_impl(
    port_: MessagePort,
    that: impl Wire2Api<WalletUnlocked> + UnwindSafe,
    message: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String, _>(
        WrapInfo {
            debug_name: "sign_message__method__WalletUnlocked",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_message = message.wire2api();
            move |task_callback| {
                Result::<_, ()>::Ok(WalletUnlocked::sign_message(&api_that, api_message))
            }
        },
    )
}
fn wire_from_bech32_string__static_method__Bech32Address_impl(
    port_: MessagePort,
    s: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, Bech32Address, _>(
        WrapInfo {
            debug_name: "from_bech32_string__static_method__Bech32Address",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_s = s.wire2api();
            move |task_callback| Result::<_, ()>::Ok(Bech32Address::from_bech32_string(api_s))
        },
    )
}
fn wire_from_b256_string__static_method__Bech32Address_impl(
    port_: MessagePort,
    s: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, Bech32Address, _>(
        WrapInfo {
            debug_name: "from_b256_string__static_method__Bech32Address",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_s = s.wire2api();
            move |task_callback| Result::<_, ()>::Ok(Bech32Address::from_b256_string(api_s))
        },
    )
}
fn wire_to_bech32_string__method__Bech32Address_impl(
    port_: MessagePort,
    that: impl Wire2Api<Bech32Address> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String, _>(
        WrapInfo {
            debug_name: "to_bech32_string__method__Bech32Address",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Result::<_, ()>::Ok(Bech32Address::to_bech32_string(&api_that))
        },
    )
}
fn wire_to_b256_string__method__Bech32Address_impl(
    port_: MessagePort,
    that: impl Wire2Api<Bech32Address> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String, _>(
        WrapInfo {
            debug_name: "to_b256_string__method__Bech32Address",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Result::<_, ()>::Ok(Bech32Address::to_b256_string(&api_that))
        },
    )
}
fn wire_connect__static_method__Provider_impl(
    port_: MessagePort,
    url: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, Provider, _>(
        WrapInfo {
            debug_name: "connect__static_method__Provider",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_url = url.wire2api();
            move |task_callback| Result::<_, ()>::Ok(Provider::connect(api_url))
        },
    )
}
// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}

impl Wire2Api<u64> for u64 {
    fn wire2api(self) -> u64 {
        self
    }
}
impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

// Section: impl IntoDart

impl support::IntoDart for Bech32Address {
    fn into_dart(self) -> support::DartAbi {
        vec![self.native.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Bech32Address {}
impl rust2dart::IntoIntoDart<Bech32Address> for Bech32Address {
    fn into_into_dart(self) -> Self {
        self
    }
}

impl support::IntoDart for Provider {
    fn into_dart(self) -> support::DartAbi {
        vec![self.node_url.into_into_dart().into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Provider {}
impl rust2dart::IntoIntoDart<Provider> for Provider {
    fn into_into_dart(self) -> Self {
        self
    }
}

impl support::IntoDart for TransactionCost {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.min_gas_price.into_into_dart().into_dart(),
            self.gas_price.into_into_dart().into_dart(),
            self.gas_used.into_into_dart().into_dart(),
            self.metered_bytes_size.into_into_dart().into_dart(),
            self.total_fee.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for TransactionCost {}
impl rust2dart::IntoIntoDart<TransactionCost> for TransactionCost {
    fn into_into_dart(self) -> Self {
        self
    }
}

impl support::IntoDart for WalletUnlocked {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.private_key.into_into_dart().into_dart(),
            self.mnemonic_phrase.into_dart(),
            self.provider.into_into_dart().into_dart(),
            self.address.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for WalletUnlocked {}
impl rust2dart::IntoIntoDart<WalletUnlocked> for WalletUnlocked {
    fn into_into_dart(self) -> Self {
        self
    }
}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use self::io::*;
