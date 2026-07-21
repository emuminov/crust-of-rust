struct FlattenIterator<T>
where
    T: Iterator,
    T::Item: IntoIterator,
{
    outer: T,
    inner: Option<<T::Item as IntoIterator>::IntoIter>,
}

impl<T> FlattenIterator<T>
where
    T: Iterator,
    T::Item: IntoIterator,
{
    fn new(iterator: T) -> Self {
        Self {
            outer: iterator,
            inner: None,
        }
    }
}

impl<T> Iterator for FlattenIterator<T>
where
    T: Iterator,
    T::Item: IntoIterator,
{
    type Item = <T::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(inner_iterator) = &mut self.inner {
                if let Some(item) = inner_iterator.next() {
                    return Some(item);
                }
            }
            self.inner = Some(self.outer.next()?.into_iter());
        }
    }
}

fn flatten<T>(iterator: T) -> FlattenIterator<T>
where
    T: Iterator,
    T::Item: IntoIterator,
{
    FlattenIterator::new(iterator)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(flatten(vec![Vec::<i32>::new()].into_iter()).next(), None);
    }

    #[test]
    fn basic() {
        assert_eq!(flatten(vec![vec![1]].into_iter()).next(), Some(1));
    }

    #[test]
    fn two_vectors() {
        let f: Vec<_> = flatten(vec![vec![1, 2, 3], vec![4, 5, 6]].into_iter()).collect();
        assert_eq!(f, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn three_vectors() {
        let f: Vec<_> = flatten(vec![vec![1, 2], vec![3, 4], vec![5, 6]].into_iter()).collect();
        assert_eq!(f, vec![1, 2, 3, 4, 5, 6]);
    }
}
