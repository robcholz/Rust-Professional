/*
    Find Intersection of Two Arrays
    Given two arrays, find the intersection of the arrays and return the elements of the intersection (without duplicates).
    The result should not contain any duplicate elements.

    You need to implement the function `intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32>`.
    The function should return a vector containing all the elements that are in both arrays.

    Hint: You can solve this problem using sorting, hash sets, or the two-pointer technique.
*/

use std::fmt::{self, Display, Formatter};

pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut nums1 = nums1;
    let mut nums2 = nums2;
    let mut result = vec![];
    nums1.sort();
    nums1.dedup();
    nums2.sort();
    nums2.dedup();
    let mut i1: usize = 0;
    let mut i2: usize = 0;
    while i1 < nums1.len() && i2 < nums2.len() {
        let element1 = nums1[i1];
        let element2 = nums2[i2];
        if element1 == element2 {
            result.push(nums1[i1]);
        }
        if element1 > element2 {
            i2 += 1;
        } else {
            i1 += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection_1() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        let result = intersection(nums1, nums2);
        println!("Intersection: {:?}", result);
        assert_eq!(result, vec![2]);
    }

    #[test]
    fn test_intersection_2() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        let result = intersection(nums1, nums2);
        println!("Intersection: {:?}", result);
        assert_eq!(result, vec![4, 9]);
    }

    #[test]
    fn test_intersection_3() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![4, 5, 6];
        let result = intersection(nums1, nums2);
        println!("Intersection: {:?}", result);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_intersection_4() {
        let nums1 = vec![1, 1, 1];
        let nums2 = vec![1, 1, 1];
        let result = intersection(nums1, nums2);
        println!("Intersection: {:?}", result);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_intersection_5() {
        let nums1 = vec![10, 20, 30];
        let nums2 = vec![30, 40, 50];
        let result = intersection(nums1, nums2);
        println!("Intersection: {:?}", result);
        assert_eq!(result, vec![30]);
    }
}
