use std::{
    any::type_name,
    ffi::{CStr, CString},
};

fn main() {
    hello_const_string();
    hello_string();
    hello_slice();
    hello_cstring();
    why_need_anpasand();
}

fn hello_string() {
    println!("------------------------------");
    println!("hello_string");
    let mut s1 = String::from("hello");
    let mut s2 = String::from("helloworld");

    println!("String type: {}", type_of(&s1));
    println!("stack address of s1: {:p}", &s1);

    println!("heap address type: {}", type_of(&s1.as_ptr()));
    println!("heap address of s1: {:?}", s1.as_ptr());
    println!("heap address of s2: {:?}", s2.as_ptr());

    println!("length of s1: {}", s1.len());
    println!("length of s2: {}", s2.len());

    println!("capacity of s1: {}", s1.capacity());
    println!("capacity of s2: {}", s2.capacity());

    s1.push_str("_new1");
    s2.push_str("_new2");
    println!("{}, {}", s1, s2);
}

fn hello_const_string() {
    println!("------------------------------");
    println!("hello_const_string");
    let s1 = "abcDEFghi";
    let s1_ptr = &s1;
    let s2 = s1.to_string();
    let s2_ptr = &s2;
    // *s2 is Deref suger
    let s2_value = &*s2;

    println!("const string type: {}", type_of(&s1));
    println!("const string value: {}", &s1);
    println!("const string reference type: {}", type_of(&s1_ptr));

    println!("to_string type: {}", type_of(&s2));
    println!("to_string reference type: {}", type_of(&s2_ptr));
    println!("to_string deref reference type: {}", type_of(&s2_value));
}

fn hello_slice() {
    println!("------------------------------");
    println!("hello_slice");
    let mut s = String::from("abcDEFghi");
    let len = s.len();
    let s_address = &s;
    let slice = &s[0..len];

    println!("String type: {}", type_of(&s));
    println!("String reference type: {}", type_of(&s_address));
    println!("string stack address: {:p}", &s);

    println!("String pointer type: {}", type_of(&s.as_ptr()));
    println!("string heap address: {:?}", s.as_ptr());

    println!("slice type: {}", type_of(&slice));
    println!("slice address: {:p}", &slice);
}

fn hello_cstring() {
    println!("------------------------------");
    println!("hello_cstring");
    let mut s1 = CString::new("hello").unwrap();
    let s1_ptr = s1.as_ptr();

    println!("CString type: {}", type_of(&s1));
    println!("CString stack address: {:p}", &s1);
    println!("CString value: {:?}", &s1);
    println!("CString ptr type: {}", type_of(&s1_ptr));
    println!("heap address: {:?}", s1_ptr);

    let mut cstr1 = unsafe { CStr::from_ptr(s1_ptr) };
    println!("CStr type: {}", type_of(&cstr1));
    println!("CStr value: {:?}", &cstr1);
}

fn why_need_anpasand() {
    println!("------------------------------");
    println!("why_need_anpasand");
    let mut s = String::from("hello");
    let s_address = &s;

    println!("String type: {}", type_of(&s));
    println!("String reference type: {}", type_of(s_address));
    println!("String reference type &: {}", type_of(&s_address));

    println!("String defer value: {}", s_address);
    println!("String defer value *: {}", *s_address);
    println!("String defer value &: {}", &s_address);

    /* expected `str`, found `&str
    match *s{
        "hello" => {
            println!("value match");
        }
        _ => {
            println!("value unmatch");
        }
    }
     */
    /* expected `String`, found `&str`
    match *s_address {
        "hello" => {
            println!("value match");
        }
        _ => {
            println!("value unmatch");
        }
    }
     */
    /* expected `String`, found `&str`
    match s {
        "hello" => {
            println!("value match");
        }
        _ => {
            println!("value unmatch");
        }
    }
    */
    /* expected `*const u8`, found `&str`
    match s.as_ptr() {
        "hello" => {
            println!("value match");
        }
        _ => {
            println!("value unmatch");
        }
    }
    */
    /* expected `&String`, found `&str`
    match &s {
        "hello" => {
            println!("value match");
        }
        _ => {
            println!("value unmatch");
        }
    }
     */

    // 値の参照を外して借用する
    match &*s {
        "hello" => {
            println!("value match");
        }
        _ => {
            println!("value unmatch");
        }
    }
    // スライス(所有権なしの参照)
    match &s[0..s.len()] {
        "hello" => {
            println!("value match");
        }
        _ => {
            println!("value unmatch");
        }
    }
    println!("{}", s);
    // コンパイル時にサイズが不定となる為、strを引数にとることはできない
    // need_str(*s);
}

/*
fn need_str(val: str) {
    println!("need str: {}", val);
}
 */

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
