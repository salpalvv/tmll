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

        // not we need to set self.head equal to the new node which contains the old self.head
        // so, we are essentially pushing this new node into the front of the list while everything is moved back one
        self.head = Link::More(new_node);
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
