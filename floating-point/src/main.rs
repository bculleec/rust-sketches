fn main() {
    let x = 2.0; // floats default to f64 
                 // this is because it's roughly the same speed with more precision
    let y: f32 = 3.0;


    let x = x + y;
    let y = y - x;

    let w = x * y;
    let z = x / y;

    println!("{x}");
    println!("{y}");
    println!("{w}");
    println!("{z}");
}
