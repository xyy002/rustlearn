use std::f32::consts::E;

pub struct Solution{

}

impl Solution {
    pub fn subarray_lcm(nums: Vec<i32>, k: i32) -> i32 {
        let mut tot :i32=1;
        let mut ans:i32=0;
        for (i,value) in nums.iter().enumerate() {
           for _ in 0..i {
            tot=lcm(tot,*value);
            if tot == k{
                ans+=1;
            }
            if tot>k {
                break;
            }
           }
        }
        ans
    }
   
}
pub fn gcd(a:i32,b:i32)->i32{
    let temp =a%b;
    if temp!=0{
        return gcd(b, temp);
    }
    return b;

    
}
pub fn lcm(a:i32,b:i32)-> i32{
    a * b / gcd(a, b)
}

fn main() {
    println!("Hello, world!");
}
