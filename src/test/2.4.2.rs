// Lexical Scope：Rust的作用域是静态作用域，即词法作用域(Lexical Scope)
// LifeTime：从使用 let 声明创建变量绑定开始，到超出词法作用域的范围时结束。

fn main() {
    let v = "hello world!";
    assert_eq!(v,"hello world!");
    // 连续用 let 定义同名变量的做法叫做变量遮蔽(Variable Shadow)，值由第二个定义决定。
    let v = "hello Rust!";
    assert_eq!(v,"hello Rust!");
    {
        // 花括号内的空间，是一块新的作用域，不同的词法作用域内定义的变量，拥有不同的 LifeTime，这里的内容不会影响到作用域外部的代码。
        let v = "hello World!";
        assert_eq!(v, "hello World!");
    }
    // 如下语句正常，不会受到影响。
    assert_eq!(v,"hello Rust!");
}