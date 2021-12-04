pub mod sub_a;
pub mod sub_b;

const MAX_POINTS: u32 = 100_000;
// can not use let
// let xx: u32 = 100_001;

pub fn run() {
    println!("hello vars");
    sub_a::func_a();
    sub_b::func_b();
    let x1 = 5;
    // x1 = 6;
    let mut x2 = 5;
    println!("{} {}", x1, x2);
    x2 = 6;
    println!("{} {}", x1, x2);

    let _i1 = 10;
    let _f1 = 3.1;

    println!("max points: {}", MAX_POINTS);
    println!("memory address const of MAX_POINTS: {:p}", &MAX_POINTS);

    let i2: i64 = 1;
    let i3: i64 = 2;
    println!("stack address of i2: {:p}", &i2);
    println!("stack address of i3: {:p}", &i3);

    let y = 5;
    println!("stack address of  y: {:p}, value is {}", &y, y);
    let y = y + 1;
    println!("stack address of  y: {:p}, value is {}", &y, y);
    let y = y * 2;
    println!("stack address of  y: {:p}, value is {}", &y, y);
    {
        let y = 0;
        println!("stack address of  y: {:p}, value is {}", &y, y);
    }
    let y = y * 2;
    println!("stack address of  y: {:p}, value is {}", &y, y);
    // show environment size
    println!("usize bit: {}", usize::BITS);
    printTuple();
    printArray();
    helloString();
}

fn printTuple() {
    println!("----- printTuple -----");

    let t1 = (500, 12.1, "dummy");
    let (x1, y1, z1) = t1;
    println!("tuple item: ({}, {}, {})", x1, y1, z1);
    println!("tuple item: ({}, {}, {})", t1.0, t1.1, t1.2);

    let mut t2 = ((0, 1), (2, 3));
    println!("tuple item: {:?}", t2);
    let ((ref mut t2x1_ptr, ref mut t2y1_ptr), _) = t2;
    println!("tuple item pointer: (({:p}, {:p}), _)", t2x1_ptr, t2y1_ptr);
    *t2x1_ptr = 5;
    *t2y1_ptr = -5;
    println!("tuple item: {:?}", t2);

    println!("----- printTuple -----");
}

fn printArray() {
    println!("----- printArray -----");
    let a1 = [1,2, 3, 4,5];
    let a2 = [0; 10];
    println!("{:?}, {:?}, {}, {}", a1, a2, a1[2], a1[3]);


    let s1 = "helloこんにちは挨拶";
    let s2 = "hello";
    println!("stack address of s1: {:p}", &s1);
    println!("stack address of s2: {:p}", &s2);
    println!("static address of s1: {:?}", s1.as_ptr());
    println!("static address of s2: {:?}", s2.as_ptr());
    println!("length of s1: {}", s1.len());
    println!("length of s2: {}", s2.len());
    println!("----- printArray -----");    
}

fn helloString() {
    println!("----- helloString -----");    
    let mut s1 = String::from("hello");
    let mut s2 = String::from("helloworld");

    println!("stack address of s1: {:p}", &s1);
    println!("stack address of s2: {:p}", &s2);

    println!("heap address of s1: {:?}", s1.as_ptr());
    println!("heap address of s2: {:?}", s2.as_ptr());

    println!("length of s1: {}", s1.len());
    println!("length of s2: {}", s2.len());

    println!("capacity of s1: {}", s1.capacity());
    println!("capacity of s2: {}", s2.capacity());

    s1.push_str("_new1");
    s2.push_str("_new2");
    println!("{}, {}", s1, s2);
    println!("----- helloString -----");    
}
