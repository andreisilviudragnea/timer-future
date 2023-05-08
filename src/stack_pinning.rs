#[cfg(test)]
mod tests {
    use std::marker::PhantomPinned;
    use std::pin::Pin;

    #[derive(Debug)]
    struct Test {
        a: String,
        b: *const String,
        _marker: PhantomPinned,
    }

    impl Test {
        fn new(txt: &str) -> Self {
            Test {
                a: String::from(txt),
                b: std::ptr::null(),
                _marker: PhantomPinned, // This makes our type `!Unpin`
            }
        }

        fn init(self: Pin<&mut Self>) {
            let self_ptr: *const String = &self.a;
            let this = unsafe { self.get_unchecked_mut() };
            this.b = self_ptr;
        }

        fn a(self: Pin<&Self>) -> &str {
            &self.get_ref().a
        }

        fn b(self: Pin<&Self>) -> &String {
            assert!(
                !self.b.is_null(),
                "Test::b called without Test::init being called first"
            );
            unsafe { &*(self.b) }
        }
    }

    #[test]
    fn test1() {
        // test1 is safe to move before we initialize it
        let mut test1 = Test::new("test1");
        // Notice how we shadow `test1` to prevent it from being accessed again
        let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };
        test1.as_mut().init();

        let mut test2 = Test::new("test2");
        let mut test2 = unsafe { Pin::new_unchecked(&mut test2) };
        test2.as_mut().init();

        assert_eq!(
            (test1.as_ref().a(), test1.as_ref().b().as_str()),
            ("test1", "test1")
        );
        assert_eq!(
            (test2.as_ref().a(), test2.as_ref().b().as_str()),
            ("test2", "test2")
        );
    }

    #[test]
    fn test2() {
        let mut test1 = Test::new("test1");
        let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };
        Test::init(test1.as_mut());

        let mut test2 = Test::new("test2");
        let mut test2 = unsafe { Pin::new_unchecked(&mut test2) };
        Test::init(test2.as_mut());

        assert_eq!(
            (test1.as_ref().a(), test1.as_ref().b().as_str()),
            ("test1", "test1")
        );
        // std::mem::swap(test1.get_mut(), test2.get_mut());
        assert_eq!(
            (test2.as_ref().a(), test2.as_ref().b().as_str()),
            ("test2", "test2")
        );
    }

    #[test]
    fn test3() {
        let mut test1 = Test::new("test1");
        let mut test1_pin = unsafe { Pin::new_unchecked(&mut test1) };
        Test::init(test1_pin.as_mut());

        drop(test1_pin);
        println!(r#"test1.b points to "test1": {:?}..."#, test1.b);

        let mut test2 = Test::new("test2");
        std::mem::swap(&mut test1, &mut test2);
        println!("... and now it points nowhere: {:?}", test1.b);

        assert_eq!(test1.b, std::ptr::null());
    }
}
