#[allow(dead_code)]
struct FlattenIterator<T>
where
    T: Iterator,
    T::Item: IntoIterator,
{
    o: T,
    i: Option<<T::Item as IntoIterator>::IntoIter>,
}

impl<T> FlattenIterator<T>
where
    T: Iterator,
    T::Item: IntoIterator,
{
    fn new(o: T) -> Self {
        Self { o, i: None }
    }
}

impl<T> Iterator for FlattenIterator<T>
where
    T: Iterator,
    T::Item: IntoIterator,
{
    type Item = <T::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<<T::Item as IntoIterator>::Item> {
        loop {
            if let Some(inner_iterator) = self.i.as_mut() {
                let inner = inner_iterator.next();
                if inner.is_some() {
                    return inner;
                }
            }

            if let Some(item) = self.o.next() {
                self.i = Some(item.into_iter());
            } else {
                return None;
            }
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
        let mut f = flatten(vec![vec![1, 2, 3], vec![4, 5, 6]].into_iter());
        assert_eq!(f.next(), Some(1));
        assert_eq!(f.next(), Some(2));
        assert_eq!(f.next(), Some(3));
        assert_eq!(f.next(), Some(4));
        assert_eq!(f.next(), Some(5));
        assert_eq!(f.next(), Some(6));
        assert_eq!(f.next(), None);
    }

    #[test]
    fn three_vectors() {
        let mut f = flatten(vec![vec![1, 2], vec![3, 4], vec![5, 6]].into_iter());
        assert_eq!(f.next(), Some(1));
        assert_eq!(f.next(), Some(2));
        assert_eq!(f.next(), Some(3));
        assert_eq!(f.next(), Some(4));
        assert_eq!(f.next(), Some(5));
        assert_eq!(f.next(), Some(6));
        assert_eq!(f.next(), None);
    }
}
