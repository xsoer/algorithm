// 单向链表
// use std::collections::LinkedList;
// use core::ptr::NonNull;

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
    len: usize,
}

#[derive(Debug)]
struct Node<T> {
    element: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(element: T) -> Self {
        Node {
            element,
            next: None,
        }
    }

    fn into_element(self: Box<Self>) -> T {
        self.element
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    // pub fn push_back_node(&mut self, mut node: Box<Node<T>>) {
    //     unsafe {
    //         node.next = None;
    //         match self.tail {
    //             None => self.head = Some(node),
    //             Some(tail) => (*tail.as_ptr()).next = Some(node),
    //         }
    //     }
    // }

    pub fn delete(&mut self, pos: usize) {}

    pub fn traverse(self) {}

    pub fn reverse(&mut self) {}

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn length(self) -> usize {
        self.len
        // let mut cnt = 0;
        // let mut node = self.next;
        // loop {
        //     match node {
        //         Some(d) =>  {
        //             cnt += 1;
        //             node = d.next
        //         },
        //         None => break
        //     }
        // }
        // cnt
    }
}

mod test {
    use super::*;

    #[test]
    fn test_linklist() {
        let list: LinkedList<u32> = LinkedList::new();

        println!("{:?}", list);
        println!("len{:?}", list.length());
    }
}
