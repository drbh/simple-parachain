use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

// prompt 1
//
// Please fix the pallet to make this test pass
#[test]
fn basic_escrow_example() {
	new_test_ext().execute_with(|| {
		// get escrow id number
		let escrow_id = 0;

		dbg!("Alice starts a simple escrow contract");
		assert_ok!(EscrowModule::start_escrow(
			Origin::signed(ALICE.into()),
			100,        // asset to put in escrow
			BOB.into(), // person who can recieve
		));

		dbg!("Charlie attempts to withdraw and can't");
		assert_noop!(
			EscrowModule::withdraw_escrow(
				Origin::signed(CHARLIE.into()),
				100,       // asset to put in escrow
				escrow_id, // escrow id
			),
			Error::<Test>::NoneValue
		);

		dbg!("Bob attempts to withdraw and can!");
		assert_ok!(EscrowModule::withdraw_escrow(
			Origin::signed(BOB.into()),
			100,       // asset to put in escrow
			escrow_id, // escrow id
		));
	});
}

// prompt 2
//
// write a test that only lets escrows withdraw if enough time (blocks)
// have past.
#[test]
fn timelocked_escrow_example() {
	new_test_ext().execute_with(|| {
		// get escrow id number
		let escrow_id = 0;

		// set to 10th block for no specific reason
		System::set_block_number(10);

		println!("Alice starts a simple escrow contract");
		assert_ok!(EscrowModule::start_escrow(
			Origin::signed(ALICE.into()),
			100,        // asset to put in escrow
			BOB.into(), // person who can recieve
		));

		dbg!("Bob attempts to withdraw and can't (too early)");
		assert_noop!(
			EscrowModule::withdraw_escrow(
				Origin::signed(BOB.into()),
				100,       // asset to put in escrow
				escrow_id, // escrow id
			),
			Error::<Test>::NoneValue
		);

		dbg!("Bob attempts to withdraw and can");
		assert_noop!(
			EscrowModule::withdraw_escrow(
				Origin::signed(BOB.into()),
				100,       // asset to put in escrow
				escrow_id, // escrow id
			),
			Error::<Test>::NoneValue
		);

		assert!(false)
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
