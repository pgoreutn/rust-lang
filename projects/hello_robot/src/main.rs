struct Robot{}

impl Robot {
     fn greet(&self){
         println!("Hello , developer !!!");
     }
}

fn main() {
    println!("Hello , Robot");
    let bot  = Robot{};
    bot.greet();

}
