// https://leetcode.com/problems/add-two-numbers/

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// You are given two non-empty linked lists
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut res = None;
    let mut carry = 0;
    let mut node1 = &l1;
    let mut node2 = &l2;
    let mut node3 = &mut res;
    loop {
        if let (Some(link1), Some(link2)) = (node1, node2) {
            let mut sum = link1.val + link2.val + carry;
            carry = sum / 10;
            sum = sum % 10;
            *node3 = Some(Box::new(ListNode::new(sum)));
            node1 = &link1.next;
            node2 = &link2.next;
            match node3 {
                None => {
                    panic!("Node 3 should not be none");
                }
                Some(link) => {
                    node3 = &mut link.next;
                }
            }
        } else if let Some(link1) = node1 {
            let mut sum = link1.val + carry;
            carry = sum / 10;
            sum = sum % 10;
            *node3 = Some(Box::new(ListNode::new(sum)));
            node1 = &link1.next;
            match node3 {
                None => {
                    panic!("Node 3 should not be none");
                }
                Some(link) => {
                    node3 = &mut link.next;
                }
            }
        } else if let Some(link2) = node2 {
            let mut sum = link2.val + carry;
            carry = sum / 10;
            sum = sum % 10;
            *node3 = Some(Box::new(ListNode::new(sum)));
            node2 = &link2.next;
            match node3 {
                None => {
                    panic!("Node 3 should not be none");
                }
                Some(link) => {
                    node3 = &mut link.next;
                }
            }
        } else {
            break;
        }
    }
    if carry != 0 {
        *node3 = Some(Box::new(ListNode::new(carry)));
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_list(values: Vec<i32>) -> Option<Box<ListNode>> {
        let mut res = None;
        if values.is_empty() {
            return res;
        }
        let mut node = &mut res;
        for val in values {
            *node = Some(Box::new(ListNode::new(val)));
            match node {
                None => {
                    panic!("Node 3 should not be none");
                }
                Some(link) => {
                    node = &mut link.next;
                }
            }
        }
        res
    }

    #[test]
    fn add_two_numbers_basic() {
        assert_eq!(
            add_two_numbers(generate_list(vec![2, 4, 3]), generate_list(vec![5, 6, 4])),
            generate_list(vec![7, 0, 8])
        );
        assert_eq!(
            add_two_numbers(generate_list(vec![0]), generate_list(vec![0])),
            generate_list(vec![0])
        );
        assert_eq!(
            add_two_numbers(
                generate_list(vec![9, 9, 9, 9, 9, 9, 9]),
                generate_list(vec![9, 9, 9, 9])
            ),
            generate_list(vec![8, 9, 9, 9, 0, 0, 0, 1])
        );
    }
}
