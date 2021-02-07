// 再使用循环的时候，while let 可以简化match代码。
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    while let Some(x) = v.pop() {
        println!("{}", x);
    }
}
