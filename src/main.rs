<<<<<<< HEAD
fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}
=======
// 固定取值范围的类型

// Unsigned Integer and Signed Integer

// u8，取值范围 2 - (2^8-1)，通常表示字节序列，在文件I/O和网络I/O读取数据流的时候使用。

// 动态取值范围的类型

// 浮点数
fn main() {
    let num = 42u32;
    let num: u32 = 42;
    let num = 0x2A;          // 十六进制
    let num = 0o106;         // 八进制
    let num = 0b1001_1011;   // 二进制
    assert_eq!(b'*', 42u8);  // 字节字面量
    assert_eq!(b'\'', 39u8);
    let num = 3.1415926f64;

    assert_eq!(-3.14, -3.14f64);
    assert_eq!(2.,2.0f64);
    assert_eq!(2e4,20000f64);
    println!("{:?}", std::f32::INFINITY);
    println!("{:?}", std::f32::NEG_INFINITY);
    println!("{:?}", std::f32::NAN);
    println!("{:?}", std::f32::MIN);
    println!("{:?}", std::f32::MAX);
}
>>>>>>> 944eac230b49f41fee56bbf512269b4d1c9e8ae6
