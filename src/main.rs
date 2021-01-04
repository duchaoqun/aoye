fn main() {
    // immutable variable: 确定了整型数字，数值不可变，切内存地址唯一绑定给 a 引用。
    let a1 = 123;
    // 不可变变量可以被重新绑定，
    let a1 = 456;
    println!("{0}",a1);

    // mutable variable: 可以改变值的变量，不安全。
    let mut b = 234;
    b = 1234;
    println!("{0}",b);

    // 常量，与不可变变量的区别是不可以被重新绑定。
    const c:i32 = 123;

    // Shadowing, 
    
}
