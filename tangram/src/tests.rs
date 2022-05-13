use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use sp_core::H256;


#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		
        // Dispatch a signed extrinsic.
		assert_ok!(TangramModule::create_realm(Origin::root(), H256::random()));

		// Read pallet storage and assert an expected result.
		// assert_eq!(TemplateModule::something(), Some(42));
	});
}