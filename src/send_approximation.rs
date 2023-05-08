use std::rc::Rc;

#[derive(Default)]
struct NotSend(Rc<()>);

async fn bar() {}

fn require_send(_: impl Send) {}

#[test]
fn test1() {
    async fn foo() {
        NotSend::default();
        bar().await;
    }

    require_send(foo());
}

#[test]
fn test2() {
    async fn foo() {
        // let x = NotSend::default();
        bar().await;
    }

    require_send(foo());
}

#[test]
fn test3() {
    async fn foo() {
        {
            let _x = NotSend::default();
        }
        bar().await;
    }

    require_send(foo());
}
