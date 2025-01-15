fn main(){
    let mut s_temp=String::new();
    std::io::stdin()
        .read_line(&mut s_temp)
        .expect("in s_temp err");
    let vec_c_temp:Vec<&str>=s_temp.trim().split_whitespace().collect();
    let a:i64=vec_c_temp[0].parse().expect("in a err");
    let b:i64=vec_c_temp[1].parse().expect("in b err");
    let c:i64=vec_c_temp[2].parse().expect("in c err");
    let mut vec_num:Vec<i64>=Vec::new();
    vec_num.push(a);
    vec_num.push(b);
    vec_num.push(c);
    vec_num.sort();
    for i in vec_num{
        print!("{i} ");
    }
    println!("");
}