use std::pin::Pin;

use futures::Future;

async fn calculator() {
    let increment = |x: i64| Box::pin(async move { x + 1 }) as Pin<Box<dyn Future<Output = i64>>>;

    let decrement = |x: i64| Box::pin(async move { x - 1 }) as Pin<Box<dyn Future<Output = i64>>>;

    let result = do_math(5, increment).await;
    assert_eq!(result, 6);

    let result = do_math(5, decrement).await;
    assert_eq!(result, 4);
}

async fn do_math<C>(value: i64, operation: C) -> i64
where
    C: FnOnce(i64) -> Pin<Box<dyn Future<Output = i64>>>,
{
    operation(value).await
}
