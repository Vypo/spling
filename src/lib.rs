pub struct Spling<T> {
    data: Vec<T>,

    head: usize,
    tail: usize,
    split: usize,
}

impl<T> Spling<T>
where
    T: Default,
{
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

    pub fn avail<'a>(&'a mut self) -> Availability<'a, T> {
        let mut head = self.head;
        let tail = self.tail;
        let split = self.split;

        let len;

        if head >= tail {
            if split == 0 {
                len = 0;
            } else if split == head {
                self.split = 0;
                self.head = 0;
                head = 0;
                len = tail;
            } else {
                len = split - head;
            }
        } else {
            len = tail - head;
        }

        Availability {
            len,
            start: head,
            spling: self,
        }
    }
}

pub struct Availability<'a, T: 'a> {
    spling: &'a mut Spling<T>,
    len: usize,
    start: usize,
}

impl<'a, T: 'a> Availability<'a, T> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn consume(self) {
        let mut new_head = self.spling.head + self.len;

        if new_head == self.spling.split {
            new_head = 0;
        }

        self.spling.head = new_head;
    }
}

impl<'a, T> AsRef<[T]> for Availability<'a, T> {
    fn as_ref(&self) -> &[T] {
        &self.spling.data[self.start..self.start + self.len]
    }
}

impl<'a, T> AsMut<[T]> for Availability<'a, T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.spling.data[self.start..self.start + self.len]
    }
}

pub struct Reservation<'a, T: 'a> {
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

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<'a, T> AsRef<[T]> for Reservation<'a, T> {
    fn as_ref(&self) -> &[T] {
        &self.spling.data[self.start..self.start + self.len]
    }
}

impl<'a, T> AsMut<[T]> for Reservation<'a, T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.spling.data[self.start..self.start + self.len]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consume_maximum() {
        let mut spling: Spling<u32> = Spling {
            data: Vec::from([0; 100].as_ref()),
            head: 0,
            tail: 100,
            split: 0,
        };

        {
            let availability = spling.avail();
            availability.consume();
        }

        assert_eq!(100, spling.head);
        assert_eq!(100, spling.tail);
        assert_eq!(0, spling.split);
    }

    #[test]
    fn consume_after_emptied() {
        let mut spling: Spling<u32> = Spling {
            data: Vec::from([0; 100].as_ref()),
            head: 100,
            tail: 50,
            split: 100,
        };

        {
            let availability = spling.avail();
            assert_eq!(50, availability.len());
            availability.consume();
        }

        assert_eq!(50, spling.head);
        assert_eq!(50, spling.tail);
        assert_eq!(0, spling.split);
    }

    #[test]
    fn avail_maximum() {
        let mut spling: Spling<u32> = Spling {
            data: Vec::from([0; 100].as_ref()),
            head: 0,
            tail: 100,
            split: 0,
        };

        let availability = spling.avail();
        assert_eq!(100, availability.len());
    }

    #[test]
    fn avail_thirty() {
        let mut spling: Spling<u32> = Spling {
            data: Vec::from([0; 100].as_ref()),
            head: 0,
            tail: 30,
            split: 0,
        };

        let availability = spling.avail();
        assert_eq!(30, availability.len());
    }

    #[test]
    fn avail_zero_elsewhere() {
        let mut spling: Spling<u32> = Spling {
            data: Vec::from([0; 100].as_ref()),
            head: 10,
            tail: 10,
            split: 0,
        };

        let availability = spling.avail();
        assert_eq!(0, availability.len());
    }

    #[test]
    fn avail_zero() {
        let mut spling: Spling<u32> = Spling {
            data: Vec::from([0; 100].as_ref()),
            head: 0,
            tail: 0,
            split: 0,
        };

        let availability = spling.avail();
        assert_eq!(0, availability.len());
    }

    #[test]
    fn avail_with_split() {
        let mut spling: Spling<u32> = Spling {
            data: Vec::from([0; 100].as_ref()),
            head: 50,
            tail: 50,
            split: 90,
        };

        let avail = spling.avail();
        assert_eq!(40, avail.len());
    }

    #[test]
    fn commit_after_emptied() {
        let mut spling: Spling<u32> = Spling {
            data: Vec::from([0; 100].as_ref()),
            head: 100,
            tail: 100,
            split: 0,
        };

        {
            let mut reservation = spling.reserve(50).unwrap();
            assert_eq!(50, reservation.len());

            {
                let banana = reservation.as_mut();
                assert_eq!(50, banana.len());
            }

            reservation.commit();
        }

        assert_eq!(100, spling.head);
        assert_eq!(50, spling.tail);
        assert_eq!(100, spling.split);
    }

    #[test]
    fn commit_with_split() {
        let mut spling: Spling<u32> = Spling {
            data: Vec::from([0; 100].as_ref()),
            head: 50,
            tail: 90,
            split: 0,
        };

        {
            let mut reservation = spling.reserve(50).unwrap();
            assert_eq!(50, reservation.len());

            {
                let banana = reservation.as_mut();
                assert_eq!(50, banana.len());
            }

            reservation.commit();
        }

        assert_eq!(50, spling.head);
        assert_eq!(50, spling.tail);
        assert_eq!(90, spling.split);
    }

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
