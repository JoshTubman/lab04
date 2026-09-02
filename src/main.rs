use std::io;

fn main() {
println!("input weight");

let mut line = String::new();
io::stdin().read_line(&mut line).expect("Failed to read line");
let weight: f64 = line.trim().parse().expect("Please type a number");

println!("input dest 1/2");

let mut line = String::new();
io::stdin().read_line(&mut line).expect("Failed to read line");
let dest: char = line.trim().parse().expect("Please type a number");
println!("express y/n");
let mut line = String::new();
io::stdin().read_line(&mut line).expect("Failed to read line");
let express: char = line.trim().parse().expect("Please type a number");

println!("tier 1/2/3");
let mut line = String::new();
io::stdin().read_line(&mut line).expect("Failed to read line");
let tier: char = line.trim().parse().expect("Please type a number");

let mut cost: f64;

if weight < 0.0{
println!("invalid weight entered");
return;
} else if weight <=1.0{
cost = 5.0;
}
else if weight <=5.0{
cost = 12.0;
}
else if weight <=20.0{
cost = 25.0;
}
else {
cost = 50.0;
}


    println!("{}",cost);
}
