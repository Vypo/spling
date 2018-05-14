extern crate spling;

use spling::Spling;

#[test]
fn it_works() {
    let mut buf: Spling<u8> = Spling::new(256);

    buf.reserve(100)
        .expect("should be able to reserve 100")
        .commit();

    {
        let avail = buf.avail();
        assert_eq!(avail.len(), 100);
    }


    buf.reserve(100)
        .expect("should be able to reserve 100")
        .commit();

    {
        let avail = buf.avail();
        assert_eq!(avail.len(), 200);
    }

    if !buf.reserve(100).is_none() {
        panic!("should not be able to reserve 100")
    }

    buf.reserve(56)
        .expect("should be able to reserve 56");

    {
        let avail = buf.avail();
        assert_eq!(avail.len(), 200);
        avail.consume();
    }

    buf.reserve(200)
        .expect("should be able to reserve 200");
}

#[test]
fn split_at_tail() {
    let mut buf: Spling<u8> = Spling::new(256);

    buf.reserve(256)
        .expect("should be able to reserve 256")
        .commit();

    {
        let avail = buf.avail();
        assert_eq!(avail.len(), 256);
        avail.consume();
    }

    {
        let avail = buf.avail();
        assert_eq!(avail.len(), 0);
        avail.consume();
    }

    buf.reserve(256)
        .expect("should be able to reserve 256");
}
