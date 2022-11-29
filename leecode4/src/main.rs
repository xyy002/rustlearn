use std::f32::consts::E;

pub struct Solution{

}

impl Solution {
    pub fn subarray_lcm(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans :i32=0;
        for i in 0..nums.len(){
            let mut x = &nums[i];
            for n in 0..i{
                // let mut y = &nums[n];
                if *(&lcm(*x, *(&nums[n])))==k{
                    ans+=1;
                }
                else {
                    break;
                }
            }
        }
        return ans;
    }
   
}
pub fn gcd(a:i32,b:i32)->i32{
    let mut temp:i32 =a%b;
    if temp>0{
        return gcd(b, temp);
    }
    else {
        return b;
    }
    
}
pub fn lcm(a:i32,b:i32)-> i32{
    a * b / gcd(a, b)
}

fn main() {
    println!("Hello, world!");
}
