// Execute multiple mathematical operations by calling a function that takes
// a closure as an argument.
fn calculator() {
    // Closure that increments a value.
    let increment = |x: i64| x + 1;

    // Closure that decrements a value.
    let decrement = |x: i64| x - 1;

    // Run the closures by calling a function that takes the operation (closure) as an argument
    // and a value on which the operation is applied.
    let result = do_math(5, increment);
    assert_eq!(result, 6);

    let result = do_math(5, decrement);
    assert_eq!(result, 4);
}

// Take a closure "operation" and apply it to the value "value".
fn do_math<C>(value: i64, operation: C) -> i64
where
    C: FnOnce(i64) -> i64,
{
    // Apply the closure to the value and return the result.
    operation(value)
}
