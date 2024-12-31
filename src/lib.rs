macro_rules! sum {
    ($($x:expr),*) => {
        {
            let mut total: isize = 0;
            $(
                total += $x;
            )*
            total
        }
    };
}

macro_rules! mul {
    ($($x:expr),*) => {
        {
            let mut total: isize = 0;
            $(
                total *= $x;
            )*
            total
        }
    };
}

macro_rules! div {
    ($($x:expr),*) => {
        {
            let mut total: isize = 0;
            $(
                total /= $x;
            )*
            total
        }
    };
}

pub fn sum(x: isize, y: isize) -> isize {
    x + y
}

pub fn diff(x: isize, y: isize) -> isize {
    x - y
}

pub fn mul(x: isize, y: isize) -> isize {
    x * y
}

pub fn div(x: isize, y: isize) -> isize {
    x / y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum() {
        let result = sum!(3, -1, 2);
        assert_eq!(result, 4);
    }
}
