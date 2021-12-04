enum List {
    Node(i32, Box<List>), 
    Nil
}
pub fn run() {
    println!("----- stack_heap -----");
    // stack over flow
    // let a1: [u8; 7000000] = [1; 7000000];
    let mut v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];
    let mut v3 = vec![9, 10];
    println!("stack address of v1: {:p}", &v1);
    println!("stack address of v2: {:p}", &v2);

    println!("heap address of v1: {:?}", v1.as_ptr());
    println!("heap address of v2: {:?}", v2.as_ptr());

    println!("length of v1: {}", v1.len());
    println!("length of v2: {}", v2.len());

    println!("capacity of v1: {}", v1.capacity());
    println!("capacity of v2: {}", v2.capacity());
    
    v1.insert(1, 10);
    println!("{:?}", v1);
    println!("length of v1: {}", v1.len());
    println!("capacity of v1: {}", v1.capacity());
    // v1.remove(0);
    v1.append(&mut v3);
    println!("{:?}", v1);
    println!("{:?}", v3);
    println!("----- stack_heap -----");

    helloBoxPointer();
}

fn helloBoxPointer() {
    println!("----- helloBoxPointer -----");
    let t1:(u64, String) = (10, String::from("hello"));
    println!("stack address of t1: {:p}", &t1);
    println!("heap address of t1.1: {:p}", t1.1.as_ptr());
    println!("length of t1.1: {:}", t1.1.len());
    println!("capacity of t1.1: {:}", t1.1.capacity());

    let mut b1 = Box::new(t1);
    (*b1).1 += " world";
    println!("({}, {})", b1.0, b1.1);
    println!("stack address of b1: {:p}", &b1);
    println!("heap address of t1: {:p}", b1);
    println!("----- helloBoxPointer -----");
}