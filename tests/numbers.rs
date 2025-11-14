use tn_math::numbers::number::Number;

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
fn multiply_numbers_structs() {
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

#[test]
fn multiply_numbers_with_rationals_structs() {
    let num1 = Number {
        integer_part: vec![3, 4, 1],
        rational_part: vec![5, 7],
        sign: tn_math::numbers::number::Sign::Negative,
    };
    let num2 = Number {
        integer_part: vec![6, 5, 4],
        rational_part: vec![8, 3],
        sign: tn_math::numbers::number::Sign::Positive,
    };

    let result = num1.multiply(&num2);
    let expected = Number {
        integer_part: vec![7,8,5,5,6],
        rational_part: vec![0,8,3,1],
        sign: tn_math::numbers::number::Sign::Negative,
    };

    println!("Result: {:?}", result);
    println!("Expected: {:?}", expected);

    assert_eq!(result.integer_part, expected.integer_part);
    assert_eq!(result.rational_part, expected.rational_part);
    assert_eq!(result.sign, expected.sign);
}

#[test]
fn multiply_big_numbers_structs() {
    let factor_a_integer_part = [
        9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9
    ];

    let factor_b_integer_part = [
        9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9
    ];

    let result_integer_part = [
        1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,8,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9
    ];

    let num1 = Number {
        integer_part: factor_a_integer_part.to_vec(),
        rational_part: vec![],
        sign: tn_math::numbers::number::Sign::Positive,
    };
    let num2 = Number {
        integer_part: factor_b_integer_part.to_vec(),
        rational_part: vec![],
        sign: tn_math::numbers::number::Sign::Positive,
    };

    let result = num1.multiply(&num2);
    let expected = Number {
        integer_part: result_integer_part.to_vec(),
        rational_part: vec![],
        sign: tn_math::numbers::number::Sign::Positive,
    };

    println!("Result: {:?}", result);
    println!("Expected: {:?}", expected);

    assert_eq!(result.integer_part, expected.integer_part);
    assert_eq!(result.rational_part, expected.rational_part);
    assert_eq!(result.sign, expected.sign);
}