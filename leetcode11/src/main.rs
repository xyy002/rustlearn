pub struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut nums_cnt = 0;
        //遍历匹配不符合的值
        for i in 0..nums.len() {
            if nums[i]!=val{
                //重新放进数组
                nums[nums_cnt]=nums[i];
                nums_cnt+=1;
            }
        }
        nums_cnt as i32
    }
}



fn main() {
    println!("Hello, world!");
}
