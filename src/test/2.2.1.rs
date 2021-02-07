// Rust 编译器再解析代码的时候，如果碰到分号，就会继续往后面执行；
// 如果碰到 Statement，则执行 Statement；
// 如果碰到 Expression，则会对 Expression 求值，如果分号后面什么都没有，就会补上 () 单元值。

// 当遇到函数的时候，会将函数体的花括号识别为块表达式(Block Expression)。
// 块表达式是由一对花括号和一系列表达式组成的，它总是返回块中最后一个表达式的值。

// extern crate std;
// use std::prelude::v1::*;

fn main() {
    // 没有参数，没有返回类型。
    // 可以省略返回类型的声明，默认返回 () 类型。
    pub fn answer() -> () {
        let a = 40;
        let b = 2;
        assert_eq!(sum(a, b), 42);  // 这个分号可以省略吗？
    }

    pub fn sum(a: i32, b: i32) -> i32 {
        // Expression，这里是Expression，如果以分号结尾，它就是Expression statement，那样的话就会返回 () 类型，所以这里不能添加分号。
        a + b
    }

    answer();
}
