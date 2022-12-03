pub struct Solution {}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        //判断字符串是否为空
       if s.trim().len()!=0{
        //寻找间隔、最后一个元素
        s.split_ascii_whitespace().last().unwrap().len() as i32
       }
       else {
           return 0;
       }
    }
}

fn main() {
    println!("Hello, world!");
}
