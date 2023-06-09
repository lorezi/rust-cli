use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input);
    
    let mut mars_weight = calculate_mass(99.8);
    mars_weight = mars_weight * 1000.0;
    println!("Weight on Mars: {}g",mars_weight);
}


fn calculate_mass(weight:f32) -> f32 {
    (weight / 9.81) * 3.711
}