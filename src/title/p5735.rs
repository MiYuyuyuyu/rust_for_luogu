struct Point{
    x:f64,
    y:f64,
}

impl Point{
    fn new() ->Point {
        let mut s_temp = String::new();
        std::io::stdin()
            .read_line(&mut s_temp)
            .unwrap();
        s_temp=s_temp.trim().to_string();
        let c:Vec<&str>=s_temp.split_whitespace().collect();
        let x=c.get(0).unwrap().parse::<f64>().unwrap();
        let y=c.get(1).unwrap().parse::<f64>().unwrap();
        Point{
            x,
            y,
        }
    }
    fn distence(&self,other:&Point) ->f64 {
        let dis_x = self.x - other.x;
        let dis_y = self.y - other.y;
        ( dis_x.powi(2)+dis_y.powi(2) ).sqrt()
    }
}

fn cover(dis1:f64,dis2:f64,dis3:f64)->f64 {
    dis1+dis2+dis3
}

fn ans(){
    let a = Point::new();
    let b = Point::new();
    let c = Point::new();
    let dis1 = a.distence(&b);
    let dis2 = a.distence(&c);
    let dis3 = b.distence(&c);
    let ans=cover(dis1, dis2, dis3);
    println!("{:.2}",ans);
}

fn main() {
    ans();
}