use std::mem;

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();
        
        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing is corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
// pub means we want people outside this module to be able to use List
pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link
}

impl List {
    pub fn new() -> Self {
        List {head: Link::Empty}
    }
}

impl List {
    pub fn push(&mut self, elem:i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty)
        });

        self.head = Link::More(new_node);
    }
    pub fn pop(&mut self) -> Option<i32> {
            match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty =>  None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}
