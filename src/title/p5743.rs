// 一只小猴买了若干个桃子。第一天他刚好吃了这些桃子的一半，又贪嘴多吃了一个；
// 接下来的每一天它都会吃剩余的桃子的一半外加一个。第 n 天早上起来一看，只剩下 1 个桃子了。请问小猴买了几个桃子？

// 输入格式
// 输入一个正整数 n，表示天数。

// 输出格式
// 输出小猴买了多少个桃子

fn main() {
    let mut s_temp = String::new();
    std::io::stdin().read_line(&mut s_temp).unwrap();
    let n = s_temp.trim().parse::<i32>().unwrap();
    let mut cnt = 1;
    for _ in 0..n-1 {
        cnt = (cnt+1)*2;
    }
    println!("{}",cnt);
}