// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number = 3; // don't rename this variable
                    // Shadowing: 透過再次對該變數用 let，可以賦予不同 type 的值
    println!("Number plus two is : {}", number + 2);
}
