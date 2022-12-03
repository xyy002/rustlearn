pub struct Solution {}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a1 = i128::from_str_radix(&a,2).unwrap();
        let b1 = i128::from_str_radix(&b,2).unwrap();
        format!("{:b}",(b1+a1)).to_string()
    }
}

fn main() {
    println!("Hello, world!");
}
