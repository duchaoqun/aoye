// Rust 编译器拥有 编译时执行函数(Compile-Time Function Execution, CTFE)能力。

// Rust 中固定长度的数组必须在编译期间就知道长度，否则编译器会出错。所以函数 init_len() 函数在编译期间求值。

// 使用 const fn 定义的函数，必须可以确定值，不能存在歧义，和 fn 的区别是，const fn 可以强制编译器在编译期间执行函数。

// 其中 const 一般用于定义全局变量。

const fn init_len() -> usize {
    return 5;
}

fn main() {
    // [0; N] 初始值为0，长度为N的数组。
    let arr = [0; init_len()];
    println!("{:?}", arr)
}

