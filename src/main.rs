fn main() {
    println!("I Love Annida Hanifah!");
    let x: i32 = 10;
    println!("{}", x);

    let mut y: i32 = 11;
    println!("{}", y);
    y = 20;
    println!("{}", y);

    let b: f64 = 3.14;
    println!("{}", b);

    let is_active: bool = true;
    println!("{}", is_active);

    let s: &str = "Halo";
    println!("{}", s);

    let mut r: String = String::from("Hallo");
    r.push_str(" Rust");
    println!("{}", r);

    let mut name: &str = "Toriq Afanudin";
    println!("Halo {}", name);

    name = "Faisa Nirbita";
    println!("Halo {}", name);

    let name: i32 = 10;
    println!("angka: {}", name);

    let pi: f64 = 3.14;
    println!("pi = {}", pi);

    let angka: i8 = 10;
    println!("{}", angka);
    let angka2: i16 = angka as i16;
    println!("{}", angka2);
    let angka3: i32 = angka as i32;
    println!("{}", angka3);

    let p: f64 = 10.0;
    let q: f64 = 7.0;
    let r: f64 = p * q;
    println!("c = {}", r);
    let s: f64 = p / q;
    println!("p / q = {}", s);
    let t: f64 = p % q;
    println!("{}", t);

    let benar: bool = true;
    let salah: bool = false;
    println!("{} {}", benar, salah);

    let result: bool = 10 > 11;
    println!("{}", result);
    let result2: bool = 10 > 9;
    let result3: bool = result && result2;
    println!("result3: {}", result3);

    let char1: char = 'a';
    let char2: char = 'b';
    println!("{}, {}", char1, char2);

    let data: (i32, f64, &str, bool) = (10, 10.5, "afan", true);
    println!("{:?}", data);
    let a: i32 = data.0;
    let b: f64 = data.1;
    let c: &str = data.2;
    println!("{} {} {}", a, b, c);
    let (d, e, f, _) = data;
    println!("{} {} {}", d, e, f);

    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);
    println!("{}", numbers[0]);
    numbers[0] = 6;
    println!("{:?}", numbers);
    let panjang_numbers: usize = numbers.len();
    println!("panjang numbers: {}", panjang_numbers);

    let matrix: [[i32; 2]; 2] = [[1, 7], [3, 5]];
    println!("{:?}", matrix);
    println!("[0][0] = {}", matrix[0][0]);
    println!("[0][1] = {}", matrix[0][1]);
    println!("[1][0] = {}", matrix[1][0]);
    println!("[1][1] = {}", matrix[1][1]);
    println!("[0] = {:?}", matrix[0]);

    const MIN: i32 = 0;
    const MAX: i32 = 100;
    println!("min: {}, max: {}", MIN, MAX);
}

#[test]
fn hello_test() {
    println!("Hello Test!")
}
