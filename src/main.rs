mod util;
mod benches;

use benches::{bench_many_dynamic_plus, bench_many_static_plus, ITER_TIMES};

fn main() {
    let static_plus_time = bench_many_static_plus();
    let dynamic_plus_time = bench_many_dynamic_plus();

    println!(
        "Time for adding two numbers {} times with static typing (in ms): {}",
        ITER_TIMES,
        static_plus_time
    );

    println!(
        "Time for adding two numbers {} times with dynamic typing (in ms): {}",
        ITER_TIMES,
        dynamic_plus_time
    );

    println!(
        "Measured difference (in ms): {}",
        dynamic_plus_time - static_plus_time
    );
}