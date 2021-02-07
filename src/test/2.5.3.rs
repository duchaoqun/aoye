fn main() {
    let number = 42;
    // match 分支使用了模式匹配（Pattern Matching）技术。
    match number{
        // match 分支左边是模式，右边就是执行代码。
        // 所有分支必须返回同一类型，但是左侧的模式可以不同。
        // 单个值
        0 => println!("Origin"),
        // 范围，早期使用 1 ... 3，现在使用 1 ..= 3
        1 ..= 3 => println!("All"),
        // 多个值
        5 |7 | 13 => println!("Bad Luck"),
        // 将值绑定到变量，供右侧代码使用
        n @ 42 => println!("Answer is {}", n),
        // 必须穷尽所有可能，其他情况就用 _ 来处理。
        _ => println!("Common")
    }
}
