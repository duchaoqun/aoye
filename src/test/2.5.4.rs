// if let 和 whild let 再某些场合代替 match 表达式。

fn main() {
    let boolean = true;
    let mut binary = 0;
    // 左侧 true 是模式，右侧 boolean 为要匹配的值
    if let true = boolean {
        binary = 1;
    }
    assert_eq!(binary, 1);
}



// 再使用循环的时候，while let 可以简化match代码。
fn main() {
    let mut v = vec![1,2,3,4,5];
    loop {
        match v.pop(){
            Some(x) => println!("{}",x),
            None => break,
        }
    }
}
// 再使用循环的时候，while let 可以简化match代码。
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    // 左侧 Some(x)为匹配模式，它会匹配 右侧pop方法返回的Option 类型结果，并自动创建绑定供 pringln! 宏语句使用，如果数组中的值取空，自动跳出循环。
    while let Some(x) = v.pop() {
        println!("{}", x);
    }
}
