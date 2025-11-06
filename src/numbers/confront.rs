#[macro_export]
macro_rules! min {
    ($first:expr $(, $rest:expr)* $(,)?) => {{
        let mut min = $first;
        $(
            if $rest < min {
                min = $rest;
            }
        )*
        min
    }};
}

#[macro_export]
macro_rules! max {
    ($first:expr $(, $rest:expr)* $(,)?) => {{
        let mut max = $first;
        $(
            if $rest > max {
                max = $rest;
            }
        )*
        max
    }};
}