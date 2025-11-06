#[test]
fn sum_number_structs() {
    use tn_math::numbers::number_trait::NumberTrait;

    let num1 = 123_i32.to_number();
    let num2 = 4560_i32.to_number();

    let result = num1.sum(&num2);
    let expected = ((123 + 4560) as i32).to_number();

    assert_eq!(result.integer_part, expected.integer_part);
    assert_eq!(result.rational_part, expected.rational_part);
    assert_eq!(result.sign, expected.sign);
}