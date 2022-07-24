use futures::{future::BoxFuture, FutureExt};

async fn calculator() {
    let increment = |x: i64| async move { x + 1 }.boxed();

    let decrement = |x: i64| async move { x - 1 }.boxed();

    let result = do_math(5, increment).await;
    assert_eq!(result, 6);

    let result = do_math(5, decrement).await;
    assert_eq!(result, 4);
}

async fn do_math<C>(value: i64, operation: C) -> i64
where
    C: FnOnce(i64) -> BoxFuture<'static, i64>,
{
    operation(value).await
}
