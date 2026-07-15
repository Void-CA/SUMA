use std::collections::VecDeque;
use std::fmt::Display;

#[derive(Clone)]
struct AvlNode<T> {
    value: T,
    height: i32,
    left: Option<Box<AvlNode<T>>>,
    right: Option<Box<AvlNode<T>>>,
}

pub struct AvlTree<T> {
    root: Option<Box<AvlNode<T>>>,
}
impl<T: Ord + Clone + Display> AvlTree<T> {
    pub fn new() -> Self {
        AvlTree { root: None }
    }

    pub fn insert(&mut self, value: T) {
        self.root = Some(Self::insert_node(self.root.take(), value));
    }

    pub fn insert_node(node: Option<Box<AvlNode<T>>>, value: T) -> Box<AvlNode<T>> {
        match node {
            Some(mut n) => {
                if value < n.value {
                    n.left = Some(Self::insert_node(n.left.take(), value));
                } else if value > n.value {
                    n.right = Some(Self::insert_node(n.right.take(), value));
                }
                Self::balance(n)
            }
            None => Box::new(AvlNode {
                value,
                height: 1,
                left: None,
                right: None,
            }),
        }
    }
    fn height(node: &Option<Box<AvlNode<T>>>) -> i32 {
        match node {
            Some(n) => n.height,
            None => 0,
        }
    }

    fn update_height(node: &mut Box<AvlNode<T>>) {
        let left_height = Self::height(&node.left);
        let right_height = Self::height(&node.right);
        node.height = 1 + left_height.max(right_height);
    }

    fn balance_factor(node: &Option<&Box<AvlNode<T>>>) -> i32 {
        match node {
            Some(n) => Self::height(&n.left) - Self::height(&n.right),
            None => 0,
        }
    }

    fn rotate_right(mut root_node: Box<AvlNode<T>>) -> Box<AvlNode<T>> {
        let mut new_root = root_node.left.take().unwrap();
        let t2 = new_root.right.take();

        new_root.right = Some(root_node);
        new_root.right.as_mut().unwrap().left = t2;

        Self::update_height(new_root.right.as_mut().unwrap());
        Self::update_height(&mut new_root);

        new_root
    }

    fn rotate_left(mut x: Box<AvlNode<T>>) -> Box<AvlNode<T>> {
        let mut y = x.right.take().unwrap();
        let t2 = y.left.take();

        y.left = Some(x);
        y.left.as_mut().unwrap().right = t2;

        Self::update_height(y.left.as_mut().unwrap());
        Self::update_height(&mut y);

        y
    }

    fn balance(node: Box<AvlNode<T>>) -> Box<AvlNode<T>> {
        let mut node = node;
        Self::update_height(&mut node);
        let balance = Self::balance_factor(&Some(&node));

        if balance > 1 {
            if Self::balance_factor(&node.left.as_ref()) < 0 {
                node.left = Some(Self::rotate_left(node.left.take().unwrap()));
            }
            return Self::rotate_right(node);
        }

        if balance < -1 {
            if Self::balance_factor(&node.right.as_ref()) > 0 {
                node.right = Some(Self::rotate_right(node.right.take().unwrap()));
            }
            return Self::rotate_left(node);
        }

        node
    }

    // ── Search operations ──

    pub fn contains(&self, value: &T) -> bool {
        Self::contains_node(&self.root, value)
    }

    fn contains_node(node: &Option<Box<AvlNode<T>>>, value: &T) -> bool {
        match node {
            Some(n) => {
                if *value < n.value {
                    Self::contains_node(&n.left, value)
                } else if *value > n.value {
                    Self::contains_node(&n.right, value)
                } else {
                    true
                }
            }
            None => false,
        }
    }

    pub fn find(&self, value: &T) -> Option<&T> {
        Self::find_node(&self.root, value)
    }

    fn find_node<'a>(node: &'a Option<Box<AvlNode<T>>>, value: &T) -> Option<&'a T> {
        match node {
            Some(n) => {
                if *value < n.value {
                    Self::find_node(&n.left, value)
                } else if *value > n.value {
                    Self::find_node(&n.right, value)
                } else {
                    Some(&n.value)
                }
            }
            None => None,
        }
    }

    pub fn min(&self) -> Option<&T> {
        let mut current = self.root.as_ref()?;
        while let Some(left) = &current.left {
            current = left;
        }
        Some(&current.value)
    }

    pub fn max(&self) -> Option<&T> {
        let mut current = self.root.as_ref()?;
        while let Some(right) = &current.right {
            current = right;
        }
        Some(&current.value)
    }

    // ── Deletion ──

    pub fn delete(&mut self, value: &T) {
        self.root = Self::delete_node(self.root.take(), value);
    }

    fn delete_node(node: Option<Box<AvlNode<T>>>, value: &T) -> Option<Box<AvlNode<T>>> {
        match node {
            Some(mut n) => {
                if *value < n.value {
                    n.left = Self::delete_node(n.left.take(), value);
                } else if *value > n.value {
                    n.right = Self::delete_node(n.right.take(), value);
                } else {
                    // Found the node to delete
                    return match (n.left.take(), n.right.take()) {
                        (None, None) => None,
                        (Some(l), None) => Some(l),
                        (None, Some(r)) => Some(r),
                        (Some(l), Some(r)) => {
                            // Find in-order successor (min of right subtree)
                            let successor_val = Self::min_node(&r).clone();
                            n.left = Some(l);
                            n.right = Self::delete_node(Some(r), &successor_val);
                            n.value = successor_val;
                            Some(Self::balance(n))
                        }
                    };
                }
                Some(Self::balance(n))
            }
            None => None,
        }
    }

    fn min_node(node: &Box<AvlNode<T>>) -> &T {
        let mut current = node;
        while let Some(left) = &current.left {
            current = left;
        }
        &current.value
    }

    /* ============================
     *     IN-ORDER TRAVERSAL
     * ============================ */
    pub fn in_order(&self) -> Vec<&T> {
        let mut result = Vec::new();
        Self::in_order_traversal(&self.root, &mut result);
        result
    }

    fn in_order_traversal<'a>(
        node: &'a Option<Box<AvlNode<T>>>,
        result: &mut Vec<&'a T>,
    ) {
        if let Some(current) = node {
            Self::in_order_traversal(&current.left, result);
            result.push(&current.value);
            Self::in_order_traversal(&current.right, result);
        }
    }

    /* ============================
     *     PRE-ORDER TRAVERSAL
     * ============================ */
    pub fn pre_order(&self) -> Vec<&T> {
        let mut result = Vec::new();
        Self::pre_order_traversal(&self.root, &mut result);
        result
    }

    fn pre_order_traversal<'a>(
        node: &'a Option<Box<AvlNode<T>>>,
        result: &mut Vec<&'a T>,
    ) {
        if let Some(current) = node {
            result.push(&current.value);
            Self::pre_order_traversal(&current.left, result);
            Self::pre_order_traversal(&current.right, result);
        }
    }

    /* ============================
     *     POST-ORDER TRAVERSAL
     * ============================ */
    pub fn post_order(&self) -> Vec<&T> {
        let mut result = Vec::new();
        Self::post_order_traversal(&self.root, &mut result);
        result
    }

    fn post_order_traversal<'a>(
        node: &'a Option<Box<AvlNode<T>>>,
        result: &mut Vec<&'a T>,
    ) {
        if let Some(current) = node {
            Self::post_order_traversal(&current.left, result);
            Self::post_order_traversal(&current.right, result);
            result.push(&current.value);
        }
    }

    /* ============================
     *       LEVEL-ORDER (BFS)
     * ============================ */
    pub fn level_order(&self) -> Vec<&T> {
        let mut result = Vec::new();
        let mut queue = VecDeque::new();

        if let Some(root) = &self.root {
            queue.push_back(root.as_ref());
        }

        while let Some(node) = queue.pop_front() {
            result.push(&node.value);

            if let Some(left) = &node.left {
                queue.push_back(left.as_ref());
            }
            if let Some(right) = &node.right {
                queue.push_back(right.as_ref());
            }
        }

        result
    }

    /* ============================
     *     PRINT IN ORDER (vector)
     * ============================ */
    pub fn print_in_order(&self) -> String {
        self.in_order()
            .iter()
            .map(|&value| value.to_string())
            .collect::<Vec<String>>()
            .join("\n") + "\n"
    }

    /* ============================
     *     PRINT TREE (DEFAULT)
     * ============================ */
    pub fn print_tree(&self) -> String {
        let mut result = String::new();
        if let Some(root) = &self.root {
            result.push_str(&format!("{}\n", root.value));

            // hijos
            let mut children: Vec<&Box<AvlNode<T>>> = Vec::new();
            if let Some(left) = &root.left {
                children.push(left);
            }
            if let Some(right) = &root.right {
                children.push(right);
            }

            for (i, child) in children.iter().enumerate() {
                let is_tail = i == children.len() - 1;
                Self::print_tree_node(child, "", is_tail, &mut result);
            }
        }
        result
    }

    fn print_tree_node(
        node: &Box<AvlNode<T>>,
        prefix: &str,
        is_tail: bool,
        result: &mut String,
    ) {
        result.push_str(&format!(
            "{}{}{}\n",
            prefix,
            if is_tail { "└── " } else { "├── " },
            node.value
        ));

        let mut children: Vec<&Box<AvlNode<T>>> = Vec::new();
        if let Some(left) = &node.left {
            children.push(left);
        }
        if let Some(right) = &node.right {
            children.push(right);
        }

        for (i, child) in children.iter().enumerate() {
            let tail = i == children.len() - 1;
            let new_prefix = format!("{}{}", prefix, if is_tail { "    " } else { "│   " });

            Self::print_tree_node(child, &new_prefix, tail, result);
        }
    }

    /* ============================
     *     DEFAULT PRINT METHOD
     * ============================ */
    pub fn print(&self) -> String {
        self.print_tree()
    }

    pub fn len(&self) -> usize {
        self.in_order().len()
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tree_height(node: &Option<Box<AvlNode<i32>>>) -> i32 {
        match node {
            Some(n) => n.height,
            None => 0,
        }
    }

    fn assert_balanced(node: &Option<Box<AvlNode<i32>>>) {
        if let Some(n) = node {
            assert_balanced(&n.left);
            assert_balanced(&n.right);
            let bf = tree_height(&n.left) - tree_height(&n.right);
            assert!(bf.abs() <= 1, "Node {} has balance factor {}", n.value, bf);
        }
    }

    #[test]
    fn test_avl_insertion() {
        let mut avl = AvlTree::new();
        avl.insert(10);
        avl.insert(20);
        avl.insert(30);
        avl.insert(40);
        avl.insert(50);

        assert_balanced(&avl.root);
        assert_eq!(avl.in_order(), vec![&10, &20, &30, &40, &50]);
        assert_eq!(avl.len(), 5);
    }

    #[test]
    fn test_avl_contains_and_find() {
        let mut avl = AvlTree::new();
        avl.insert(5);
        avl.insert(3);
        avl.insert(7);
        avl.insert(1);
        avl.insert(9);

        assert_balanced(&avl.root);
        assert!(avl.contains(&5));
        assert!(avl.contains(&1));
        assert!(avl.contains(&9));
        assert!(!avl.contains(&0));
        assert!(!avl.contains(&4));

        assert_eq!(avl.find(&5), Some(&5));
        assert_eq!(avl.find(&1), Some(&1));
        assert_eq!(avl.find(&10), None);
    }

    #[test]
    fn test_avl_min_max() {
        let mut avl = AvlTree::new();
        avl.insert(10);
        avl.insert(20);
        avl.insert(30);
        avl.insert(5);
        avl.insert(15);

        assert_eq!(avl.min(), Some(&5));
        assert_eq!(avl.max(), Some(&30));
    }

    #[test]
    fn test_avl_delete() {
        let mut avl = AvlTree::new();
        avl.insert(10);
        avl.insert(20);
        avl.insert(30);
        avl.insert(5);
        avl.insert(15);

        assert_balanced(&avl.root);
        assert_eq!(avl.len(), 5);

        avl.delete(&20);
        assert_balanced(&avl.root);
        assert!(!avl.contains(&20));
        assert_eq!(avl.len(), 4);
        assert_eq!(avl.in_order(), vec![&5, &10, &15, &30]);

        avl.delete(&5);
        assert_balanced(&avl.root);
        assert!(!avl.contains(&5));
        assert_eq!(avl.len(), 3);

        avl.delete(&10);
        assert_balanced(&avl.root);
        assert_eq!(avl.len(), 2);

        avl.delete(&15);
        avl.delete(&30);
        assert!(avl.is_empty());
    }
}