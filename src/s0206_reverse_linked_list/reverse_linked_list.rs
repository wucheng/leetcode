use crate::leetcode_data_struct::data_struct::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut mut_head = head;
        let mut new_head: Option<Box<ListNode>> = None;
        loop {
            match mut_head {
                None => break,
                Some(mut origin_head) => {
                    let origin_head_next = origin_head.next;
                    origin_head.next = new_head;
                    new_head = Some(origin_head);
                    mut_head = origin_head_next;
                }
            }
        }
        new_head
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::leetcode_data_struct::data_struct::ListNode;

    #[test]
    pub fn test_none() {
        let result = Solution::reverse_list(None);
        assert_eq!(result.is_none(), true);
    }

    #[test]
    pub fn test_reverse_listnode() {
        let mut first_node = ListNode::new(1);
        let mut second_node = ListNode::new(2);
        let mut third_node = ListNode::new(3);

        second_node.next = Some(Box::new(third_node));
        first_node.next = Some(Box::new(second_node));

        let result = Solution::reverse_list(Some(Box::new(first_node)));
        assert_eq!(result.is_some(), true);
        let result_first_node = result.as_ref().unwrap();
        assert_eq!(result_first_node.val, 3);
        assert_eq!(result_first_node.next.is_some(), true);

        let result_second_node = result_first_node.next.as_ref().unwrap();
        assert_eq!(result_second_node.val, 2);
        assert_eq!(result_second_node.next.is_some(), true);

        let result_third_node = result_second_node.next.as_ref().unwrap();
        assert_eq!(result_third_node.val, 1);
        assert_eq!(result_third_node.next.is_none(), true);
    }
}