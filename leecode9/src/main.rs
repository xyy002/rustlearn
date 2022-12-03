#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
pub struct Solution{}
impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1,list2) {
            (None,None)=>None,
            (None,r)=>r,
            (l,None)=>l,
            (Some(mut l),Some(mut r))=>{
                if l.val<=r.val{
                    l.next=Self::merge_two_lists(l.next,Some(r));
                    Some(l)
                }else {
                    r.next=Self::merge_two_lists(Some(l),r.next);
                    Some(r)
                }
            }
        }
    }
}


fn main() {
    println!("Hello, world!");
}
