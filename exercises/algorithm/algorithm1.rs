/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/


use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self
    where
        T: Ord,
    {
        let mut a_elements = Vec::new();
        let mut current = list_a.start;
        while let Some(node_ptr) = current {
            unsafe {
                let node = Box::from_raw(node_ptr.as_ptr());
                a_elements.push(node.val);
                current = node.next;
            }
        }

        let mut b_elements = Vec::new();
        current = list_b.start;
        while let Some(node_ptr) = current {
            unsafe {
                let node = Box::from_raw(node_ptr.as_ptr());
                b_elements.push(node.val);
                current = node.next;
            }
        }

        let mut merged = Vec::new();
        let mut a_iter = a_elements.into_iter();
        let mut b_iter = b_elements.into_iter();
        let mut a_next = a_iter.next();
        let mut b_next = b_iter.next();

        loop {
            match (a_next, b_next) {
                (Some(a_val), Some(b_val)) => {
                    if a_val <= b_val {
                        merged.push(a_val);
                        a_next = a_iter.next();
                        b_next = Some(b_val);
                    } else {
                        merged.push(b_val);
                        b_next = b_iter.next();
                        a_next = Some(a_val);
                    }
                }
                (Some(a_val), None) => {
                    merged.push(a_val);
                    merged.extend(a_iter);
                    break;
                }
                (None, Some(b_val)) => {
                    merged.push(b_val);
                    merged.extend(b_iter);
                    break;
                }
                (None, None) => break,
            }
        }

        let mut merged_list = LinkedList::new();
        for val in merged {
            merged_list.add(val);
        }

        merged_list
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];
        
        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        let mut list_c = LinkedList::merge(list_a, list_b);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        let mut list_c = LinkedList::merge(list_a, list_b);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
}