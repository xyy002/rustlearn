pub struct Solution {}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        //�ж��ַ����Ƿ�Ϊ��
       if s.trim().len()!=0{
        //Ѱ�Ҽ�������һ��Ԫ��
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
