pub struct List<T> {
    head: Link<T>
}

enum Link<T> {
    Empty,
    More(Box<Node<T>>),
}

impl<T> Default for Link<T> {
    fn default() -> Self { Link::Empty }
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}



impl<T> List<T> {

    /// Creates a new, empty [`List`].
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    /// Adds `elem` to the [`List`] at the front.
    pub fn push(&mut self, elem: T) {
        self.head = Link::More(
            Box::new(
                Node {
                    elem: elem,
                    next: std::mem::take(&mut self.head),
                }
            )
        );
    }

    /// Deletes and returns the first value from the [`List`].
    /// Returns [`None`] and leaves the [`List`] unchanged if it is empty.
    pub fn pop(&mut self) -> Option<T> {
       match std::mem::take(&mut self.head) { 
          Link::Empty => None,
          Link::More(node) => {
              let item = node.elem;
              self.head = node.next;
              Some(item)
          }
       }
    }


}

/// Custom destructor for [`List`], avoids recursive default [`Drop`] implementation.
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Link::More(node) = &mut self.head {
            self.head = std::mem::take(&mut node.next);
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
}
