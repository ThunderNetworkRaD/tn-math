pub fn subtract(x: f64, y: f64) -> f64 {
    x - y
}

#[macro_export]
macro_rules! difference {
    ($first:expr, $($x:expr),*) => {
        {
            let mut total: f64 = $first;
            $(
                total = subtract(total, $x);
            )*
            total
        }
    };
}