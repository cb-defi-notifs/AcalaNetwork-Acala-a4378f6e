// This file is part of Acala.

// Copyright (C) 2020-2022 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use crate::evm::alice_evm_addr;
use crate::payment::{with_fee_aggregated_path_call, with_fee_currency_call, with_fee_path_call, INFO, POST_INFO};
use crate::setup::*;
use module_aggregated_dex::SwapPath;
use module_support::{AggregatedSwapPath, ExchangeRate, Swap, SwapLimit, EVM as EVMTrait};
use nutsfinance_stable_asset::traits::StableAsset as StableAssetT;
use primitives::{currency::AssetMetadata, evm::EvmAddress};
use sp_runtime::{
	traits::{SignedExtension, UniqueSaturatedInto},
	transaction_validity::{InvalidTransaction, TransactionValidityError},
	Percent,
};
use std::str::FromStr;

fn inject_liquidity(
	account_id: AccountId,
	currency_id_a: CurrencyId,
	currency_id_b: CurrencyId,
	max_amount_a: Balance,
	max_amount_b: Balance,
) -> Result<(), &'static str> {
	let _ = Dex::enable_trading_pair(Origin::root(), currency_id_a, currency_id_b);
	Dex::add_liquidity(
		Origin::signed(account_id),
		currency_id_a,
		currency_id_b,
		max_amount_a,
		max_amount_b,
		Default::default(),
		false,
	)?;
	Ok(())
}

pub fn enable_default_stable_asset(currencies: Vec<CurrencyId>, amounts: Vec<u128>, minter: Option<AccountId>) {
	let precisions = currencies.iter().map(|_| 1u128).collect::<Vec<_>>();
	enable_stable_asset(
		currencies,
		amounts,
		precisions,
		minter,
		10_000_000u128,
		20_000_000u128,
		50_000_000u128,
		1_000u128,
	);
}

pub fn mock_production_stable_asset(
	currencies: Vec<CurrencyId>,
	amounts: Vec<u128>,
	minter: Option<AccountId>,
	precisions: Vec<u128>,
) {
	enable_stable_asset(
		currencies,
		amounts,
		precisions,
		minter,
		0u128,
		25_000_000u128,
		30_000_000u128,
		3_000u128,
	);
}

fn enable_stable_asset(
	currencies: Vec<CurrencyId>,
	amounts: Vec<u128>,
	precisions: Vec<u128>,
	minter: Option<AccountId>,
	mint_fee: u128,
	swap_fee: u128,
	redeem_fee: u128,
	initial: u128,
) {
	let pool_asset = CurrencyId::StableAssetPoolToken(0);
	assert_ok!(StableAsset::create_pool(
		Origin::root(),
		pool_asset,
		currencies, // assets
		precisions,
		mint_fee,
		swap_fee,
		redeem_fee,
		initial,
		AccountId::from(BOB),     // fee recipient
		AccountId::from(CHARLIE), // yield recipient
		1_000_000_000_000u128,    // precision
	));

	let asset_metadata = AssetMetadata {
		name: b"Token Name".to_vec(),
		symbol: b"TN".to_vec(),
		decimals: 12,
		minimal_balance: 1,
	};
	assert_ok!(AssetRegistry::register_stable_asset(
		RawOrigin::Root.into(),
		Box::new(asset_metadata.clone())
	));

	assert_ok!(StableAsset::mint(
		Origin::signed(minter.unwrap_or(AccountId::from(ALICE))),
		0,
		amounts,
		0u128
	));
}

#[test]
fn stable_asset_mint_works() {
	ExtBuilder::default()
		.balances(vec![
			(
				AccountId::from(ALICE),
				RELAY_CHAIN_CURRENCY,
				1_000_000_000 * dollar(NATIVE_CURRENCY),
			),
			(
				AccountId::from(ALICE),
				LIQUID_CURRENCY,
				12_000_000_000 * dollar(NATIVE_CURRENCY),
			),
		])
		.build()
		.execute_with(|| {
			let exchange_rate = Homa::current_exchange_rate();
			assert_eq!(exchange_rate, ExchangeRate::saturating_from_rational(1, 10)); // 0.1

			let ksm_target_amount = 10_000_123u128;
			let lksm_target_amount = 10_000_456u128;
			let account_id: AccountId = StableAssetPalletId::get().into_sub_account_truncating(0);
			enable_default_stable_asset(
				vec![RELAY_CHAIN_CURRENCY, LIQUID_CURRENCY],
				vec![ksm_target_amount, lksm_target_amount],
				None,
			);
			System::assert_last_event(Event::StableAsset(nutsfinance_stable_asset::Event::Minted {
				minter: AccountId::from(ALICE),
				pool_id: 0,
				a: 1000,
				input_amounts: vec![10_000_123u128, 10_000_456u128],
				min_output_amount: 0,
				balances: vec![10_000_123u128, 10_000_456u128],
				total_supply: 20_000_579u128,
				fee_amount: 20000,
				output_amount: 19_980_579u128,
			}));

			let ksm_balance = Currencies::free_balance(RELAY_CHAIN_CURRENCY, &account_id);
			let lksm_balance = Currencies::free_balance(LIQUID_CURRENCY, &account_id);
			assert_eq!(ksm_target_amount, ksm_balance);

			let lksm_amount = 100_004_560u128;
			assert_eq!(lksm_amount, lksm_balance);

			let converted_lksm_balance = exchange_rate.checked_mul_int(lksm_balance).unwrap_or_default();
			assert_eq!(converted_lksm_balance == lksm_target_amount, true);
		});
}

#[test]
fn three_usd_pool_mint_redeem_works() {
	let dollar = dollar(NATIVE_CURRENCY);
	let alith = MockAddressMapping::get_account_id(&alice_evm_addr());
	let usdt: CurrencyId = CurrencyId::ForeignAsset(0);
	let usdc: CurrencyId = CurrencyId::Erc20(erc20_address_0());
	let minimal_balance = 10_000u128;
	ExtBuilder::default()
		.balances(vec![
			// alice() used to deploy erc20 contract
			(alice(), NATIVE_CURRENCY, 1_000_000 * dollar),
			// alith used to mint 3USD.
			(alith.clone(), NATIVE_CURRENCY, 1_000_000_000 * dollar),
			(alith.clone(), USD_CURRENCY, 1_000_000_000 * dollar),
			(AccountId::from(BOB), USD_CURRENCY, 1_000_000 * dollar),
		])
		.build()
		.execute_with(|| {
			enable_usdt(6, minimal_balance);
			assert_ok!(Currencies::deposit(usdt, &alith, 1_000_000_000 * dollar));

			// USDC is Erc20 token, decimals=6
			deploy_usdc_erc20(minimal_balance);

			<EVM as EVMTrait<AccountId>>::set_origin(alith.clone());

			mock_production_stable_asset(
				vec![
					USD_CURRENCY, // 0: AUSD
					usdc,         // 1: USDC, decimals=6
					usdt,         // 2: USDT, decimals=6
				],
				vec![320_928_161_121_617_931u128, 902_252_973_831u128, 641_091_908_733u128],
				Some(alith.clone()),
				vec![1u128, 1_000_000u128, 1_000_000u128],
			);

			let lp_token = CurrencyId::StableAssetPoolToken(0);
			let amount1 = 10_000 * dollar;
			let amount2 = 100_000 * dollar;

			// mint/inject will increate lp
			let mut lp = mint_or_redeem(true, amount1, 0, 17, 0);
			for i in vec![18, 18, 18, 19, 19, 20] {
				lp = mint_or_redeem(true, amount1, lp, i, 0);
			}
			for i in vec![24, 27] {
				lp = mint_or_redeem(true, amount2, lp, i, 0);
			}

			// burn/redeem will decrease lp
			for i in vec![24, 20] {
				lp = mint_or_redeem(false, amount2, lp, i, 1004);
			}
			for i in vec![19, 19, 19, 18, 18] {
				lp = mint_or_redeem(false, amount1, lp, i, 1004);
			}
			for (a, i) in vec![(21281 * dollar, 17), (64 * dollar, 17), (dollar, 17)] {
				let lp_amount = Tokens::free_balance(lp_token, &AccountId::from(BOB));
				assert!(lp_amount < a);
				lp = mint_or_redeem(false, lp_amount, lp, i, 1004);
			}
		});
}

fn mint_or_redeem(mint: bool, amount: Balance, lp: Balance, i: u32, j: u32) -> Balance {
	let lp_token = CurrencyId::StableAssetPoolToken(0);
	if !mint {
		// redeem
		assert_ok!(StableAsset::redeem_single(
			Origin::signed(AccountId::from(BOB)),
			0,
			amount,
			0,
			0,
			3
		));
	} else {
		// mint
		assert_ok!(StableAsset::mint(
			Origin::signed(AccountId::from(BOB)),
			0,
			vec![amount, 0, 0],
			0u128
		));
	}
	let pool = StableAsset::pool(0).unwrap();
	let lp1 = Tokens::free_balance(lp_token, &AccountId::from(BOB));
	let asud_proportion = Ratio::saturating_from_rational(pool.balances[0], pool.total_supply);
	let rate = if !mint {
		Ratio::saturating_from_rational(amount, lp - lp1)
	} else {
		Ratio::saturating_from_rational(amount, lp1 - lp)
	};
	assert!(
		asud_proportion > Ratio::saturating_from_rational(i, 100)
			&& asud_proportion < Ratio::saturating_from_rational(i + 1, 100)
	);
	if !mint {
		assert!(rate > Ratio::saturating_from_rational(1, 1) && rate < Ratio::saturating_from_rational(j, 1000));
	} else {
		assert!(rate < Ratio::saturating_from_rational(1, 1));
	}

	lp1
}

#[test]
fn three_usd_pool_payment_works() {
	let dollar = dollar(NATIVE_CURRENCY);
	let alith = MockAddressMapping::get_account_id(&alice_evm_addr());

	let usdt: CurrencyId = CurrencyId::ForeignAsset(0);
	let usdc: CurrencyId = CurrencyId::Erc20(erc20_address_0());
	ExtBuilder::default()
		.balances(vec![
			// alice() used to deploy erc20 contract
			(alice(), NATIVE_CURRENCY, 1_000_000 * dollar),
			// alith used to mint 3USD.
			(alith.clone(), NATIVE_CURRENCY, 1_000_000_000 * dollar),
			(alith.clone(), USD_CURRENCY, 1_000_000_000 * dollar),
			(AccountId::from(ALICE), USD_CURRENCY, 1_000_000 * dollar),
			(AccountId::from(BOB), USD_CURRENCY, 1_000_000 * dollar),
			(AccountId::from(BOB), NATIVE_CURRENCY, 1_000_000 * dollar),
		])
		.build()
		.execute_with(|| {
			enable_3usd_pool(alith.clone());

			// deposit USDT/USDC to BOB, used for swap
			assert_ok!(Currencies::deposit(usdt, &AccountId::from(BOB), 1_000_000 * dollar));
			assert_ok!(EvmAccounts::claim_account(
				Origin::signed(AccountId::from(BOB)),
				EvmAccounts::eth_address(&bob_key()),
				EvmAccounts::eth_sign(&bob_key(), &AccountId::from(BOB))
			));
			assert_ok!(Currencies::transfer(
				Origin::signed(alith.clone()),
				sp_runtime::MultiAddress::Id(AccountId::from(BOB)),
				usdc,
				10 * dollar,
			));
			assert_eq!(Currencies::free_balance(usdc, &AccountId::from(BOB)), 10 * dollar);
			assert_eq!(Currencies::free_balance(usdc, &bob()), 10 * dollar);

			enable_aggregated_dex();

			evm_origin_works();

			// USDC=Erc20(contract) or USDT=ForeignAsset(0) as fee token.
			// before USDC/USDT enabled as fee pool, it works by direct swap.
			assert_aggregated_dex_event(usdc, with_fee_currency_call(usdc), None);
			assert_aggregated_dex_event(usdt, with_fee_currency_call(usdt), None);

			with_fee_call_works();

			// enable USDT/USDC as charge fee pool
			let len = enable_charge_fee_pool();
			assert_ok!(
				<module_transaction_payment::ChargeTransactionPayment<Runtime>>::from(0).validate(
					&AccountId::from(BOB),
					&with_fee_currency_call(usdt),
					&INFO,
					len,
				)
			);
			assert_ok!(
				<module_transaction_payment::ChargeTransactionPayment<Runtime>>::from(0).validate(
					&AccountId::from(BOB),
					&with_fee_currency_call(usdc),
					&INFO,
					len,
				)
			);
			// when sub-account has not enough native token, trigger swap
			assert_aggregated_dex_event(usdt, with_fee_currency_call(usdt), Some(len));
			assert_aggregated_dex_event(usdc, with_fee_currency_call(usdc), Some(len));
		});
}

fn assert_aggregated_dex_event(
	_usd_token: CurrencyId,
	with_fee_call: <Runtime as module_transaction_payment::Config>::Call,
	len: Option<usize>,
) {
	System::reset_events();
	assert_ok!(
		<module_transaction_payment::ChargeTransactionPayment::<Runtime>>::from(0).validate(
			&AccountId::from(BOB),
			&with_fee_call,
			&INFO,
			len.unwrap_or(50)
		)
	);
	assert!(System::events().iter().any(|r| matches!(
		r.event,
		Event::StableAsset(nutsfinance_stable_asset::Event::TokenSwapped {
			pool_id: 0,
			a: 1000,
			input_asset: _usd_token,
			output_asset: USD_CURRENCY,
			..
		})
	)));
	assert!(System::events()
		.iter()
		.any(|r| matches!(r.event, Event::Dex(module_dex::Event::Swap { .. }))));
}

pub fn deploy_default_erc20() {
	deploy_erc20_contracts(
		include_str!("../../../ts-tests/build/Erc20DemoContract2.json"),
		100_000_000_000,
	);
}

pub fn deploy_usdc_erc20(minimal_balance: Balance) {
	deploy_erc20_contracts(
		include_str!("../../../ts-tests/build/Erc20DemoContract1.json"),
		minimal_balance,
	);
}

pub fn deploy_erc20_contracts(json_file: &str, minimal_balance: Balance) {
	let json: serde_json::Value = serde_json::from_str(json_file).unwrap();
	let code = hex::decode(json.get("bytecode").unwrap().as_str().unwrap()).unwrap();

	assert_ok!(EVM::create(Origin::signed(alice()), code, 0, 2100_000, 100000, vec![]));
	assert_ok!(EVM::publish_free(Origin::root(), erc20_address_0()));
	assert_ok!(AssetRegistry::register_erc20_asset(
		Origin::root(),
		erc20_address_0(),
		minimal_balance
	));
}

pub fn erc20_address_0() -> EvmAddress {
	EvmAddress::from_str("0x5e0b4bfa0b55932a3587e648c3552a6515ba56b1").unwrap()
}

fn enable_usdt(decimals: u8, minimal_balance: Balance) {
	// USDT is asset on Statemine
	assert_ok!(AssetRegistry::register_foreign_asset(
		Origin::root(),
		Box::new(
			MultiLocation::new(
				1,
				X2(
					Parachain(1000),
					GeneralKey("USDT".as_bytes().to_vec().try_into().unwrap())
				)
			)
			.into()
		),
		Box::new(AssetMetadata {
			name: b"USDT".to_vec(),
			symbol: b"USDT".to_vec(),
			decimals,
			minimal_balance
		})
	));
}

pub fn enable_3usd_pool(account: AccountId) {
	let dollar = dollar(NATIVE_CURRENCY);
	let minimal_balance: u128 = Balances::minimum_balance() / 10;
	let usdt: CurrencyId = CurrencyId::ForeignAsset(0);
	let usdc: CurrencyId = CurrencyId::Erc20(erc20_address_0());

	enable_usdt(12, minimal_balance);
	// deposit USDT to alith, used for 3USD liquidity provider
	assert_ok!(Currencies::deposit(usdt, &account, 1_000_000 * dollar));

	// USDC is Erc20 token
	deploy_default_erc20();

	<EVM as EVMTrait<AccountId>>::set_origin(account.clone());

	// create three stable asset pool
	let three_usds = vec![
		usdt,         // PoolTokenIndex=0: USDT
		usdc,         // PoolTokenIndex=1: USDC
		USD_CURRENCY, // PoolTokenIndex=2: AUSD
	];
	enable_default_stable_asset(
		three_usds,
		vec![1000 * dollar, 1000 * dollar, 1000 * dollar],
		Some(account.clone()),
	);
	System::assert_last_event(Event::StableAsset(nutsfinance_stable_asset::Event::Minted {
		minter: account,
		pool_id: 0,
		a: 1000,
		input_amounts: vec![1000 * dollar, 1000 * dollar, 1000 * dollar],
		min_output_amount: 0,
		balances: vec![1000 * dollar, 1000 * dollar, 1000 * dollar],
		total_supply: 3000 * dollar,
		fee_amount: 3 * dollar,
		output_amount: 2997 * dollar,
	}));
}

fn enable_aggregated_dex() {
	let dollar = dollar(NATIVE_CURRENCY);
	let alith = MockAddressMapping::get_account_id(&alice_evm_addr());

	let usdt: CurrencyId = CurrencyId::ForeignAsset(0);
	let usdc: CurrencyId = CurrencyId::Erc20(erc20_address_0());

	// inject liquidity of AUSD to native token.
	assert_ok!(inject_liquidity(
		alith,
		USD_CURRENCY,
		NATIVE_CURRENCY,
		1000 * dollar,
		10000 * dollar
	));
	assert_eq!(
		Dex::get_liquidity_pool(USD_CURRENCY, NATIVE_CURRENCY),
		(1000 * dollar, 10000 * dollar)
	);
	// Taiga(USDT, AUSD), Dex(AUSD, ACA)
	assert_ok!(AggregatedDex::update_aggregated_swap_paths(
		Origin::root(),
		vec![(
			(CurrencyId::ForeignAsset(0), NATIVE_CURRENCY),
			Some(vec![
				SwapPath::Taiga(0, 0, 2),
				SwapPath::Dex(vec![USD_CURRENCY, NATIVE_CURRENCY])
			])
		),]
	));
	// Taiga(USDC, AUSD), Dex(AUSD, ACA)
	assert_ok!(AggregatedDex::update_aggregated_swap_paths(
		Origin::root(),
		vec![(
			(usdc, NATIVE_CURRENCY),
			Some(vec![
				SwapPath::Taiga(0, 1, 2),
				SwapPath::Dex(vec![USD_CURRENCY, NATIVE_CURRENCY])
			])
		),]
	));
	// AggregatedDex::swap works: USDC->AUSD->ACA, USDT->AUSD->ACA, AUSD->ACA
	let usd_tokens: Vec<CurrencyId> = vec![usdc, usdt, USD_CURRENCY];
	#[cfg(any(feature = "with-karura-runtime", feature = "with-acala-runtime"))]
	let swap_amounts: Vec<u128> = vec![9_940_060_348_765u128, 9_920_180_467_236u128, 9_920_507_587_087u128];
	#[cfg(feature = "with-mandala-runtime")]
	let swap_amounts: Vec<u128> = vec![9_959_980_429_142u128, 9_940_040_907_508u128, 9_940_348_860_887u128];
	for (token, swap_amount) in usd_tokens.iter().zip(swap_amounts.iter()) {
		assert_eq!(
			AcalaSwap::swap(
				&AccountId::from(BOB),
				*token,
				NATIVE_CURRENCY,
				SwapLimit::ExactSupply(dollar, 0)
			),
			Ok((dollar, *swap_amount))
		);
	}
}

fn evm_origin_works() {
	let usdc: CurrencyId = CurrencyId::Erc20(erc20_address_0());

	let set_evm_origin = module_evm::SetEvmOrigin::<Runtime>::new();
	let pre = set_evm_origin
		.clone()
		.pre_dispatch(&AccountId::from(BOB), &with_fee_currency_call(usdc), &INFO, 50)
		.unwrap();

	let origin =
		<module_evm_bridge::EVMBridge<Runtime> as module_support::evm::EVMBridge<AccountId, Balance>>::get_origin();
	assert_eq!(origin, Some(AccountId::from(BOB)));

	assert_ok!(module_evm::SetEvmOrigin::<Runtime>::post_dispatch(
		Some(pre),
		&INFO,
		&POST_INFO,
		50,
		&Ok(())
	));
	let origin =
		<module_evm_bridge::EVMBridge<Runtime> as module_support::evm::EVMBridge<AccountId, Balance>>::get_origin();
	assert_eq!(origin, None);

	// Origin is None, transfer erc20 failed.
	assert_noop!(
		<module_transaction_payment::ChargeTransactionPayment::<Runtime>>::from(0).validate(
			&AccountId::from(BOB),
			&with_fee_currency_call(usdc),
			&INFO,
			50
		),
		TransactionValidityError::Invalid(InvalidTransaction::Payment)
	);

	// set origin in SetEvmOrigin::validate() then transfer erc20 will success.
	assert_ok!(set_evm_origin.validate(&AccountId::from(BOB), &with_fee_currency_call(usdc), &INFO, 50));
	let origin =
		<module_evm_bridge::EVMBridge<Runtime> as module_support::evm::EVMBridge<AccountId, Balance>>::get_origin();
	assert_eq!(origin, Some(AccountId::from(BOB)));
}

fn enable_charge_fee_pool() -> usize {
	let dollar = dollar(NATIVE_CURRENCY);
	let alith = MockAddressMapping::get_account_id(&alice_evm_addr());

	let usdt: CurrencyId = CurrencyId::ForeignAsset(0);
	let usdc: CurrencyId = CurrencyId::Erc20(erc20_address_0());
	let usdt_sub_account: AccountId = TransactionPaymentPalletId::get().into_sub_account_truncating(usdt);
	let usdc_sub_account: AccountId = TransactionPaymentPalletId::get().into_sub_account_truncating(usdc);

	let minimal_balance: u128 = Balances::minimum_balance() / 10;
	let usdt_ed: u128 = (<Currencies as MultiCurrency<AccountId>>::minimum_balance(usdt)).unique_saturated_into();
	let usdc_ed: u128 = (<Currencies as MultiCurrency<AccountId>>::minimum_balance(usdc)).unique_saturated_into();
	assert_eq!(usdt_ed, minimal_balance);
	assert_eq!(usdc_ed, 0); // erc20 minimum_balance/ED is 0.

	// make sure treasury account has native and none native token.
	let treasury_account = TreasuryAccount::get();
	assert_ok!(Currencies::update_balance(
		Origin::root(),
		MultiAddress::Id(treasury_account.clone()),
		NATIVE_CURRENCY,
		100 * dollar as i128,
	));
	assert_ok!(Currencies::deposit(usdt, &treasury_account, 10 * dollar));
	assert_ok!(Currencies::transfer(
		Origin::signed(alith.clone()),
		sp_runtime::MultiAddress::Id(treasury_account.clone()),
		usdc,
		10 * dollar,
	));

	// enable fee pool for none native token
	#[cfg(any(feature = "with-karura-runtime", feature = "with-acala-runtime"))]
	let len = 33300;
	#[cfg(feature = "with-mandala-runtime")]
	let len = 3330;
	let fee = module_transaction_payment::Pallet::<Runtime>::compute_fee(len, &INFO, 0);
	let surplus_perc = Percent::from_percent(50); // CustomFeeSurplus
	let fee_surplus = surplus_perc.mul_ceil(fee);
	let fee = fee + fee_surplus; // 501,000,001,739
	let fee_pool_size = 5 * dollar;
	// set threshold to large value, so that after a few tx, trigger swap.
	assert_ok!(TransactionPayment::enable_charge_fee_pool(
		Origin::root(),
		usdt,
		fee_pool_size,
		fee_pool_size - fee,
	));
	assert_ok!(TransactionPayment::enable_charge_fee_pool(
		Origin::root(),
		usdc,
		fee_pool_size,
		fee_pool_size - fee,
	));
	assert_eq!(
		fee_pool_size,
		Currencies::free_balance(NATIVE_CURRENCY, &usdt_sub_account)
	);
	assert_eq!(
		fee_pool_size,
		Currencies::free_balance(NATIVE_CURRENCY, &usdc_sub_account)
	);
	assert_eq!(usdt_ed, Currencies::free_balance(usdt, &usdt_sub_account));
	assert_eq!(usdc_ed, Currencies::free_balance(usdc, &usdc_sub_account));
	assert!(module_transaction_payment::Pallet::<Runtime>::token_exchange_rate(usdt).is_some());
	assert!(module_transaction_payment::Pallet::<Runtime>::token_exchange_rate(usdc).is_some());

	// validate on payment works
	let rate = module_transaction_payment::Pallet::<Runtime>::token_exchange_rate(usdt).unwrap();
	let usd_fee_amount: u128 = rate.saturating_mul_int(fee);
	let usdt_amount = Currencies::free_balance(usdt, &AccountId::from(BOB));
	let usdc_amount = Currencies::free_balance(usdc, &AccountId::from(BOB));
	assert_ok!(
		<module_transaction_payment::ChargeTransactionPayment<Runtime>>::from(0).validate(
			&AccountId::from(BOB),
			&with_fee_currency_call(usdt),
			&INFO,
			len as usize,
		)
	);
	assert_ok!(
		<module_transaction_payment::ChargeTransactionPayment<Runtime>>::from(0).validate(
			&AccountId::from(BOB),
			&with_fee_currency_call(usdc),
			&INFO,
			len as usize,
		)
	);
	assert_eq!(
		usd_fee_amount,
		usdt_amount - Currencies::free_balance(usdt, &AccountId::from(BOB))
	);
	assert_eq!(
		usd_fee_amount,
		usdc_amount - Currencies::free_balance(usdc, &AccountId::from(BOB))
	);
	assert_eq!(
		fee,
		fee_pool_size - Currencies::free_balance(NATIVE_CURRENCY, &usdc_sub_account)
	);
	assert_eq!(
		fee,
		fee_pool_size - Currencies::free_balance(NATIVE_CURRENCY, &usdt_sub_account)
	);

	len as usize
}

fn with_fee_call_works() {
	let usdt: CurrencyId = CurrencyId::ForeignAsset(0);
	let usdc: CurrencyId = CurrencyId::Erc20(erc20_address_0());

	// AUSD as fee token, only dex swap event produced.
	assert_ok!(
		<module_transaction_payment::ChargeTransactionPayment::<Runtime>>::from(0).validate(
			&AccountId::from(BOB),
			&with_fee_currency_call(USD_CURRENCY),
			&INFO,
			50
		)
	);
	#[cfg(any(feature = "with-karura-runtime", feature = "with-acala-runtime"))]
	let (amount1, amount2) = (227029695u128, 2250001739u128);
	#[cfg(feature = "with-mandala-runtime")]
	let (amount1, amount2) = (906308684u128, 9000001739u128);
	System::assert_has_event(Event::Dex(module_dex::Event::Swap {
		trader: AccountId::from(BOB),
		path: vec![USD_CURRENCY, NATIVE_CURRENCY],
		liquidity_changes: vec![amount1, amount2],
	}));

	// with_fee_path_call failed
	let invalid_swap_path = vec![
		vec![usdt, USD_CURRENCY, NATIVE_CURRENCY],
		vec![usdt, USD_CURRENCY],
		vec![usdt, NATIVE_CURRENCY],
		vec![usdc, USD_CURRENCY, NATIVE_CURRENCY],
		vec![usdc, USD_CURRENCY],
		vec![usdc, NATIVE_CURRENCY],
	];
	for path in invalid_swap_path {
		assert_noop!(
			<module_transaction_payment::ChargeTransactionPayment::<Runtime>>::from(0).validate(
				&AccountId::from(BOB),
				&with_fee_path_call(path),
				&INFO,
				50
			),
			TransactionValidityError::Invalid(InvalidTransaction::Payment)
		);
	}
	// USD_CURRENCY to NATIVE_CURRENCY is valid, because it exist in dex swap.
	assert_ok!(
		<module_transaction_payment::ChargeTransactionPayment::<Runtime>>::from(0).validate(
			&AccountId::from(BOB),
			&with_fee_path_call(vec![USD_CURRENCY, NATIVE_CURRENCY]),
			&INFO,
			50
		)
	);

	// with_fee_aggregated_path_call also works by direct swap.
	let usdt_aggregated_path = vec![
		AggregatedSwapPath::<CurrencyId>::Taiga(0, 0, 2), // USDT, AUSD
		AggregatedSwapPath::<CurrencyId>::Dex(vec![USD_CURRENCY, NATIVE_CURRENCY]),
	];
	let usdc_aggregated_path = vec![
		AggregatedSwapPath::<CurrencyId>::Taiga(0, 1, 2), // USDC, AUSD
		AggregatedSwapPath::<CurrencyId>::Dex(vec![USD_CURRENCY, NATIVE_CURRENCY]),
	];
	let invalid_aggregated_path = vec![
		AggregatedSwapPath::<CurrencyId>::Taiga(0, 0, 1), // USDT, USDC
		AggregatedSwapPath::<CurrencyId>::Dex(vec![USD_CURRENCY, NATIVE_CURRENCY]),
	];
	assert_noop!(
		<module_transaction_payment::ChargeTransactionPayment::<Runtime>>::from(0).validate(
			&AccountId::from(BOB),
			&with_fee_aggregated_path_call(invalid_aggregated_path),
			&INFO,
			50
		),
		TransactionValidityError::Invalid(InvalidTransaction::Payment)
	);

	assert_aggregated_dex_event(usdc, with_fee_aggregated_path_call(usdc_aggregated_path), None);
	assert_aggregated_dex_event(usdt, with_fee_aggregated_path_call(usdt_aggregated_path), None);
}
