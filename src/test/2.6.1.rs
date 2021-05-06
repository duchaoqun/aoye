// bool 类型，类型名是 bool，有两个值，true and false.
fn main() {
    // 不指定类型，自动推断
    let x = true;
    assert_eq!(x as i32, 1);
    // 显示的指定类型
    let y: bool = false;
    let x = 5;
    if x > 1 {
        println!(" x is bigger then 1")
    };
    assert_eq!(y as i32, 0);
}
