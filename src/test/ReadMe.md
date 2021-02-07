
# 2 语言精要

## 2.2 语句和表达式

- Statement
  - Declaration statement，用于声明各种 Item，像声明变量、静态变量、常量、结构体、函数等，以及通过 extern 和 use 关键字引入的包和模块等。
  - Expression statement，特指以分号结尾的表达式。此类表达式求值结果将会被舍弃，并总是返回 () 类型。
- Expression

## 2.3 变量与绑定

用 let 关键字来创建变量，表示了标识符(Identifier)和值(Value)之间的一种绑定关系。

### 2.3.1 位置表达式(Place Expression)和值表达式(Value Expression)

### 2.3.2 不可变绑定和可变绑定

- 使用 let 声明的 Place Expression 默认不可变，为不可变绑定。
- 可变绑定需要使用 let mut 来声明。

### 2.3.3 所有权(OwnerShip)与借用(Borrow)

- 当 Place Expression 出现在 Value Context 中时，该 Place Expression 将会把内存地址转移给另外一个 Place Expression，这就是所有权转移。

## 2.4 函数的闭包

### 2.4.1 函数定义

函数是通过 fn 关键字定义的，main 函数，代表可执行程序的入口。

### 2.4.2 作用域和生命周期

### 2.4.3 函数指针

### 2.4.4 CTFE 机制

### 2.4.5 闭包

## 2.5 流程控制表达式

### 2.5.1 条件表达式

### 2.5.2 循环表达式

### 2.5.3 match 表达式和模式匹配

### 2.5.4 if let 和 while let 表达式

## 2.6 基本数据类型

### 2.6.1 bool 类型

### 2.6.2 基本数字类型

### 2.6.3 字符类型