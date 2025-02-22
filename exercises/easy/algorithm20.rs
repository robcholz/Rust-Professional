/*
    Sum of Two Integers
    Given two integers, calculate their sum without using the `+` operator. 
    You need to implement the function `get_sum(a: i32, b: i32) -> i32`.
    The function should return the sum of the two integers `a` and `b`.

    Hint: You can solve this problem using bitwise operations.
*/

use std::fmt::Display;

pub fn get_sum(a: i32, b: i32) -> i32 {
    let single_bit_adder = |a: u8, b: u8, carry: u8| {
        let output_carry: u8 = ((!a) & b & carry) | (a & (!b) & carry) | (a & b & (!carry) | (a & b & carry));
        let mut output: u8 = ((!a) & (!b) & carry) | (a & (!b) & (!carry)) | ((!a) & b & (!carry)) | (a & b & carry);
        output = output & 1;
        return (output, output_carry);
    };
    let read_n_bit = |a: i32, index: u8| -> u8 { ((a >> index) & 1) as u8 };
    let add_to_result = |result: &mut i32, value: u8, index: u8| {
        *result = *result | ((value as i32) << index)
    };
    let mut result: i32 = 0;
    let mut carry: u8 = 0;
    for index in 0..32 {
        let a_n = read_n_bit(a, index);
        let b_n = read_n_bit(b, index);
        let (output, o_carry) = single_bit_adder(a_n, b_n, carry);
        carry = o_carry;
        add_to_result(&mut result, output, index);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_1() {
        let result = get_sum(1, 2);
        println!("Sum of 1 and 2: {}", result);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_sum_2() {
        let result = get_sum(-1, 1);
        println!("Sum of -1 and 1: {}", result);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_sum_3() {
        let result = get_sum(100, 200);
        println!("Sum of 100 and 200: {}", result);
        assert_eq!(result, 300);
    }

    #[test]
    fn test_sum_4() {
        let result = get_sum(-50, -50);
        println!("Sum of -50 and -50: {}", result);
        assert_eq!(result, -100);
    }

    #[test]
    fn test_sum_5() {
        let result = get_sum(0, 0);
        println!("Sum of 0 and 0: {}", result);
        assert_eq!(result, 0);
    }
}
