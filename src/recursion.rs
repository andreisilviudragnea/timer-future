use futures::future::BoxFuture;
use futures::FutureExt;

async fn _step_one() { /* ... */
}

async fn _step_two() { /* ... */
}

struct _StepOne;
struct _StepTwo;

// This function:
async fn _foo() {
    _step_one().await;
    _step_two().await;
}

// generates a type like this:
enum _Foo {
    First(_StepOne),
    Second(_StepTwo),
}

// So this function:
// async fn recursive() {
//     recursive().await;
//     recursive().await;
// }

// generates a type like this:
// enum Recursive {
//     First(Recursive),
//     Second(Recursive),
// }

fn _recursive() -> BoxFuture<'static, ()> {
    async {
        _recursive().await;
        _recursive().await;
    }
    .boxed()
}
