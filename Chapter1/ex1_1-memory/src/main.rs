fn listing1_1() {
    let x = 42;
    let y = 43;
    let var1 = &x;
    let mut var2 = &x;
    var2 = &y;

    println!("{} {} {}", var2, *var2, &var2);
}

fn listing1_2() {
    let mut x;
    x = 42;
    let y = &x;
    // x = 43;           // cannot change x while y has borrowed it
    assert_eq!(*y, 42);
    x = 43;
}

fn main() {
    listing1_1();
    listing1_2();
}
