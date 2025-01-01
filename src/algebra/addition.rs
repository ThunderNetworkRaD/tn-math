pub fn sum(x: f64, y: f64) -> f64 {
    x + y
}

#[macro_export]
macro_rules! sum {
    ($($x:expr),*) => {
        {
            let mut total: f64 = 0.0;
            $(
                total = sum(total, $x);
            )*
            total
        }
    };
}