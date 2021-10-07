use crate::mock::*;
use frame_support::{assert_ok, dispatch::{
    DispatchResult, 
}};
use crate::accounts::*;

#[test]
fn it_works_account_add_with_role_and_data() {
	new_test_ext().execute_with(|| {
        let some_new_account = 666;
        let assign_role_result = EvercityAccounts::account_add_with_role_and_data(Origin::signed(ROLES[0].0), some_new_account, CC_INVESTOR_ROLE_MASK);
        assert_ok!(assign_role_result, ());
	});
}

#[test]
fn it_fils_account_add_with_role_and_data_not_master() {
	new_test_ext().execute_with(|| {
        let some_new_account = 666;
        let assign_role_result = EvercityAccounts::account_add_with_role_and_data(Origin::signed(ROLES[1].0), some_new_account, CC_INVESTOR_ROLE_MASK);
        assert_ne!(assign_role_result, DispatchResult::Ok(()));
	});
}

#[test]
fn it_fails_account_set_with_role_and_data_not_exits() {
	new_test_ext().execute_with(|| {
        let some_new_account = 666;
        let assign_role_result = EvercityAccounts::account_set_with_role_and_data(Origin::signed(ROLES[0].0), some_new_account, CC_INVESTOR_ROLE_MASK);
        assert_ne!(assign_role_result, DispatchResult::Ok(()));
	});
}

#[test]
fn it_works_account_set_with_role_and_data() {
	new_test_ext().execute_with(|| {
        let some_new_account = 666;
        let _ = EvercityAccounts::account_add_with_role_and_data(Origin::signed(ROLES[0].0), some_new_account, CC_INVESTOR_ROLE_MASK);
        let assign_role_result = EvercityAccounts::account_set_with_role_and_data(Origin::signed(ROLES[0].0), some_new_account, CC_AUDITOR_ROLE_MASK);
        assert_ok!(assign_role_result, ());
	});
}

#[test]
fn it_fails_account_set_with_role_and_data_not_master() {
	new_test_ext().execute_with(|| {
        let some_new_account = 666;
        let _ = EvercityAccounts::account_add_with_role_and_data(Origin::signed(ROLES[0].0), some_new_account, CC_INVESTOR_ROLE_MASK);
        let assign_role_result = EvercityAccounts::account_set_with_role_and_data(Origin::signed(ROLES[1].0), some_new_account, CC_AUDITOR_ROLE_MASK);
        assert_ne!(assign_role_result, DispatchResult::Ok(()));
	});
}