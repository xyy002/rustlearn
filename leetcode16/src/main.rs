pub struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut sum :i64 = 1;
        for i in 1..n{
            let mut t :i64 = 1;
            for j in 0..i {
                t*=(n-i-j) as i64;
                t/=(j+1) as i64;
            }
            sum+=t;
        }
        sum as i32
    }
}


fn main() {
    println!("Hello, world!");
}
