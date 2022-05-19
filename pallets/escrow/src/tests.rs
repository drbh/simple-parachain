use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, traits::fungible::Inspect};

type EscrowEvent = crate::Event<Test>;

fn last_event() -> EscrowEvent {
	System::events()
		.into_iter()
		.map(|r| r.event)
		.filter_map(|e| if let Event::EscrowModule(inner) = e { Some(inner) } else { None })
		.last()
		.expect("Event expected")
}

#[test]
fn cannot_create_escrow_with_zero_amount() {
	new_test_ext().execute_with(|| {
		assert_eq!(Balances::balance(&ALICE.into()), 200);
		System::set_block_number(10);
		dbg!("Alice starts a simple escrow contract, but with zero amount escrow can't be created");
		assert_noop!(EscrowModule::start_escrow(
			Origin::signed(ALICE.into()),
			0,        // asset to put in escrow
			BOB.into(), // person who can recieve
		),
		Error::<Test>::EscrowAmountCannotBeZero
			);
	});
}

#[test]
fn basic_escrow_example() {
	new_test_ext().execute_with(|| {
		// get escrow id number
		let escrow_id = 0;

		assert_eq!(Balances::balance(&ALICE.into()), 200);
		System::set_block_number(10);
		dbg!("Alice starts a simple escrow contract");
		assert_ok!(EscrowModule::start_escrow(
			Origin::signed(ALICE.into()),
			100,        // asset to put in escrow
			BOB.into(), // person who can recieve
		));

		assert_eq!(Balances::balance(&ALICE.into()), 100);
		assert_eq!(
			last_event(),
			crate::Event::EscrowStarted {
				amount: 100,
				initiator: ALICE.into(),
				recipient: BOB.into(),
				timestamp: 10
			}
		);

		System::set_block_number(13);
		dbg!("Charlie attempts to withdraw and can't");
		assert_noop!(
			EscrowModule::withdraw_escrow(
				Origin::signed(CHARLIE.into()),
				100,       // asset to put in escrow
				escrow_id, // escrow id
			),
			Error::<Test>::SenderIsNotRecipient
		);

		assert_eq!(Balances::balance(&BOB.into()), 200);
		dbg!("Bob attempts to withdraw and can!");
		assert_ok!(EscrowModule::withdraw_escrow(
			Origin::signed(BOB.into()),
			100,       // asset to put in escrow
			escrow_id, // escrow id
		));
		assert_eq!(Balances::balance(&BOB.into()), 300);

		assert_eq!(
			last_event(),
			crate::Event::EscrowWithdrawn {
				amount: 100,
				recipient: BOB.into(),
				escrow_timestamp: 10,
				withdrawn_timestamp: 13
			}
		);
	});
}

#[test]
fn timelocked_escrow_example() {
	new_test_ext().execute_with(|| {
		// get escrow id number
		let escrow_id = 0;

		// set to 10th block for no specific reason
		System::set_block_number(10);

		assert_eq!(Balances::balance(&ALICE.into()), 200);
		println!("Alice starts a simple escrow contract");
		assert_ok!(EscrowModule::start_escrow(
			Origin::signed(ALICE.into()),
			100,        // asset to put in escrow
			BOB.into(), // person who can recieve
		));

		assert_eq!(Balances::balance(&ALICE.into()), 100);
		assert_eq!(
			last_event(),
			crate::Event::EscrowStarted {
				amount: 100,
				initiator: ALICE.into(),
				recipient: BOB.into(),
				timestamp: 10
			}
		);

		dbg!("Bob attempts to withdraw and can't (too early)");
		assert_noop!(
			EscrowModule::withdraw_escrow(
				Origin::signed(BOB.into()),
				100,       // asset to put in escrow
				escrow_id, // escrow id
			),
			Error::<Test>::CannotWithdrawBeforeTime
		);

		assert_eq!(Balances::balance(&BOB.into()), 200);
		System::set_block_number(13);
		dbg!("Bob attempts to withdraw and can");
		assert_ok!(EscrowModule::withdraw_escrow(
			Origin::signed(BOB.into()),
			100,       // asset to put in escrow
			escrow_id, // escrow id
		));
		assert_eq!(Balances::balance(&BOB.into()), 300);

		assert_eq!(
			last_event(),
			crate::Event::EscrowWithdrawn {
				amount: 100,
				recipient: BOB.into(),
				escrow_timestamp: 10,
				withdrawn_timestamp: 13
			}
		);
	});
}

#[test]
fn invalid_escrow_id() {
	new_test_ext().execute_with(|| {
		// set to 10th block for no specific reason
		System::set_block_number(10);

		println!("Alice starts a simple escrow contract");
		assert_ok!(EscrowModule::start_escrow(
			Origin::signed(ALICE.into()),
			100,        // asset to put in escrow
			BOB.into(), // person who can recieve
		));

		assert_eq!(Balances::balance(&ALICE.into()), 100);
		assert_eq!(
			last_event(),
			crate::Event::EscrowStarted {
				amount: 100,
				initiator: ALICE.into(),
				recipient: BOB.into(),
				timestamp: 10
			}
		);

		dbg!("Bob attempts to withdraw and can't (wrong escrow_id)");
		assert_noop!(
			EscrowModule::withdraw_escrow(
				Origin::signed(BOB.into()),
				100, // asset to put in escrow
				1,   // escrow id
			),
			Error::<Test>::NoEscrowFound
		);
	});
}

#[test]
fn storage_updated_correctly_with_withdraw_escrow() {
	new_test_ext().execute_with(|| {
		// set to 10th block for no specific reason
		System::set_block_number(10);

		println!("Alice starts a simple escrow contract");
		assert_ok!(EscrowModule::start_escrow(
			Origin::signed(ALICE.into()),
			100,        // asset to put in escrow
			BOB.into(), // person who can recieve
		));
		assert_eq!(Balances::balance(&ALICE.into()), 100);
		assert_eq!(
			last_event(),
			crate::Event::EscrowStarted {
				amount: 100,
				initiator: ALICE.into(),
				recipient: BOB.into(),
				timestamp: 10
			}
		);

		System::set_block_number(13);
		dbg!("Bob attempts to withdraw 20 tokens and can withdraw successfully!");
		assert_ok!(EscrowModule::withdraw_escrow(
			Origin::signed(BOB.into()),
			20, // asset to withdraw from escrow
			0,  // escrow id
		));
		assert_eq!(Balances::balance(&BOB.into()), 220);
		assert_eq!(
			last_event(),
			crate::Event::EscrowWithdrawn {
				amount: 20,
				recipient: BOB.into(),
				escrow_timestamp: 10,
				withdrawn_timestamp: 13
			}
		);
		dbg!("Bob attempts to withdraw 100 tokens, but escrow has 80 only, so fails.");
		assert_noop!(
			EscrowModule::withdraw_escrow(
				Origin::signed(BOB.into()),
				100, // asset to withdraw from escrow
				0,   // escrow id
			),
			Error::<Test>::CannotWithdrawAmountGraterThanEscrow
		);
		dbg!(
			"Bob attempts to withdraw remaining 80 tokens and can withdraw successfully! This also
			removes escrow"
		);
		assert_ok!(EscrowModule::withdraw_escrow(
			Origin::signed(BOB.into()),
			80, // asset to withdraw from escrow
			0,  // escrow id
		));
		assert_eq!(Balances::balance(&BOB.into()), 300);

		assert_eq!(
			last_event(),
			crate::Event::EscrowWithdrawn {
				amount: 80,
				recipient: BOB.into(),
				escrow_timestamp: 10,
				withdrawn_timestamp: 13
			}
		);
		dbg!("Bob attempts to withdraw 20 tokens from escrow which does not exist");
		assert_noop!(
			EscrowModule::withdraw_escrow(
				Origin::signed(BOB.into()),
				20, // asset to withdraw from escrow
				0,  // escrow id
			),
			Error::<Test>::NoEscrowFound
		);
	});
}

// Extra prompts

// A) so far all of the amounts are not linked to any asset
// - how might we use assets instead of arbitraty values

// possible solutions:
//
// maybe we can complete the mint and balance_of functions in the pallet
// and add create our own token. Then we can use this in our escrow logic above
//
// maybe we can use the pallet_assets or pallet_balances to add native token
// functionality into our pallet for our escrow logic
//
// B) what other things can we do with an escrow? we use time and recipient above
// what are some other ways we can leverage an escrow. What might a use case look like?
//
// C) please point out, fix and optimize this pallet code. What improvements can be made?
