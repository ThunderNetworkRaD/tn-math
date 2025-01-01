pub fn multiplication(x: f64, y: f64) -> f64 {
    x * y
}

#[macro_export]
macro_rules! product {
    ($($x:expr),*) => {
        {
            let mut total: f64 = 1.0;
            $(
                total = multiplication($x, total);
            )*
            total
        }
    };
}