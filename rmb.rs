use std::collections::HashMap;
use std::io::{self, Write};
use std::process;

/// 数字映射
/// ```
/// 0  1  2  3  4  5  6  7  8  9
/// |  |  |  |  |  |  |  |  |  |
/// 零 壹 贰 叁 肆 伍 陆 柒 捌  玖
/// ```
const NUM_ARR: [&str; 10] = ["零", "壹", "贰", "叁", "肆", "伍", "陆", "柒", "捌", "玖"];

/// 单位映射
///
///
const UNIT_ARR: [&str; 14] = [
    "分", "角", "元", "拾", "佰", "仟", "萬", "拾", "佰", "仟", "亿", "拾", "佰", "仟",
];

/// 金额转换人民币大写
///
pub fn to_rmb(amount: f64) -> String {
    // 单位映射 (tuple数组转map)
    let unit_map: HashMap<u8, &str> = [
        (0, "分"),
        (1, "角"),
        (2, "元"),
        (3, "拾"),
        (4, "佰"),
        (5, "仟"),
        (6, "萬"),
        (7, "拾"),
        (8, "佰"),
        (9, "仟"),
        (10, "亿"),
        (11, "拾"),
        (12, "佰"),
        (13, "仟"),
    ]
    .iter()
    .cloned()
    .collect();

    let mut cent_amount = (amount * 100.0).round() as u64;
    let mut pointer: u8 = 0; // 单位指针
    let mut remainder: u32;
    let mut last_remainder: u32 = 0; // 上一个余数

    let mut result = String::new();
    while cent_amount > 0 {
        remainder = (cent_amount % 10) as u32; // 余数
        cent_amount = cent_amount / 10; // 缩小10倍
        /* println!(
            "amount: {}, remainder: {}, pointer: {}",
            cent_amount, remainder, pointer
        ); */

        if remainder == 0 {
            if pointer == 2 || pointer == 6 || pointer == 10 {
                // 余数为0, 元、萬、亿单位需要保留
                // result = String::from(*unit_map.get(&pointer).unwrap()) + result.as_str();
                result = String::from(UNIT_ARR[pointer as usize]) + result.as_str();
            } else {
                if last_remainder > 0 {
                    // 余数为0, 上一个余数也为0, 避免零仟零佰
                    result = String::from(NUM_ARR[remainder as usize]) + result.as_str();
                }
            }
        } else {
            /* result = String::from(NUM_ARR[remainder as usize])
                + *unit_map.get(&pointer).unwrap()
                + result.as_str(); */
            result = String::from(NUM_ARR[remainder as usize])
                + UNIT_ARR[pointer as usize]
                + result.as_str();
        }
        last_remainder = remainder;
        pointer += 1; // 从右往左
        if pointer == 2 && "" == result {
            // 无角、分尾数
            result = String::from("整");
        }
        // println!("{}", result);
    }
    return result;
}

fn main() {
    // println!("Hello, world!");
    // println!("{}", to_rmb(12345.678))
    loop {
        let mut input = String::new();
        print!("Please input(0退出): ");
        io::stdout().flush().unwrap();
        // io::stdin().read_line(&mut input).expect("未输入正确的数字串");
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if "0" == input.trim() {
                    process::exit(0);
                }
                // 如果想使一个可恢复错误按不可恢复错误处理，
                // Result 类提供了两个办法：unwrap() 和 expect(message: &str)
                // println!("{}", to_rmb(input.trim().parse().unwrap()));
                if let Ok(amount) = input.trim().parse::<f64>() {
                    println!("{}", to_rmb(amount))
                } else {
                    println!("Err: 未输入正确的数字串")
                }
            }
            Err(error) => println!("Err: {}", error),
        }
    }
}
