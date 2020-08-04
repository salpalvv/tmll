pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        // why is this "List" in the docs and not "Self"?
        Self {
            head: None,
        }
    }

    pub fn push(&mut self, elem: T) {
        // this takes the value in self.head (which is the current List's head) and moves it into next
        // replacing it with the second parameter
        // this makes self.head == None
        let next = self.head.take();

        let new_node = Box::new(Node {
            elem,
            next,
        });

        // now we need to set self.head equal to the new node which contains the old self.head
        // so, we are essentially pushing this new node into the front of the list while everything is moved back one
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        // matching on the replaced &mut self.head while setting &mut self.head to None temporarily
        //match self.head.take() {
        //    None => None,
        //    // if there is a Link, and not None, we want to move the current head (None)
        //    // to the next node and pull out the destructured, matched replaced &mut self.head's elem
        //    Some(node) => {
        //        self.head = node.next;
        //        Some(node.elem)
        //    }
        //}

        // refactored with a map
        // None return None
        // Some(node) do the stuff in closure (move head and return node.elem(with option mapped over it))
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        // `while let` == "do this thing until this pattern doesn't match"
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to None
            // so no unbounded recursion occurs.
        }
    }
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
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

    #[test]
    fn basics_float() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1.);
        list.push(2.);
        list.push(3.);

        assert_eq!(list.pop(), Some(3.));
        assert_eq!(list.pop(), Some(2.));

        list.push(4.);

        assert_eq!(list.pop(), Some(4.));
        assert_eq!(list.pop(), Some(1.));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek_test() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.peek_mut().map(|x| {
            *x = 42
        });

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.peek_mut(), Some(&mut 42));
    }
}