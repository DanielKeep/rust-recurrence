#[macro_use] extern crate recurrence;

#[test]
fn test_fib_10() {
    let fib = recurrence![ fib[n]: f64 = 0.0, 1.0 ... fib[n-1] + fib[n-2] ];
    let fib_10: Vec<_> = fib.take(10).collect();
    assert_eq!(&*fib_10, &[0.0, 1.0, 1.0, 2.0, 3.0, 5.0, 8.0, 13.0, 21.0, 34.0]);
}
