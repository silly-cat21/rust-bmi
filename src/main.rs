use std::io;

fn main(){

println!("vitej v programu pro vypocet bmi indexu.");
println!("zadej svou vahu v kg: ");

let mut a = String::new();
io::stdin().read_line(&mut a).expect("chyba");
let vaha: f32 = a.trim().parse().unwrap();

println!("zadej svou vysku v centimetrech:");
let mut b = String::new();
io::stdin().read_line(&mut b).expect("chyba");
let vyska: f32 = b.trim().parse().unwrap();

let vysl: f32 = vaha / (vyska * vyska); 

if vysl < 18.5{
    println!("mas podvahu");
}
else if vysl >= 18.5 && vysl < 25.0{
    println!("mas optimalni vahu");
}
else if vysl >= 25.0 && vysl < 30.0{
    println!("mas nadvahu");
}
else if vysl >= 30.0 && vysl < 40.0 {
    println!("hodnota obezity prvniho stupne!");
}
else if vysl >= 40.0{
    println!("tezka obezita :(");
}
else {
    println!("chyba");
}
}