pub  struct  Solution{}
impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
       let mut n =n;
        let mut result = String::new();
        while n > 0 {
           let mut r = n % 26;
           if r == 0 {
                    r=26;
               n-=1;
            }
            result=String::from_utf8(vec![r as u8 + 64]).unwrap()+&result;
            n/=26;
        }
        result
    }
}
fn main() {
    println!("Hello, world!");
}
