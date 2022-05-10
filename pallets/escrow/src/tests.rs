use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(EscrowModule::do_something(Origin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(EscrowModule::something(), Some(42));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(EscrowModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
	});
}

#[test]
fn basic_escrow_example() {
	new_test_ext().execute_with(|| {
		// Alice starts a simple escrow contract
		assert_ok!(EscrowModule::start_escrow(
			Origin::signed(ALICE.into()),
			100,        // asset to put in escrow
			BOB.into(), // person who can recieve
		));

		// get escrow id number
		let escrow_id = 1;

		// Charlie attempts to withdraw
		assert_noop!(
			EscrowModule::withdraw_escrow(
				Origin::signed(CHARLIE.into()),
				100,       // asset to put in escrow
				escrow_id, // escrow id
			),
			Error::<Test>::NoneValue
		);

		// Bob attempts to withdraw and can!
		assert_ok!(EscrowModule::withdraw_escrow(
			Origin::signed(BOB.into()),
			100,       // asset to put in escrow
			escrow_id, // escrow id
		));

		assert_eq!(0, 1);
	});
}
