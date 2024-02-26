// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let cat = ("Furry McFurson", 3.5); // 建立一個元組 (Tuple) 的方法是寫一個用括號囊括起來的數值列表
    let (name, age) = cat; // 用模式配對和 let 將 cat 拆成兩個個別的變數 name、age。這叫做解構（destructuring），因為它將單一 Tuple 拆成了兩個部分。

    println!("{} is {} years old.", name, age);
}
