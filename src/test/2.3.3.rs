fn main() {
    let place1 = "Rust";
    // place1 是 Place Expression 出现在赋值操作符右边，将内存地址转移给了 other
    let other = place1;
    println!("{:?}", place1);
    println!("{:?}", other);

    // place2 拥有 OwenShip，然后 Move 给 other，本身就无效了？
    let place2 = "Rust".to_string();
    // ? place2 是 Value Expression，出现在赋值操作符右边，将所有权给了 other，本身就无效了？？？
    let other = place2;
    // ？ 无效之后就不能用了？
    println!("{:?}", other);
    // println!("{:?}",place2)


    // 借用（Borrow）操作符 &，获取内存地址。
    let a = [1, 2, 3];
    let b = &a;               // 这里的右侧变成了 Place Context
    println!("{:p}", b);      // 宏{:p}打印内存地址

    let mut c = vec![1, 2, 3];
    let d = &mut c;
    d.push(4);
    println!("{:?}", d);
    let e = &42;              // 这里的右侧变成了 Place Context
    assert_eq!(42, *e)        // true,  *e 解释引用操作符 * 按照内存地址取值
}    