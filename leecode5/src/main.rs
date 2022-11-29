pub struct Solution{
}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x<0 ||(x%10==0&&x!=0)
        {
          return false;
        }
        let (mut y,mut n)=(x,0);
        while true {
            if y<=n{break;}
            n=n*10+y%10;
            y/=10;
        }

         y==n||y==n/10
       

    }
}

fn main() {
    println!("Hello, world!");
}
