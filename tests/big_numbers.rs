use tn_math::big_numbers::big_decimal::BigDecimal;

#[test]
fn test_conversions() {
    assert_eq!(
        BigDecimal::from_str("123.456").unwrap(),
        BigDecimal{integer_part: Vec::from([1, 2, 3]), fractional_part: Vec::from([4, 5, 6])}
    );
    assert_eq!(
        BigDecimal{integer_part: Vec::from([1, 2, 3]), fractional_part: Vec::from([4, 5, 6])}.to_string(),
        "123.456".to_string()
    )
}

#[test]
fn test_sum() {
    let number1 = BigDecimal::from_str("123.456").unwrap();
    let number2 = BigDecimal::from_str("789.012").unwrap();
    assert_eq!(
        number1.sum(&number2),
        BigDecimal{integer_part: Vec::from([9, 1, 2]), fractional_part: Vec::from([4, 6, 8])}
    );

    let number1 = BigDecimal::from_str("123456789").unwrap();
    let number2 = BigDecimal::from_str("987654321").unwrap();
    assert_eq!(
        number1.sum(&number2),
        BigDecimal{integer_part: Vec::from([1, 1, 1, 1, 1, 1, 1, 1, 1, 0]), fractional_part: Vec::new()}
    );

    let number1 = BigDecimal::from_str("11.9999").unwrap();
    let number2 = BigDecimal::from_str("11.1111").unwrap();
    assert_eq!(
        number1.sum(&number2),
        BigDecimal{integer_part: Vec::from([2, 3]), fractional_part: Vec::from([1, 1, 1, 0])}
    );
}

#[test]
fn test_sanitize() {
    let number1 = BigDecimal::from_str("001000.00110").unwrap();
    assert_eq!(
        number1.sanitize(),
        BigDecimal{integer_part: Vec::from([1, 0, 0, 0]), fractional_part: Vec::from([0, 0, 1, 1])}
    );
}