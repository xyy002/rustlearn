pub  struct Solution{}
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |acc, &x| acc ^ x)
        // return 0;
    }
}
fn main() {
    println!("Hello, world!");
}
