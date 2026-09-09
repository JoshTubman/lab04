use std::io;

fn main() {
println!("input weight");

let mut line = String::new();
io::stdin().read_line(&mut line).expect("Failed to read line");
let weight: f64 = line.trim().parse().expect("Please type a number");

let mut cost: f64;
// weight function
if weight < 0.0{
println!("invalid weight entered");
return;
} else if weight <=1.0{
cost = 5.0;}
else if weight <=5.0{
cost = 12.0;}
else if weight <=20.0{
cost = 25.0;}
else {
cost = 50.0;}
println!("Input Destination: Domestic-1/Internatinal-2");

let mut line = String::new();
io::stdin().read_line(&mut line).expect("Failed to read line");
let dest: char = line.trim().parse().expect("Please type a number");
// destination Functon
if dest == '2'{
cost *= 2.0;
} 
else if dest == '1'{} 
else{
println!("Invalid Destination Entered");
return;}

println!("express y/n");
let mut line = String::new();
io::stdin().read_line(&mut line).expect("Failed to read line");
let express: char = line.trim().parse().expect("Please type a number");
// Express function
if express == 'y'{
	if dest == '2'{
	cost += 20.0;
	} else{
	cost += 8.0;
	}
}
else if express == 'n'{}
else{
println!("Invalid Response Entered");
return; }

println!("Membership tier: None-1/Base-2/Premium-3");
let mut line = String::new();
io::stdin().read_line(&mut line).expect("Failed to read line");
let tier: char = line.trim().parse().expect("Please type a number");
//Membership Function
if tier == '1'{}
else if tier == '2'{
cost *= 0.9;
} else if tier == '3'{
cost *= 0.8;
} else{
println!("Invalid Membership Entered");
return; }

// final output
println!("total cost = ${:.02}", {cost});
}
