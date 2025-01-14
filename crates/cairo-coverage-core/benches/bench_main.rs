mod benchmarks;

use criterion::criterion_main;

criterion_main! {
    benchmarks::starknet_staking::benches
}
