
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

pub struct Solution {}
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut prev = &mut head;
        while let Some(node) = prev {
            while let Some(next) = node.next.as_mut() {
                if next.val == node.val {
                    node.next = next.next.take();
                } else {
                    break;
                }
            }
            prev = &mut node.next;
        }
        head

    }
}

fn main() {
    println!("Hello, world!");
}
