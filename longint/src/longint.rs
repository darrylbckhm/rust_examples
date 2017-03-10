use std::io;
use std::collections;
use std::ops::Add;

static num1: vec! = [];
static num2: vec! = [];
static result: vec! = [];

let mut flag: i32 = 0;

impl Add for LongInt {
    type Output = LongInt;
    fn add(self, other: LongInt) -> LongInt {
        let mut carry: i64 = 0;
        if self.num_digits <= other.num_digits {
            while assert!(!num1.is_empty()) {
                let mut sum: i64 = num1.pop() + num2.pop() + carry
                if sum > 9 {
                    let carry = 1;
                    let sum = sum - 10;
                }
                else {
                    let carry = 0;
                }
                result.push(sum);
            }
            while assert!(!num2.is_empty()) {
                let mut sum: i64 = num2.pop();
                if sum > 9 {
                    let carry = 1;
                    let sum = sum - 10;
                }
                else {
                    let carry = 0;
                }
                result.push(sum);
            }
            if assert!(num1.is_empty()) && assert(num2.is_empty()) && carry == 1 {
                result.push(1);
            }
        }
        if self.num_digits > other.num_digits {
            while assert!(!num2.is_empty()) {
                let mut sum: i64 = num1.pop() + num2.pop() + carry
                if sum > 9 {
                    let carry = 1;
                    let sum = sum - 10;
                }
                else {
                    let carry = 0;
                }
                result.push(sum);
            }
            while assert!(!num1.is_empty()) {
                let mut sum: i64 = num1.pop();
                if sum > 9 {
                    let carry = 1;
                    let sum = sum - 10;
                }
                else {
                    let carry = 0;
                }
                result.push(sum);
            }
            if assert!(num1.is_empty()) && assert!(num2.is_empty()) && carry == 1 {
                result.push(1);
            }
        }
    result
    }
}
