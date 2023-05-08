use std::marker::PhantomPinned;
use std::pin::Pin;

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl Test {
    fn new(txt: &str) -> Pin<Box<Self>> {
        let t = Test {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned,
        };
        let mut boxed = Box::pin(t);
        let self_ptr: *const String = &boxed.a;
        let pin1 = boxed.as_mut();
        let test = unsafe { pin1.get_unchecked_mut() };
        test.b = self_ptr;

        boxed
    }

    fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    fn b(self: Pin<&Self>) -> &String {
        unsafe { &*self.b }
    }
}

#[test]
fn test1() {
    let test1 = Test::new("test1");
    let test2 = Test::new("test2");

    assert_eq!(
        (test1.as_ref().a(), test1.as_ref().b().as_str()),
        ("test1", "test1")
    );
    assert_eq!(
        (test2.as_ref().a(), test2.as_ref().b().as_str()),
        ("test2", "test2")
    );
}
