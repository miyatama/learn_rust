pub fn run() {
    let res1 = division_option(5.0, 2.0);
    print_division_option(res1);

    let res2 = division_option(5.0, 0.0);
    print_division_option(res2);

    let res3 = division_result(5.0, 2.0);
    print_divition_result(res3);

    let res3 = division_result(5.0, 0.0);
    print_divition_result(res3);

    let arr0 = [0, 1, 2];
    let arr0some = some(&arr0);
    print_some(arr0some);

    let arr1 = [1];
    let arr1some = some(&arr1);
    print_some(arr1some);
}

fn division_option(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    }else{ 
        Some(x / y)
    }
}

fn print_division_option(option: Option<f64>) {
    match option {
        Some(x) => println!("Result: {:3}", x),
        None => println!("Not allowed"),
    }
}

fn division_result(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err(String::from("Not allowed"))
    } else {
        Ok(x / y)
    }
}

fn print_divition_result(result: Result<f64, String>) {
    match result {
        Err(x) => println!("{}",x),
        Ok(x) => println!("result is {:3}", x),
    }
}

fn some(a: &[i32]) -> Option<i32> {
    let a0 = a.get(0)?;
    let a1 = a.get(1)?;
    let a2 = a.get(2)?;
    Some(a0 + a1 + a2)
}

fn print_some(result: Option<i32>) {
    match result {
        None => println!("out of index"),
        Some(x) => println!("result is {}", x),
    }
}