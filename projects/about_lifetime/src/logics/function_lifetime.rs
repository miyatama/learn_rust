pub fn run_func_lifetime() {
    let str1 = String::from("エリザベス");
    let str2 = "モリザベス";
    let result = longest(str1.as_str(), str2);
    println!("{}", result);

    let a = 50;
    let b = 51;
    let c = 49;
    let z1 = process01(&a, &b, &c);
    println!("z1: {}", z1);
    let z2 = process02(&a, &b, &c);
    println!("z2: {}", z2);
    let z3 = process03(&a);
    println!("z3: {}", z3);
    let z4 = process04(&a);
    println!("z4: {}", z4);
}

// 小文字の連続ならOKなので、frierenでも大丈夫
fn longest<'frieren>(x: &'frieren str, y: &'frieren str) -> &'frieren str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 各引数は独自のライフタイムを得る(戻り値を&i32へすると、ライフタイム指定が必要になる)
fn process01<'izen, 'himmel, 'frieren>(x: &'izen i32, y: &'himmel i32, z: &'frieren i32) -> i32 {
    *x + *y + *z
}

fn process02<'frieren>(x: &'frieren i32, _y: &'frieren i32, _z: &'frieren i32) -> &'frieren i32 {
    x
}

// 入力ライフタイム引数が一つなら出力ライフタイム引数も同じになる
fn process03(x: &i32) -> &i32 {
    x
}

fn process04<'frieren>(x: &'frieren i32) -> &'frieren i32 {
    x
}
