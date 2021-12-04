pub fn run() {
    println!("---- ownership start -----");
    canNotMoveOwnership();
    canMoveOwnership();
    canMoveOwnershipUsingClone();
    println!("---- ownership end -----");
}

fn canNotMoveOwnership() {
    println!("----- canNotMoveOwnership() start -----");
    let s1 = String::from("hello");
    let s2 = s1;

    // can not access s1.owner ship moved to s2.
    // println!("s1 is {}.s2 is {}.", s1, s2);
    println!("s2 is {}.", s2);
    println!("----- canNotMoveOwnership() end -----");
}

fn canMoveOwnership() {
    // Copy traitを持つデータ型は所有権が存在しない
    let i1: u32 = 1;
    let i2 = i1;
    println!("i1 = {}.i2 = {}.", i1, i2);
    println!("stack address of i1: {:p}", &i1);
    println!("stack address of i2: {:p}", &i2);

    let sl1 = "hello";
    let sl2 = sl1;
    println!("sl1 = {}.sl2 = {}.", sl1, sl2);
    println!("stack address of sl1: {:p}", &sl1);
    println!("stack address of sl2: {:p}", &sl2);

}

fn canMoveOwnershipUsingClone() {
    println!("----- canMoveOwnershipUsingClone() start -----");
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("stack address of s1: {:p}", &s1);
    println!("stack address of s2: {:p}", &s2);
    println!("heap address of s1: {:?}", s1.as_ptr());
    println!("heap address of s2: {:?}", s2.as_ptr());

    println!("s1: {}.s2: {}.", s1, s2);

    let s3 = String::from("hello world.");
    println!("stack address of s3: {:p}", &s3);
    println!("heap address of s3: {:?}", s3.as_ptr());
    println!("length of s3: {}", s3.len());
    println!("capacity of s3: {}", s3.capacity());
    take_ownership(s3);
    // after move ownership
    // println!("after take_ownership: {}", s3);

    let s4 = String::from("hello world.");
    println!("stack address of s4: {:p}", &s4);
    println!("heap address of s4: {:?}", s4.as_ptr());
    println!("length of s4: {}", s4.len());
    println!("capacity of s4: {}", s4.capacity());
    let s5 = take_giveback_ownership(s4);
    println!("stack address of s5: {:p}", &s5);
    println!("heap address of s5: {:?}", s5.as_ptr());
    println!("length of s5: {}", s5.len());
    println!("capacity of s5: {}", s5.capacity());

    let s6 = String::from("hello world");
    let i1 = calculate_length(&s6);
    println!("s6 is {}.length is {}.", s6, i1);

    let mut s7 = String::from("stop");
    println!("before: {}.", s7);
    change(&mut s7);
    println!("after: {}.", s7);

    // immutable ref
    let s8 = String::from("the world!");
    let r1 = &s8;
    let r2 = &s8;
    println!("{}, {}, {}", s8, r1, r2);

    // can not mix a mutable and imutable
    // let mut s9 = String::from("stop");
    // let r3 = &s9;
    // let r4 = &mut s9;
    // println!("{}, {}, {}", s9, r3, r4);

    let mut s10 = String::from("world");
    let r5 = &mut s10;
    // println!("{}", s10);
    println!("{}", r5);
    println!("{}", s10);

    let mut s11 = String::from("dio");
    let r6 = &s11;
    let r7 = &s11;
    println!("r6: {}, r7: {}", r6, r7);
    let r8 = &mut s11;
    println!("r8: {}", r8);
    *r8 = String::from("Jojo");
    println!("s11: {}", s11);

    println!("----- canMoveOwnershipUsingClone() end -----");
}

fn take_ownership(s: String) {
    println!("take_ownership: {}", s);
    println!("take_ownership stack address of s: {:p}", &s);
    println!("take_ownership heap address of s: {:?}", s.as_ptr());
    println!("take_ownership length of s: {}", s.len());
    println!("take_ownership capacity of s: {}", s.capacity());
}

fn take_giveback_ownership(s: String) -> String {
    println!("take_giveback_ownership: {}", s);
    println!("take_giveback_ownership stack address of s: {:p}", &s);
    println!("take_giveback_ownership heap address of s: {:?}", s.as_ptr());
    println!("take_giveback_ownership length of s: {}", s.len());
    println!("take_giveback_ownership capacity of s: {}", s.capacity());
    s
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" the world!")
}