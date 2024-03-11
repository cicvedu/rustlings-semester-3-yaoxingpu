/*
 * @Author: yaoxingpu yaoxpu@163.com
 * @Date: 2024-03-11 11:52:39
 * @LastEditors: yaoxingpu yaoxpu@163.com
 * @LastEditTime: 2024-03-11 19:07:22
 * @FilePath: /rustlings-semester-3-yaoxingpu/exercises/move_semantics/move_semantics5.rs
 * @Description:
 *
 */
// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.



fn main() {
    /* 在 Rust 中，你不能同时拥有两个可变引用指向同一个值。
    在你的代码中，你试图创建两个可变引用 y 和 z 指向 x。
    这是不允许的，因为 Rust 的借用规则禁止在同一个作用域内拥有多个可变引用指向同一数据。 */
    let mut x = 100;
    {
        let y = &mut x;
        *y += 100;
    }

    {
        let z = &mut x;
        *z += 1000;
    }
    assert_eq!(x, 1200);
}
