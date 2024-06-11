#![cfg(feature = "runtime-benchmarks")]

use super::*;

#[allow(unused)]
use crate::Pallet as ValidatorSet;
use frame_benchmarking::v2::*;
use frame_support::traits::EnsureOrigin;
use frame_system::RawOrigin;

const SEED: u32 = 0;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn add_validator() {
		let validator: T::ValidatorId = account("validator", 0, SEED);

		#[extrinsic_call]
		add_validator(RawOrigin::Root, validator);
	}

	#[benchmark]
	fn remove_validator() {
		let validator: T::ValidatorId = account("validator", 0, SEED);

		#[extrinsic_call]
		remove_validator(RawOrigin::Root, validator);
	}

	impl_benchmark_test_suite!(ValidatorSet, crate::mock::new_test_ext(), crate::mock::Test);
}
