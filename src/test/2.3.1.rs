// Place Expression 可以对某个数据单元的内存进行读写，主要是进行写，语义上来说代表了持久性数据。
// Value Expression 一般只引用了每个存储单元中的数据值，只能读取，语义上来说代表了临时数据。

pub fn temp() -> i32{
    return 1;
}

fn main() {

    let x = &temp();
    // Place Context: 下面四种情况。
    // 赋值或者复合赋值左侧
    // 一元引用表达式的独立操作数
    // 包含隐士借用（引用）的操作数
    // match 判别式或者 let 绑定右侧在使用 ref 模式匹配的时候。

    // Valce Context： 除了上述情况，其他的 Expression 都是值上下文。
    
    // e.g. 左侧调用函数返回的是值他是 Value Expression，这里是 Place Context，编译器会报错。
    // temp() = *x;
}