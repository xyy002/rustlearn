/*
 * @Author: 青杉衫 12385774@qq.com
 * @Date: 2022-11-30 21:25:27
 */

pub struct Solution{}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut v =vec!['0'];
        for c in s.chars() {
            match c {
                '('|'['|'{'=>{v.push(c)},
                ')' => {if v.pop().unwrap() != '(' {return false}},
                ']' => {if v.pop().unwrap() != '[' {return false}},
                '}' => {if v.pop().unwrap() != '{' {return false}},
                _ => {},
                }
        }
        v.len()==1
    }
}

fn main() {
    println!("Hello, world!");
}
