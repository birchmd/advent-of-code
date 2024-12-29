use std::iter::Iterator;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LinkedList<T> {
    Cons { head: T, tail: Box<LinkedList<T>> },
    Nil,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self::Nil
    }

    pub fn cons(self, t: T) -> Self {
        Self::Cons {
            head: t,
            tail: Box::new(self),
        }
    }

    pub fn collect_reverse<I>(items: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        items
            .into_iter()
            .fold(LinkedList::Nil, |acc, item| acc.cons(item))
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, LinkedList::Nil)
    }

    pub fn iter(&self) -> LLIterRef<T> {
        LLIterRef { state: self }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct LLIterRef<'a, T> {
    state: &'a LinkedList<T>,
}

impl<'a, T> Iterator for LLIterRef<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            LinkedList::Cons { head, tail } => {
                self.state = tail;
                Some(head)
            }
            LinkedList::Nil => None,
        }
    }
}

pub struct LLIter<T> {
    state: LinkedList<T>,
}

impl<T> Iterator for LLIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut x: LinkedList<T> = LinkedList::Nil;
        std::mem::swap(&mut self.state, &mut x);
        match x {
            LinkedList::Cons { head, tail } => {
                self.state = *tail;
                Some(head)
            }
            LinkedList::Nil => None,
        }
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type IntoIter = LLIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        LLIter { state: self }
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type IntoIter = LLIterRef<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[test]
fn test_iter() {
    let list = LinkedList::new().cons(3).cons(2).cons(1).cons(0);

    let expected_items = vec![0, 1, 2, 3];
    assert!(list.iter().zip(&expected_items).all(|(a, b)| a == b));
    assert_eq!(list.into_iter().collect::<Vec<usize>>(), expected_items);
}

#[test]
fn test_collect() {
    let items = vec![0, 1, 2, 3];
    let list = LinkedList::collect_reverse(items.into_iter().map(|x| x + x));
    let expected_list = LinkedList::new().cons(0).cons(2).cons(4).cons(6);

    assert_eq!(list, expected_list);
}
