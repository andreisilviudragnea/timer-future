#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
}

impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        let self_ref: *const String = &self.a;
        self.b = self_ref;
    }

    fn a(&self) -> &str {
        &self.a
    }

    fn b(&self) -> &String {
        assert!(
            !self.b.is_null(),
            "Test::b called without Test::init being called first"
        );
        unsafe { &*(self.b) }
    }
}

#[test]
fn test1() {
    let mut test1 = Test::new("test1");
    test1.init();
    let mut test2 = Test::new("test2");
    test2.init();

    assert_eq!((test1.a(), test1.b().as_str()), ("test1", "test1"));
    assert_eq!((test2.a(), test2.b().as_str()), ("test2", "test2"));
}

#[test]
fn test2() {
    let mut test1 = Test::new("test1");
    test1.init();
    let mut test2 = Test::new("test2");
    test2.init();

    assert_eq!((test1.a(), test1.b().as_str()), ("test1", "test1"));
    std::mem::swap(&mut test1, &mut test2);
    assert_eq!((test2.a(), test2.b().as_str()), ("test1", "test2"));
}

#[test]
fn test3() {
    let mut test1 = Test::new("test1");
    test1.init();
    let mut test2 = Test::new("test2");
    test2.init();

    std::mem::swap(&mut test1, &mut test2);
    assert_eq!((test1.a(), test1.b().as_str()), ("test2", "test1"));
    assert_eq!((test2.a(), test2.b().as_str()), ("test1", "test2"));
}
