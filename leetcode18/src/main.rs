pub struct Solution {};

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        for i in 0..n{
            nums1[(m+i) as usize]=nums2[i as usize];
        }
        nums1.sort();
    }
}

fn main() {
    println!("Hello, world!");
}
