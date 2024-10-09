/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

//渐入佳境
use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        match self.root {
            None => self.root = Some(Box::new(TreeNode::new(value))),
            Some(ref mut node) => node.insert(value),
        }
        // match self.root {
        //     None => self.root = Some(Box::new(TreeNode::new(value))),
        //     Some(ref mut node) => {
        //         if node.value < value{
        //             match node.right {
        //                 None => node.right = Some(Box::new(TreeNode::new(value))),
        //                 Some(ref mut node_ptr) =>  node_ptr.insert(value),
        //             }
        //         }else if node.value > value{
        //             match node.left {
        //                 None => node.left = Some(Box::new(TreeNode::new(value))),
        //                 Some(ref mut node_ptr) => node_ptr.insert(value),
        //             }
        //         }
        //     }
        // }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        match self.root {
            None => false,
            Some(ref node) => node.search(value)
        }
        // match self.root {
        //     Some(ref mut node) => {
        //         while node != None {
        //             if value == node.value{ // 值相同
        //                 return true;
        //             }else if value <  node.value{ // 值更小
        //                 match node.left {
        //                     None => false,
        //                     Some(ref node_left) => node = &mut node_left,
        //                 }
        //             }else { // 值更大
        //                 match node.right {
        //                     None => false,
        //                     Some(ref node_right) => node = &mut node_right,
        //                 }
        //             }
        //         }
        //         return false;
        //     }
        //     None => false, 
        // }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        if self.value == value{
            return;
        }else if self.value < value{
            match self.right {
                Some(ref mut node_rigth) => node_rigth.insert(value),
                None => self.right = Some(Box::new(TreeNode::new(value))),
            }
        }else {
            match self.left {
                Some(ref mut node_left) => node_left.insert(value),
                None => self.left = Some(Box::new(TreeNode::new(value))),
            }           
        }
    }

    fn search(&self, value: T) -> bool{
        if self.value == value{
            true
        }else if self.value < value{
            match self.right {
                Some(ref node_rigth) => node_rigth.search(value),
                None => false,
            }
        }else {
            match self.left {
                Some(ref node_left) => node_left.search(value),
                None => false,
            }           
        }       
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


