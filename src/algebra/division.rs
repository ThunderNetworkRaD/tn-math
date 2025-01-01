pub fn divide(x: f64, y: f64) -> f64 {
    x / y
}

#[macro_export]
macro_rules! division {
    ($first:expr, $($x:expr),*) => {
        {
            let mut total: f64 = $first;
            $(
                total = divide(total, $x);
            )*
            total
        }
    };
}