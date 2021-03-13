use crate::util::*;

pub const ITER_TIMES: usize = 10000;

fn many_static_plus() {
    for _ in 0..ITER_TIMES {
        static_plus(1, 1); 
    }
}

fn many_dynamic_plus() {
    for _ in 0..ITER_TIMES {
        dynamic_plus(Any::Integer(1), Any::Integer(1));
    }
}

pub fn bench_many_static_plus() -> u128 {
    measure_time(many_static_plus)
}

pub fn bench_many_dynamic_plus() -> u128 {
    measure_time(many_dynamic_plus)
}
