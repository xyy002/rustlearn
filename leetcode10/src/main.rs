pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.dedup(); //api ����
        nums.len() as i32
    }
}


fn main() {
    println!("Hello, world!");
}
