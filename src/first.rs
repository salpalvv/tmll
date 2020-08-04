use std::mem;

pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        // why is this "List" in the docs and not "Self"?
        Self {
            head: Link::Empty,
        }
    }

    pub fn push(&mut self, elem: i32) {
        // this takes the value in self.head (which is the current List's head) and moves it into next
        // replacing it with the second parameter
        // this makes self.head == Link::Empty
        let next = mem::replace(&mut self.head, Link::Empty);

        let new_node = Box::new(Node {
            elem,
            next,
        });

        // now we need to set self.head equal to the new node which contains the old self.head
        // so, we are essentially pushing this new node into the front of the list while everything is moved back one
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        // matching on the replaced &mut self.head while setting &mut self.head to Link::Empty temporarily
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            // if there is a Link, and not empty, we want to move the current head (Link::Empty)
            // to the next node and pull out the destructured, matched replaced &mut self.head's elem
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }

}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // `while let` == "do this thing until this pattern doesn't match"
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
    }
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);

        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}