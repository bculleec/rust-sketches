fn main() {
    let tup: (i32, f64, u8) = (599, 4.5, 2); // A tuple can have different types but cannot shrink
                                             // or grow once declared
    let (x, y, z) = tup; // de-structuring
    println!("{x}");
    println!("{y}");
    println!("{z}");
}
