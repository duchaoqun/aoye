// 如下 for ... in Expression 的本质是迭代器。 其中 1..101是一个Range类型，它是一个迭代器。 
fn main() {
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz")
        } else {
            println!("{}", n);
        }
    }
}



// 样例代码: Rust 无限循环需要使用 loop，不能使用 while true
fn while_true(x: i32) -> i32 {

    // while true Expression 返回的结果是 () 单元值，该函数要求返回的是 i32 类型值。
    // 因为 Rust 编译器在对 while 循环做流分析（Flow Sensitive）的时候，不会检查循环条件，编译器会认为 while 条件可真可假，循环体里面的表达式也会忽略。
    // 编译器只知道 while Expression 返回的是 () 单元值，其他什么都不知道。这都是由于 CTFE 功能的限制，只能等 CTFE 功能完善了。
    while true {
        return x + 1;
    }
    x
}

fn main() {
    let y = while_true(5);
    assert_eq!(y,6);
}