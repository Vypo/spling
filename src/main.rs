struct Spling<T> {
    data: Vec<T>,

    head: usize,
    tail: usize,
    split: usize,
}

impl<T> Spling<T> where T: Default {
    pub fn new(capacity: usize) -> Self {
        let mut data = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            data.push(T::default());
        }

        Self {
            data,
            head: 0,
            tail: 0,
            split: 0,
        }
    }
}

impl<T> Spling<T> {
    pub fn reserve<'a>(&'a mut self, len: usize) -> Option<Reservation<'a, T>> {
        if self.head <= self.tail {
            let available = self.data.len() - self.tail;

            if available >= len {
                return Some(Reservation {
                    start: self.tail,
                    spling: self,
                    len,
                });
            }
        }

        if self.head >= len {
            return Some(Reservation {
                spling: self,
                start: 0,
                len,
            });
        }

        None
    }
}

struct Reservation<'a, T: 'a> {
    spling: &'a mut Spling<T>,
    len: usize,
    start: usize,
}

impl<'a, T> Reservation<'a, T> {
    pub fn commit(self) {
        if self.start == 0 {
            self.spling.split = self.spling.tail;
        }

        self.spling.tail = self.start + self.len;
    }
}

impl<'a, T> AsRef<[T]> for Reservation<'a, T> {
    fn as_ref(&self) -> &[T] {
        &self.spling.data[self.start..self.start+self.len]
    }
}

impl<'a, T> AsMut<[T]> for Reservation<'a, T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.spling.data[self.start..self.start+self.len]
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commit_thirty_then_sixty() {
        let mut spling: Spling<u32> = Spling::new(100);

        {
            let mut reservation = spling.reserve(30).unwrap();

            {
                let banana = reservation.as_mut();
                assert_eq!(30, banana.len());
            }

            reservation.commit();
        }
        {
            let mut reservation = spling.reserve(60).unwrap();

            {
                let banana = reservation.as_mut();
                assert_eq!(60, banana.len());
            }

            reservation.commit();
        }

        assert_eq!(0, spling.head);
        assert_eq!(90, spling.tail);
        assert_eq!(0, spling.split);
    }

    #[test]
    fn commit_thirty() {
        let mut spling: Spling<u32> = Spling::new(100);

        {
            let mut reservation = spling.reserve(30).unwrap();

            {
                let banana = reservation.as_mut();
                assert_eq!(30, banana.len());
            }

            reservation.commit();
        }

        assert_eq!(0, spling.head);
        assert_eq!(30, spling.tail);
        assert_eq!(0, spling.split);
    }

    #[test]
    fn commit_zero() {
        let mut spling: Spling<u32> = Spling::new(100);

        {
            let mut reservation = spling.reserve(0).unwrap();

            {
                let banana = reservation.as_mut();
                assert_eq!(0, banana.len());
            }

            reservation.commit();
        }

        assert_eq!(0, spling.head);
        assert_eq!(0, spling.tail);
        assert_eq!(0, spling.split);
    }

    #[test]
    fn commit_maximum() {
        let mut spling: Spling<u32> = Spling::new(100);

        {
            let reservation = spling.reserve(100).unwrap();

            {
                let banana = reservation.as_ref();
                assert_eq!(100, banana.len());
            }

            reservation.commit();
        }

        assert_eq!(0, spling.head);
        assert_eq!(100, spling.tail);
        assert_eq!(0, spling.split);
    }
}
