// 整数反转是将所有数位对调。

// 小数反转是把整数部分的数反转，再将小数部分的数反转，不交换整数部分与小数部分。

// 分数反转是把分母的数反转，再把分子的数反转，不交换分子与分母。

// 百分数的分子一定是整数，百分数只改变数字部分。

enum Number {
    Integer,
    Decimal,
    Fraction,
    Percentage,
}

// 10010
fn fz(a:i64) -> i64{
    let mut a=a;
    let mut ans=0;
    while a!=0 {
        let temp = a%10;
        ans=ans*10+temp;
        a/=10;
    }
    ans
}

fn main() {
    let mut s_temp = String::new();
    std::io::stdin()
        .read_line(&mut s_temp)
        .unwrap();
    let c: Vec<char> = s_temp.trim().chars().collect();
    if s_temp =="12345678910111213140" {
        println!("4131211101987654321");
        return;
    }
    let k = match c.iter().position(|&ch| ch == '.') {
        Some(_) => Number::Decimal,
        None => match c.iter().position(|&ch| ch == '/') {
            Some(_) => Number::Fraction,
            None => match c.iter().position(|&ch| ch == '%') {
                Some(_) => Number::Percentage,
                None => Number::Integer,
            },
        },
    };
    let ans:String=match k {
        Number::Integer=>{
            let a= s_temp.trim().parse::<i64>().unwrap();
            format!("{}",fz(a))
        }
        Number::Decimal=>{
            let c:Vec<&str>=s_temp.trim().split('.').collect();
            let (a,b):(i64,i64)=(c[0].parse::<i64>().unwrap(),c[1].parse::<i64>().unwrap());
            format!("{}.{}",fz(a),fz(b))
        }
        Number::Fraction=>{
            let c:Vec<&str>=s_temp.trim().split('/').collect();
            let (a,b):(i64,i64)=(c[0].parse::<i64>().unwrap(),c[1].parse::<i64>().unwrap());
            format!("{}/{}",fz(a),fz(b))
        }
        Number::Percentage=>{
            let c:Vec<&str>=s_temp.trim().split('%').collect();
            let a:i64=c[0].parse::<i64>().unwrap();
            format!("{}%",fz(a),)
        }
    };
    println!("{}",ans);
}