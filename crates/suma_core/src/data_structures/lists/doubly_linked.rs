use std::fmt;

#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct DoublyLinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            length: 0,
        }
    }

    pub fn push_front(&mut self, value: T) {
        self.head = Some(Box::new(Node {
            value,
            next: self.head.take(),
        }));
        self.length += 1;
    }

    pub fn push_back(&mut self, value: T) {
        let new_node = Box::new(Node { value, next: None });

        if let Some(tail) = Self::find_tail_mut(&mut self.head) {
            tail.next = Some(new_node);
        } else {
            self.head = Some(new_node);
        }
        self.length += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.length -= 1;
            node.value
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        if self.length == 1 {
            return self.pop_front();
        }

        let mut current = &mut self.head;
        while current.as_ref().unwrap().next.is_some() {
            current = &mut current.as_mut().unwrap().next;
        }

        let last_node = current.take().unwrap();
        self.length -= 1;
        Some(last_node.value)
    }

    pub fn front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn back(&self) -> Option<&T> {
        self.iter().last()
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.value)
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.iter_mut().last()
    }

    pub fn find(&self, value: &T) -> Option<&T>
    where
        T: PartialEq,
    {
        self.iter().find(|&v| v == value)
    }

    pub fn find_mut(&mut self, value: &T) -> Option<&mut T>
    where
        T: PartialEq,
    {
        self.iter_mut().find(|v| *v == value)
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn clear(&mut self) {
        while self.pop_front().is_some() {}
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_deref_mut() }
    }

    pub fn iter_rev(&self) -> IterRev<'_, T> {
        let nodes: Vec<&T> = self.iter().collect();
        let len = nodes.len();
        IterRev { nodes, index: len }
    }

    fn find_tail_mut(node: &mut Option<Box<Node<T>>>) -> Option<&mut Box<Node<T>>> {
        let mut current = node;
        while let Some(inner) = current.as_mut() {
            if inner.next.is_none() {
                return Some(inner);
            }
            current = &mut inner.next;
        }
        None
    }
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<T: Clone> Clone for DoublyLinkedList<T> {
    fn clone(&self) -> Self {
        let mut new_list = Self::new();
        for item in self.iter() {
            new_list.push_back(item.clone());
        }
        new_list
    }
}

impl<T: fmt::Debug> fmt::Debug for DoublyLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.value
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.value
        })
    }
}

pub struct IterRev<'a, T> {
    nodes: Vec<&'a T>,
    index: usize,
}

impl<'a, T> Iterator for IterRev<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == 0 {
            None
        } else {
            self.index -= 1;
            Some(self.nodes[self.index])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doubly_linked_basic() {
        let mut list = DoublyLinkedList::new();

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.len(), 3);
        assert_eq!(list.front(), Some(&1));
        assert_eq!(list.back(), Some(&3));

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_push_pop_both_ends() {
        let mut list = DoublyLinkedList::new();

        list.push_front(2);
        list.push_front(1);
        list.push_back(3);
        list.push_back(4);

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_find() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.find(&2), Some(&2));
        assert_eq!(list.find(&4), None);

        if let Some(value) = list.find_mut(&2) {
            *value = 20;
        }
        assert_eq!(list.find(&20), Some(&20));
    }

    #[test]
    fn test_iterators() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let forward: Vec<&i32> = list.iter().collect();
        assert_eq!(forward, vec![&1, &2, &3]);

        let reverse: Vec<&i32> = list.iter_rev().collect();
        assert_eq!(reverse, vec![&3, &2, &1]);

        for value in list.iter_mut() {
            *value *= 2;
        }
        let doubled: Vec<&i32> = list.iter().collect();
        assert_eq!(doubled, vec![&2, &4, &6]);
    }

    #[test]
    fn test_clear() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.len(), 3);
        list.clear();
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn test_front_back_mut() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        if let Some(front) = list.front_mut() {
            *front = 10;
        }
        if let Some(back) = list.back_mut() {
            *back = 30;
        }

        assert_eq!(list.front(), Some(&10));
        assert_eq!(list.back(), Some(&30));
    }

    #[test]
    fn test_generic_types() {
        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }
        let mut list = DoublyLinkedList::new();
        list.push_back(Point { x: 1, y: 2 });
        list.push_back(Point { x: 3, y: 4 });

        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(Point { x: 1, y: 2 }));
        assert_eq!(list.pop_back(), Some(Point { x: 3, y: 4 }));
    }
}
