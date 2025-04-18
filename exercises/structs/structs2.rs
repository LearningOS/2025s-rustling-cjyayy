// structs2.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs2` or use the `hint` watch subcommand for a


// structs2.rs
#[derive(Debug, Clone)] // 确保添加了 Clone
struct Order {
    name: String,
    year: u32,
    made_by_phone: bool,
    made_by_mobile: bool,
    made_by_email: bool,
    item_number: u32,
    count: u32,
}

// 关键修正：添加 pub 使函数对外可见
pub fn create_order_template() -> Order {
    Order {
        name: String::from("Bob"),
        year: 2019,
        made_by_phone: false,
        made_by_mobile: false,
        made_by_email: true,
        item_number: 123,
        count: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*; // 正确导入父模块内容

    #[test]
    fn your_order() {
        let order_template = create_order_template(); // 现在可以找到函数了
        let your_order = Order {
            name: String::from("Hacker in Rust"),
            count: 1,
            ..order_template.clone()
        };

        assert_eq!(your_order.name, "Hacker in Rust");
        assert_eq!(your_order.year, order_template.year);
        assert_eq!(your_order.made_by_phone, order_template.made_by_phone);
        assert_eq!(your_order.made_by_mobile, order_template.made_by_mobile);
        assert_eq!(your_order.made_by_email, order_template.made_by_email);
        assert_eq!(your_order.item_number, order_template.item_number);
        assert_eq!(your_order.count, 1);
    }
}