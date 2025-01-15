fn p5714(){
    let mut s_temp=String::new();
    std::io::stdin()
        .read_line(&mut s_temp)
        .expect("in s_temp err");
    let vec_c_temp:Vec<&str>=s_temp.trim().split_whitespace().collect();
    let m:f64=vec_c_temp[0].parse().expect("in m err");
    let h:f64=vec_c_temp[1].parse().expect("in h err");

    let bmi=m/(h.powi(2));
    if bmi <18.5{
        println!("Underweight");
    }
    else if bmi<24.0 {
        println!("Normal");
    }
    else{
        let mut formatted_bmi = format!("{:6}", bmi);
        formatted_bmi=formatted_bmi[..7].to_string();
        let end_temp:Vec<char>=formatted_bmi.chars().collect();
        let formatted_bmi=formatted_bmi.trim_end_matches('0');
        if (end_temp[end_temp.len()-1] as u8) > b'5'{
            formatted_bmi[end_temp.len()]+=1;
        }
        println!("{}\nOverweight", formatted_bmi);
    }
}