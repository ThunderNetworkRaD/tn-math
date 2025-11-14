#[test]
fn sum_number_structs() {
    use tn_math::numbers::number_trait::NumberTrait;

    let num1 = 143_usize.to_number();
    let num2 = 4560_usize.to_number();

    let result = num1.sum(&num2);
    let expected = ((143 + 4560) as usize).to_number();

    assert_eq!(result.integer_part, expected.integer_part);
    assert_eq!(result.rational_part, expected.rational_part);
    assert_eq!(result.sign, expected.sign);
}

#[test]
fn sum_number_structs_with_rationals() {
    use tn_math::numbers::{number::Number, number::Sign};

    let num1 = Number {
        integer_part: vec![3, 4, 1],
        rational_part: vec![5, 7],
        sign: Sign::Positive,
    };
    let num2 = Number {
        integer_part: vec![6, 5, 4],
        rational_part: vec![5, 3],
        sign: Sign::Positive,
    };

    let result = num1.sum(&num2);
    let expected = Number {
        integer_part: vec![0, 0, 6],
        rational_part: vec![1, 0],
        sign: Sign::Positive,
    };

    assert_eq!(result.integer_part, expected.integer_part);
    assert_eq!(result.rational_part, expected.rational_part);
    assert_eq!(result.sign, expected.sign);
}

#[test]
fn sum_number_structs_with_different_signs_positive() {
    use tn_math::numbers::{number::Number, number::Sign};

    let num1 = Number {
        integer_part: vec![3, 4, 1],
        rational_part: vec![5, 7],
        sign: Sign::Negative,
    };
    let num2 = Number {
        integer_part: vec![6, 5, 4],
        rational_part: vec![5, 3],
        sign: Sign::Positive,
    };

    let result = num1.sum(&num2);
    let expected = Number {
        integer_part: vec![2, 1, 3],
        rational_part: vec![9, 6],
        sign: Sign::Positive,
    };

    assert_eq!(result.integer_part, expected.integer_part);
    assert_eq!(result.rational_part, expected.rational_part);
    assert_eq!(result.sign, expected.sign);
}

#[test]
fn sum_number_structs_with_different_signs_negative() {
    use tn_math::numbers::{number::Number, number::Sign};

    let num1 = Number {
        integer_part: vec![3, 4, 1],
        rational_part: vec![5, 7],
        sign: Sign::Positive,
    };
    let num2 = Number {
        integer_part: vec![6, 5, 4],
        rational_part: vec![5, 3],
        sign: Sign::Negative,
    };

    let result = num1.sum(&num2);
    let expected = Number {
        integer_part: vec![2, 1, 3],
        rational_part: vec![9, 6],
        sign: Sign::Negative,
    };

    assert_eq!(result.integer_part, expected.integer_part);
    assert_eq!(result.rational_part, expected.rational_part);
    assert_eq!(result.sign, expected.sign);
}

#[test]
fn multiply_number_structs() {
    use tn_math::numbers::number_trait::NumberTrait;

    let num1 = 143_usize.to_number();
    let num2 = 4560_usize.to_number();

    let result = num1.multiply(&num2);
    let expected = ((143 * 4560) as usize).to_number();

    println!("Result: {:?}", result);
    println!("Expected: {:?}", expected);

    assert_eq!(result.integer_part, expected.integer_part);
    assert_eq!(result.rational_part, expected.rational_part);
    assert_eq!(result.sign, expected.sign);
}