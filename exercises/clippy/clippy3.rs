// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.



fn main() {
    // 1. 修复 Option 处理
    let my_option: Option<()> = None;
    if my_option.is_none() {
        // my_option.unwrap();  // 这会导致 panic，应该移除或正确处理
    }

    // 2. 修复数组语法
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // 3. 修复 Vec 处理
    let mut my_vec = vec![1, 2, 3, 4, 5];
    my_vec.clear();  // 替代 resize(0, 5)
    println!("This Vec is empty, see? {:?}", my_vec);

    // 4. 修复变量交换
    let mut value_a = 45;
    let mut value_b = 66;
    // 正确交换两个变量的值
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}