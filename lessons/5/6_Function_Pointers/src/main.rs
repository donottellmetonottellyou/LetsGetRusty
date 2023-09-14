fn main() {
    let ten = 10;
    // If are_both_true uses function pointers, error.
    let greater_than = |x: &i32| *x > ten;

    dbg!(are_both_true(greater_than, less_than, &9));
}

fn less_than(x: &i32) -> bool {
    *x < 20
}

fn are_both_true<T, U, V>(f1: T, f2: U, item: &V) -> bool
where
    T: Fn(&V) -> bool,
    U: Fn(&V) -> bool,
{
    f1(item) && f2(item)
}
