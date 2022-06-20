/// This is the top-level (singly-)linked list structure.
pub struct List<T> {
    head: Link<T>
}

/// A [`Link`] is just the fancy version of a pointer to some position in our list (e.g. a head
/// pointer)
type Link<T> = Option<Box<Node<T>>>;

/// The [`Node`] structure is the atomic data structure of a linked list. It holds a single element
/// of generic type and an owned, optional reference to the next item.
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


    /// Returns a reference to the first value from the [`List`] without removing it.
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    /// Returns a mutable reference to the first value from the [`List`] without removing it.
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }


    /// Returns an [`Iterator`] by value into the [`List`]
    pub fn into_iter(self) -> IntoIter<T> { IntoIter(self) }

    /// Returns an [`Iterator`] by reference into the [`List`]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref()
        }
    }
    ///
    /// Returns a mutable [`Iterator`] by reference into the [`List`]
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut()
        }
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

//////////////////////////////
// Iterator implementations //
//////////////////////////////

pub struct IntoIter<T>(List<T>);

/// Iterator trait implementation for a [`List`].
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let IntoIter(list) = self;
        list.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}

/// Iterator trait implementation for a [`List`].
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}
    
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>
}

/// Iterator trait implementation for a [`List`].
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}
    


///////////
// Tests //
///////////
#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn basic_stack() {
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
    fn peek() {
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

    #[test]
    fn into_iter() {
        // Add to the list, then iterate through it.
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        for val in vec![Some(3), Some(2), Some(1), None] {
            assert_eq!(iter.next(), val);
        }
    }

    #[test]
    fn iter() {
        // Add to the list, then iterate through it.
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        // Add to the list, then iterate through it.
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}
