pub fn power(x: f64, y: f64) -> f64 {
    x.powf(y)
}

#[macro_export]
macro_rules! power {
    ($base:expr, $($x:expr),*) => {
        {
            let mut total: f64 = $base;
            $(
                total = power(total, $x);
            )*
            total
        }
    };
}

pub fn power_tower(a: f64, b: u64) -> f64 {
    let mut result = a;
    for _ in 1..b {
        result = power(a, result);
    }
    result
}

// pub fn power_tower_tower(a: u64, b: u64) -> u64 {
//    let mut result = a;
//    for _ in 1..b {
//        result = power_tower(a as f64, result) as u64;
//    }
//    result
// }