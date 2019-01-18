
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;
impl Solution {
    pub fn merge_trees(t1: Option<Rc<RefCell<TreeNode>>>, t2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match (t1, t2) {
            (None, None) => return None,
            (Some(t1_rc), None) => return Some(t1_rc.clone()),
            (None, Some(t2_rc)) => return Some(t2_rc.clone()),
            (Some(t1_rc), Some(t2_rc)) => {
                let total_val = t1_rc.borrow().val + t2_rc.borrow().val;
                let left_tree = Solution::merge_trees(t1_rc.borrow().left.clone(), t2_rc.borrow().left.clone());
                let right_tree = Solution::merge_trees(t1_rc.borrow().right.clone(), t2_rc.borrow().right.clone());
                return Some(Rc::new(RefCell::new(TreeNode {
                    val: total_val,
                    left: left_tree,
                    right: right_tree
                })));
            }
        }
        #[warn(unreachable_code)]
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use super::TreeNode;
    use std::rc::Rc;
    use std::cell::RefCell;


    #[test]
    fn test_none_merge_none() {
        let result = Solution::merge_trees(None, None);
        assert_eq!(result.is_none(), true);
    }

    #[test]
    fn test_some_merge_none() {
        let left_a = Rc::new(RefCell::new(TreeNode::new(10)));
        let a = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(left_a),
            right: None
        })));

        let result = Solution::merge_trees(a, None);
        assert_eq!(result.is_some(), true);
        assert_eq!(Rc::strong_count(result.as_ref().unwrap()), 1);
        assert_eq!(Rc::weak_count(result.as_ref().unwrap()), 0);
        assert_eq!(result.as_ref().unwrap().borrow().val, 5);
        assert_eq!(result.as_ref().unwrap().borrow().left.is_some(), true);
        assert_eq!(result.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().val, 10);
        assert_eq!(result.as_ref().unwrap().borrow().right.is_none(), true);
    }

    #[test]
    fn test_none_merge_some() {
        let left_a = Rc::new(RefCell::new(TreeNode::new(10)));
        let a = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(left_a),
            right: None
        })));

        let result = Solution::merge_trees(a, None);
        assert_eq!(result.is_some(), true);
        assert_eq!(Rc::strong_count(result.as_ref().unwrap()), 1);
        assert_eq!(Rc::weak_count(result.as_ref().unwrap()), 0);
        assert_eq!(result.as_ref().unwrap().borrow().val, 5);
        assert_eq!(result.as_ref().unwrap().borrow().left.is_some(), true);
        assert_eq!(result.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().val, 10);
        assert_eq!(result.as_ref().unwrap().borrow().right.is_none(), true);
    }

    #[test]
    fn test_some_merge_some() {
        let left_a = Rc::new(RefCell::new(TreeNode::new(20)));
        let right_a = Rc::new(RefCell::new(TreeNode::new(10)));
        let a = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(left_a),
            right: Some(right_a),
        })));

        let left_b = Rc::new(RefCell::new(TreeNode::new(200)));
        let right_b = Rc::new(RefCell::new(TreeNode::new(100)));
        let b = Some(Rc::new(RefCell::new(TreeNode{
            val: 1000,
            left: Some(left_b),
            right: Some(right_b)
        })));

        let result = Solution::merge_trees(a, b);
        assert_eq!(result.is_some(), true);
        assert_eq!(Rc::strong_count(result.as_ref().unwrap()), 1);
        assert_eq!(Rc::weak_count(result.as_ref().unwrap()), 0);
        assert_eq!(result.as_ref().unwrap().borrow().val, 1005);

        let result_left = result.as_ref().unwrap().borrow().left.clone();
        assert_eq!(result_left.is_some(), true);
        assert_eq!(Rc::strong_count(result_left.as_ref().unwrap()), 2);
        assert_eq!(Rc::weak_count(result_left.as_ref().unwrap()), 0);
        assert_eq!(result_left.as_ref().unwrap().borrow().val, 220);
        assert_eq!(result_left.as_ref().unwrap().borrow().left.is_none(), true);
        assert_eq!(result_left.as_ref().unwrap().borrow().right.is_none(), true);

        let result_right = result.as_ref().unwrap().borrow().right.clone();
        assert_eq!(result_right.is_some(), true);
        assert_eq!(Rc::strong_count(result_right.as_ref().unwrap()), 2);
        assert_eq!(Rc::weak_count(result_right.as_ref().unwrap()), 0);
        assert_eq!(result_right.as_ref().unwrap().borrow().val, 110);
        assert_eq!(result_right.as_ref().unwrap().borrow().left.is_none(), true);
        assert_eq!(result_right.as_ref().unwrap().borrow().right.is_none(), true);
    }
}