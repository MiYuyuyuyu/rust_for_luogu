fn p5714() {
    let mut s_temp = String::new();
    std::io::stdin()
        .read_line(&mut s_temp)
        .expect("in s_temp err");
    let vec_c_temp: Vec<&str> = s_temp.trim().split_whitespace().collect();
    let m: f64 = vec_c_temp[0].parse().expect("in m err");
    let h: f64 = vec_c_temp[1].parse().expect("in h err");

    let bmi = m / (h.powi(2));
    if bmi < 18.5 {
        println!("Underweight");
    } else if bmi < 24.0 {
        println!("Normal");
    } else {
        // 计算整数部分的位数
        let integer_part = bmi.trunc();
        let integer_digits = integer_part.to_string().len();
        // 计算小数部分的位数
        let decimal_digits = 6 - integer_digits;
        // 直接在 println! 中使用格式化字符串
        println!("{:.1$}\nOverweight", bmi, decimal_digits);
    }
}