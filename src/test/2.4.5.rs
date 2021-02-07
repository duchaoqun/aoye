// 可以像匿名函数一样被调用。

// 可以捕获上下文环境中的资源变量。

fn main() {
    let out = 42;

    // 如下是错误的，函数不能使用外部的 out 变量。
    // fn add(i:i32, j:i32) -> i32{
    //     i + j + out;
    // }
    fn add(i: i32, j: i32) -> i32 {
        i + j
    };

    let closure_annotated = |i: i32, j: i32| -> i32 { i + j + out };

    let i = 1;
    let j = 2;
    let r1 = add(i, j);
    let r2 = closure_annotated(i,j);
}



// 样例代码：闭包作为参数的情况。

// 参数是一个泛型参数，该泛型接收  Fn() -> i32 trait 的限定，只允许实现了 Fn -> i32 的类型作为参数。

// Rust 中闭包实际上就是由一个匿名结构体和trait来组合实现的。
fn math<F: Fn() -> i32>(op: F) -> i32 {
    op()
}

fn main() {
    let a = 2;
    let b = 3;
    assert_eq!(math(|| a + b), 5);
    assert_eq!(math(|| a * b), 6);
}



// 样例代码：闭包作为返回值的情况，impl Fn(i32) -> i32 表示实现 Fn(i32) -> i32 的类型。

fn two_times_impl() -> impl Fn(i32) -> i32{
    let i = 2;
    // 这里使用 move 关键字将 i 的所有权交给闭包来返回，否则编译器会报错。
    move |j| j * i
}
fn main() {
    let result = two_times_impl();
    assert_eq!(result(2),4);
}
