use std::sync::Arc;

/// This is an immutable linked list, like in a functional programming language.
pub struct List<T> {
    head: Link<T>
}

type Link<T> = Option<Arc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    /// Default, empty constructor for a [`List`].
    pub fn new() -> Self {
        List { head: None }
    }

    /// Attach a new element `elem` to the front of a [`List`].
    pub fn prepend(&self, elem: T) -> List<T> {
        List {
            head: Some(Arc::new(Node {
                elem: elem,
                next: self.head.clone(),
            }))
        }
    }

    /// Return the tail of a [`List`], which is every element besides the first element.
    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone())
        }
    }

    /// Return the head of a [`List`], which is the first element.
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem )
    }

    /// Return an [`Iter`] across a [`List`].
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
    }
}

/////////////////
// Custom Drop //
/////////////////
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(boxed_node) = cur_link {
            if let Ok(mut node) = Arc::try_unwrap(boxed_node) {
                cur_link = node.next.take();
            } else {
                break;
            }
        }
    }
}



//////////////////////////////
// Iterator implementations //
//////////////////////////////

// Iter
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
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
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let mut list = list.prepend(1).prepend(2).prepend(3);
        let expected = vec![Some(&3), Some(&2), Some(&1), None];

        for val in expected {
            assert_eq!(list.head(), val);
            list = list.tail();
        }

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);

    }

    #[test]
    fn iter() {
        let list = List::new().prepend(1).prepend(2).prepend(3);
        let expected = vec![Some(&3), Some(&2), Some(&1), None];

        let mut iter = list.iter();
        for val in expected {
            assert_eq!(iter.next(), val);
        }
    }
}
