use std::time::{SystemTime, UNIX_EPOCH};

pub enum Any {
    Integer(i32),
    Float(f64),
    String(String),
    None
}

pub fn static_plus(x: i32, y: i32) -> i32 {
    x + y
}

pub fn dynamic_plus(x: Any, y: Any) -> Any {
    match (x, y) {
        (Any::Integer(xval), Any::Integer(yval)) => Any::Integer(xval + yval),
        (Any::Float(xval), Any::Float(yval)) => Any::Float(xval + yval),
        (Any::String(xval), Any::String(yval)) => Any::String(
            format!("{}{}", xval, yval)
        ),
        _ => panic!("Type coercion is not neccessary for the purpose of task")
    }
}

pub fn measure_time(func: fn()) -> u128 {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    (func)();
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

    end - start
}
