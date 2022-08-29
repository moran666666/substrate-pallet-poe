use crate::{mock::*, Error, Config, Proofs};
use frame_support::{assert_noop, assert_ok, BoundedVec};

// create_claim_works 创建存证的用例测试
#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let bounded_claim = BoundedVec::<u8, <Test as Config>::MaxBytesInHash>::try_from(claim).unwrap();

		assert_ok!(PoeModule::create_claim(Origin::signed(1), bounded_claim.clone()));

		assert_eq!(
			Proofs::<Test>::get(&bounded_claim),
			Some((1, frame_system::Pallet::<Test>::block_number()))
		);
	});
}

// create_claim_failed_when_already_claimed 创建已存在的存证失败用例测试
#[test]
fn create_claim_failed_when_already_claimed() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let bounded_claim = BoundedVec::<u8, <Test as Config>::MaxBytesInHash>::try_from(claim).unwrap();

		let _ = PoeModule::create_claim(Origin::signed(1), bounded_claim.clone()); // 先创建存证
		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), bounded_claim.clone()),  // 再次创建跟上面一样的存证，因为上面已创建，这将会导致创建失败
			Error::<Test>::ProofAlreadyClaimed
		);
	});
}

// revoke_claim_works 撤消存证的用例测试
#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let bounded_claim = BoundedVec::<u8, <Test as Config>::MaxBytesInHash>::try_from(claim).unwrap();

		let _ = PoeModule::create_claim(Origin::signed(1), bounded_claim.clone()); // 先创建存证

		assert_ok!(PoeModule::revoke_claim(Origin::signed(1), bounded_claim.clone())); // 断言：撤消存证
	});
}

// transfer_claim_works 转移存证的用例测试
#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let bounded_claim = BoundedVec::<u8, <Test as Config>::MaxBytesInHash>::try_from(claim).unwrap();

		let _ = PoeModule::create_claim(Origin::signed(1), bounded_claim.clone()); // 先创建存证

		let receiver = 2;
		assert_ok!(PoeModule::transfer_claim(Origin::signed(1), receiver, bounded_claim.clone())); // 断言：转移存证
	});
}

// claim_bounded_error 存证超限错误的用例测试
#[test]
fn claim_bounded_error() {
	new_test_ext().execute_with(|| {
		let claim = vec![0u8; 1024];  // 设置一个超长度限制的大数
		let bounded_claim = BoundedVec::<u8, <Test as Config>::MaxBytesInHash>::try_from(claim).expect("claim bounded overflow error"); // 溢出返回错误

		assert_ok!(PoeModule::create_claim(Origin::signed(1), bounded_claim.clone()));
	});
}