#![allow(non_snake_case)]

#[derive(Debug, PartialEq)]
pub enum LinkedList<T> {
    Empty,
    Elem(T, Box<LinkedList<T>>)
}

impl<T> LinkedList<T> {

    /// create a new list
    ///
    /// # Example
    /// ```
    /// use LinkedList::*;
    ///
    /// let l = LinkedList::<i32>::new();
    /// assert_eq!(LinkedList::Empty, l);
    /// ```
    pub fn new() -> LinkedList<T> {
        return LinkedList::Empty;
    }

    /// push an item onto a list
    ///
    /// # Example
    /// ```
    /// use LinkedList::*;
    ///
    /// let l = LinkedList::<i32>::new().push(42);
    /// assert_eq!(LinkedList::Elem(42, Box::new(LinkedList::Empty)), l);
    /// ```
    pub fn push(self : LinkedList<T>, value : T) -> LinkedList<T> {
        return LinkedList::Elem(value, Box::new(self));
    }

    /// removes the most recently pushed item from the list, as
    /// well as the remainder of the list
    ///
    /// # Example 1
    /// ```
    /// use LinkedList::*;
    ///
    /// let l = LinkedList::<i32>::new();
    /// let m = l.pop();
    /// assert_eq!(None, m);
    /// ```
    ///
    /// # Example 2
    /// ```
    /// use LinkedList::*;
    ///
    /// let l = LinkedList::<i32>::new();
    /// let m = l.push(42);
    /// let n = m.pop();
    /// assert_eq!(Some((42, LinkedList::Empty)), n);
    /// ```
    pub fn pop(self : LinkedList<T>) -> Option<(T, LinkedList<T>)> {
        match self {
            LinkedList::Empty => return None,
            LinkedList::Elem(value, tail) => {
                return Some((value, *tail))
            }
        }
    }

    /// destroys the list efficiently
    ///
    /// # Example
    /// ```
    /// use LinkedList::*;
    ///
    /// let mut l = LinkedList::<i32>::new();
    /// for i in 1 .. 10_000_000 {
    ///     l = l.push(i);
    /// }
    /// l.drop();
    /// ```
    pub fn drop(self : LinkedList<T>) {
        let mut current = self;
        while let LinkedList::Elem(_, tail) = current {
            current = *tail;
        }
    }
}

// impl<T> Drop for LinkedList<T> {
//     fn drop(self : &mut LinkedList<T>) {
//         unreachable!();
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_empty_list_is_empty() {
        let l = LinkedList::<i32>::new();
        assert_eq!(LinkedList::Empty, l);
    }

    #[test]
    fn push_item_adds_item() {
        let l = LinkedList::<i32>::new().push(42);
        assert_eq!(LinkedList::Elem(42, Box::new(LinkedList::Empty)), l);
    }

    #[test]
    fn push_two_items_adds_two_items_in_reverse_order() {
        let l = LinkedList::<i32>::new().push(42).push(13);
        assert_eq!(LinkedList::Elem(13, Box::new(LinkedList::Elem(42, Box::new(LinkedList::Empty)))), l);
    }

    #[test]
    fn pop_item_from_empty_return_none() {
        let l = LinkedList::<i32>::new();
        let m = l.pop();
        assert_eq!(None, m);
    }

    #[test]
    fn pop_item_removes_item() {
        let l = LinkedList::<i32>::new();
        let m = l.push(42);
        let n = m.pop();
        assert_eq!(Some((42, LinkedList::Empty)), n);
    }

    #[test]
    fn pop_two_items_from_list() {
        let l = LinkedList::<i32>::new().push(42).push(13);

        let first = l.pop();
        assert_eq!(Some((13, LinkedList::Elem(42, Box::new(LinkedList::Empty)))), first);

        let second = first.unwrap().1.pop();
        assert_eq!(Some((42, LinkedList::Empty)), second);

        let third = second.unwrap().1.pop();
        assert_eq!(None, third);
    }

    #[test]
    fn ten_million_items() {
        let mut l = LinkedList::<i32>::new();
        for i in 1 .. 10_000_000 {
            l = l.push(i);
        }
        l.drop();
    }
}
