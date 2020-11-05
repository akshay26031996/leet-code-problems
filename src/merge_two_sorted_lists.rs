// https://leetcode.com/problems/merge-two-sorted-lists/

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
pub fn merge_two_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    if l1.is_none() {
        return l2;
    }
    if l2.is_none() {
        return l1;
    }
    let mut res = None;
    {
        let mut node1 = &l1;
        let mut node2 = &l2;
        let mut node3 = &mut res;
        loop {
            if let (Some(link1), Some(link2)) = (node1, node2) {
                if link1.val > link2.val {
                    *node3 = Some(Box::new(ListNode::new(link2.val)));
                    node2 = &link2.next;
                } else {
                    *node3 = Some(Box::new(ListNode::new(link1.val)));
                    node1 = &link1.next;
                }
                match node3 {
                    None => {
                        panic!("Node 3 should not be none");
                    }
                    Some(link) => {
                        node3 = &mut link.next;
                    }
                }
            } else if let Some(link1) = node1 {
                *node3 = Some(Box::new(ListNode::new(link1.val)));
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
                *node3 = Some(Box::new(ListNode::new(link2.val)));
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
    fn merge_two_sorted_lists_basic() {
        assert_eq!(
            merge_two_lists(generate_list(vec![1, 2, 4]), generate_list(vec![1, 3, 4])),
            generate_list(vec![1, 1, 2, 3, 4, 4])
        );
        assert_eq!(
            merge_two_lists(generate_list(vec![]), generate_list(vec![])),
            generate_list(vec![])
        );
        assert_eq!(
            merge_two_lists(generate_list(vec![]), generate_list(vec![1])),
            generate_list(vec![1])
        );
    }
}
