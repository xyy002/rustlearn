pub struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let v: Vec<_> = s.chars().collect();
        let mut index: usize = 0;
        let mut rt: i32 = 0;
        let mut pre: i32 = 0;
        while index<v.len() {
            let mut cont :i32 =0;
            match v[index] {
                'I'=>cont=1,
                'V'=>cont=5,
                'X'=>cont=10,
                'L'=>cont=50,                
                'C'=>cont=100,
                'D'=>cont=500,
                'M'=>cont=1000,
                _=>cont=0,
            }
            rt+=cont;
            if cont>pre&&pre!=0{
                rt -= 2*pre;
            }
            pre=cont;
            index+=1;
        }
       rt
    }
}

fn main() {
    println!("Hello, world!");
}
