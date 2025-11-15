use numforge::numbers::{Number, Sign};

#[test]
fn sum_number_structs() {
    use numforge::numbers::number_trait::NumberTrait;

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
    use numforge::numbers::number_trait::NumberTrait;

    let num1 = 143_usize.to_number();
    let num2 = 4560_usize.to_number();

    let result = num1.multiply(&num2);
    let expected = ((143 * 4560) as usize).to_number();

    #[cfg(feature = "std")]
    {
        println!("Result: {:?}", result);
        println!("Expected: {:?}", expected);
    }

    assert_eq!(result.integer_part, expected.integer_part);
    assert_eq!(result.rational_part, expected.rational_part);
    assert_eq!(result.sign, expected.sign);
}

#[test]
fn multiply_numbers_with_rationals_structs() {
    let num1 = Number {
        integer_part: vec![3, 4, 1],
        rational_part: vec![5, 7],
        sign: Sign::Negative,
    };
    let num2 = Number {
        integer_part: vec![6, 5, 4],
        rational_part: vec![8, 3],
        sign: Sign::Positive,
    };

    let result = num1.multiply(&num2);
    let expected = Number {
        integer_part: vec![7,8,5,5,6],
        rational_part: vec![0,8,3,1],
        sign: Sign::Negative,
    };

    #[cfg(feature = "std")]
    {
        println!("Result: {:?}", result);
        println!("Expected: {:?}", expected);
    }


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
        sign: Sign::Positive,
    };
    let num2 = Number {
        integer_part: factor_b_integer_part.to_vec(),
        rational_part: vec![],
        sign: Sign::Positive,
    };

    let result = num1.multiply(&num2);
    let expected = Number {
        integer_part: result_integer_part.to_vec(),
        rational_part: vec![],
        sign: Sign::Positive,
    };

    #[cfg(feature = "std")]
    {
        println!("Result: {:?}", result);
        println!("Expected: {:?}", expected);
    }


    assert_eq!(result.integer_part, expected.integer_part);
    assert_eq!(result.rational_part, expected.rational_part);
    assert_eq!(result.sign, expected.sign);
}

#[test]
fn test_shifts() {
    let mut number = Number {
        integer_part: vec![3, 2, 1],
        rational_part: vec![4, 5, 6],
        sign: Sign::Positive,
    };

    number.shift_integer_to_rational();
    assert_eq!(number.integer_part, vec![2, 1]);
    assert_eq!(number.rational_part, vec![3, 4, 5, 6]);

    number.shift_rational_to_integer();
    assert_eq!(number.integer_part, vec![3, 2, 1]);
    assert_eq!(number.rational_part, vec![4, 5, 6]);

    number.shift_rationals_to_integers(2);
    assert_eq!(number.integer_part, vec![5,4,3,2,1]);
    assert_eq!(number.rational_part, vec![6]);

    number.shift_integers_to_rationals(3);
    assert_eq!(number.integer_part, vec![2,1]);
    assert_eq!(number.rational_part, vec![3,4,5,6]);

    number.shift_integers_to_rationals(3);
    assert_eq!(number.integer_part, vec![]);
    assert_eq!(number.rational_part, vec![0,1,2,3,4,5,6]);

    number.shift_rationals_to_integers(7);
    assert_eq!(number.integer_part, vec![6,5,4,3,2,1]);
    assert_eq!(number.rational_part, vec![]);
}