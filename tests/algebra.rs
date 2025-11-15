use tn_math::algebra::{sum, subtract, multiplication, divide, power, power_tower};

#[cfg(feature = "algebra")]
#[test]
fn test_sum() {
    assert_eq!(sum(2.0, 3.0), 5.0);
    assert_eq!(sum(-1.0, 1.0), 0.0);
    assert_eq!(sum(10.0, -5.0), 5.0);
}

#[cfg(feature = "algebra")]
#[test]
fn test_subtraction() {
    assert_eq!(subtract(5.0, 2.0), 3.0);
    assert_eq!(subtract(-1.0, 1.0), -2.0);
    assert_eq!(subtract(10.0, 5.0), 5.0);
}

#[cfg(feature = "algebra")]
#[test]
fn test_multiplication() {
    assert_eq!(multiplication(10.0, 5.0), 50.0);
    assert_eq!(multiplication(-1.0, 10.0), -10.0);
    assert_eq!(multiplication(2.0, 3.0), 6.0);
}

#[cfg(feature = "algebra")]
#[test]
fn test_division() {
    assert_eq!(divide(10.0, 5.0), 2.0);
    assert_eq!(divide(-1.0, 1.0), -1.0);
    assert_eq!(divide(20.0, 4.0), 5.0);
}

#[cfg(feature = "algebra")]
#[test]
fn test_summation_macro() {
    use tn_math::summation;

    assert_eq!(summation!(1.0, 2.0, -5.0, -7.0, 12.0), 3.0);
    assert_eq!(summation!(10.0, -3.0, 2.0, -1.0), 8.0);
    assert_eq!(summation!(5.0, 5.0, 5.0), 15.0);
}

#[cfg(feature = "algebra")]
#[test]
fn test_subtraction_macro() {
    use tn_math::difference;

    assert_eq!(difference!(10.0, 5.0, -5.0, 7.0, -1.0), 4.0);
    assert_eq!(difference!(20.0, 10.0, 5.0, -3.0), 8.0);
    assert_eq!(difference!(15.0, 10.0, 5.0), 0.0);
}

#[cfg(feature = "algebra")]
#[test]
fn test_multiplication_macro() {
    use tn_math::product;

    assert_eq!(product!(10.0, 50.0, 4.0), 2000.0);
    assert_eq!(product!(2.0, 3.0, 4.0), 24.0);
    assert_eq!(product!(5.0, 5.0, 5.0), 125.0);
}

#[cfg(feature = "algebra")]
#[test]
fn test_division_macro() {
    use tn_math::division;

    assert_eq!(division!(1000.0, 50.0, 20.0), 1.0);
    assert_eq!(division!(120.0, 4.0, 3.0), 10.0);
    assert_eq!(division!(240.0, 4.0, 6.0), 10.0);
}

#[cfg(feature = "algebra")]
#[test]
fn test_power() {
    assert_eq!(power(2.0, 3.0), 8.0);
    assert_eq!(power(2.0, -3.0), 0.125);
    assert_eq!(power(-2.0, 3.0), -8.0);
}

#[cfg(feature = "algebra")]
#[test]
fn test_power_macro() {
    use tn_math::power;
    assert_eq!(power!(2.0, 3.0), 8.0);
    assert_eq!(power!(2.0, 3.0, 2.0), 64.0);
    assert_eq!(power!(2.0, 3.0, 2.0, 2.0), 4096.0);
}

#[cfg(feature = "algebra")]
#[test]
fn test_power_tower() {
    assert_eq!(power_tower(2.0, 3), 16.0);
    assert_eq!(power_tower(2.0, 4), 65536.0);
    assert_eq!(power_tower(3.0, 2), 27.0);
}

//#[test]
//fn test_power_tower_tower() {
//    assert_eq!(power_tower_tower(2, 3), 65536);
//    assert_eq!(power_tower_tower(3, 2), 7625597484987);
//    assert_eq!(power_tower_tower(2, 2), 16);
// }