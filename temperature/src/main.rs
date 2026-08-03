use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("用法: {} <数值> <单位>", args[0]);
        eprintln!("单位: C 表示摄氏, F 表示华氏");
        std::process::exit(1)
    }
    let value: f64 = match args[1].trim().parse() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("错误, '{}' 不是有效数字", args[1]);
            std::process::exit(1)
        }
    };

    let unit = args[2].to_uppercase();

    if unit == "C" {
        let f = c_to_f(value);
        println!("{:.2}°C = {:.2}°F", value, f);
    } else if unit == "F" {
        let c = f_to_c(value);
        println!("{:.2}°F = {:.2}°C", value, c)
    } else {
        eprintln!("错误: 单位必须是 C 或 F");
        std::process::exit(1);
    }
}

fn c_to_f(v: f64) -> f64 {
    v * 1.8 + 32.0
}

fn f_to_c(v: f64) -> f64 {
    (v - 32.0) * 5.0 / 9.0
}
