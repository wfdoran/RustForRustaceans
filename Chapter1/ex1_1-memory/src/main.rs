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

fn listing1_3() {
    let x1 = 42;
    let y1 = Box::new(84);
    {
        let z = (x1, y1);
    }
    let x2 = x1;
    println!("{} {}", x1, x2);
    // let y2 = y1;        // y1 went away when z was dropped
}

fn listing1_4() {
    fn cache(input: &i32, sum: &mut i32) {
        *sum = *input + *input;
    }

    let a = 5;
    let mut b: i32 = 0;

    cache(&a, &mut b);

    assert_eq!(b, 2*a);
}

fn listing1_5() {
    fn noalias(input: &i32, output: &mut i32) {
        // since input is &T not &mut T, compiler can safely turn this into if{}else{}
        if *input == 1 {
            *output = 2;
        } 
        if *input != 1 {
            *output = 3;
        }
    }

    let mut a: i32 = 0;
    noalias(&1, &mut a);
    assert_eq!(a, 2);
    noalias(&23, &mut a);
    assert_eq!(a, 3);
}

fn listing1_6() {
    let x = 42;
    let mut y = &x;
    let z = &mut y;

    println!("{}", z);

    let x2 = 24;
    *z = &x2;
    println!("{}", y);
}

fn listing1_7() {
    fn replace_with_84(s: &mut Box<i32>) {
        let was = std::mem::take(s);
        *s = was;
        let mut r = Box::new(84);
        std::mem::swap(s, &mut r);
        assert_ne!(*r, 84);
    }

    let mut s = Box::new(42);
    println!("{}", s);
    replace_with_84(&mut s);
    println!("{}", s);

}

fn main() {
    listing1_1();
    listing1_2();
    listing1_3();
    listing1_4();
    listing1_5();
    listing1_6();
    listing1_7();
}
