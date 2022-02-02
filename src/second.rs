pub struct List<T> {
    head: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}



impl<T> List<T> {

    /// Creates a new, empty [`List`].
    pub fn new() -> Self {
        List { head: None }
    }

    /// Adds `elem` to the [`List`] at the front.
    pub fn push(&mut self, elem: T) {
        self.head = Some(
            Box::new(
                Node {
                    elem: elem,
                    next: self.head.take(),
                }
            )
        );
    }

    /// Deletes and returns the first value from the [`List`].
    /// Returns [`None`] and leaves the [`List`] unchanged if it is empty.
    pub fn pop(&mut self) -> Option<T> {
       self.head.take().map(|node| {
           self.head = node.next;
           node.elem
       })
    }


    /// Returns a reference to the first value from the [`List`] without removing it, as in [`pop`]
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    /// Returns a mutable reference to the first value from the [`List`] without removing it, as in
    /// [`pop`]
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

/// Custom destructor for [`List`], avoids recursive default [`Drop`] implementation.
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(node) = &mut self.head {
            self.head = node.next.take();
        }
    }
}
    


///////////
// Tests //
///////////
#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn basic_stack_tests() {
        let mut list = List::new();

        // Empty list behavior check
        assert_eq!(list.pop(), None);

        // Add to the list, then check for non-emptiness and orderliness
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        list.push(4);
        list.push(5);
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));

        // Verify list is empty again
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek_tests() {
        let mut list = List::new();

        // Empty list behavior check
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);

        // Add to the list, then for the correct item
        list.push(1);
        list.push(2);
        assert_eq!(list.peek(), Some(&2));
        assert_eq!(list.peek_mut(), Some(&mut 2));

        // Verify mutation of the head item
        *list.peek_mut().unwrap() = -3;
        assert_eq!(list.peek(), Some(&-3));

    }
}
