// Rust 中，函数是一等公民。



// 第一个例子
// Fn-pointer Type 类型：第一个参数 op: fn(i32, i32) -> i32
pub fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    op(a, b)
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn product(a: i32, b: i32) -> i32 {
    a * b
}

fn main() {
    let a = 2;
    let b = 3;
    assert_eq!(math(sum, a, b), 5);
    assert_eq!(math(product, a, b), 6);
}



// 第二个例子
// 返回一个值
fn is_true() -> bool {
    true
}

// 返回一个函数，直接返回 is_true 的指针
fn true_maker() -> fn() -> bool {
    // 直接使用函数名，其实使用的是函数类型 Fn-Item Type
    is_true
}

fn main() {
    // 返回的指针加上()，就可以执行对应的函数。
    assert_eq!(true_maker()(), true);
}
