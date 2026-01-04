fn main() {
    println!("I Love Annida Hanifah!");
    let x = 10;
    println!("{}", x);

    let mut y = 11;
    println!("{}", y);
    y = 20;
    println!("{}", y);

    let b: f64 = 3.14;
    println!("{}", b);

    let is_active: bool = true;
    println!("{}", is_active);

    let s = "Halo";
    println!("{}", s);

    let mut r = String::from("Hallo");
    r.push_str(" Rust");
    println!("{}", r);

    let mut name = "Toriq Afanudin";
    println!("Halo {}", name);

    name = "Faisa Nirbita";
    println!("Halo {}", name);

    let name = 10;
    println!("angka: {}", name);

    let pi = 3.14;
    println!("pi = {}", pi);

    let angka: i8 = 10;
    println!("{}", angka);
    let angka2: i16 = angka as i16;
    println!("{}", angka2);
    let angka3 = angka as i32;
    println!("{}", angka3);

    let p = 10.0;
    let q = 7.0;
    let r = p * q;
    println!("c = {}", r);
    let s = p / q;
    println!("p / q = {}", s);
    let t = p % q;
    println!("{}", t);

    let benar = true;
    let salah = false;
    println!("{} {}", benar, salah);

    let result = 10 > 11;
    println!("{}", result);
    let result2 = 10 > 9;
    let result3 = result && result2;
    println!("result3: {}", result3);

    let char1 = 'a';
    let char2 = 'b';
    println!("{}, {}", char1, char2);

    let data = (10, 10.5, "afan", true);
    println!("{:?}", data);
    let a = data.0;
    let b = data.1;
    let c = data.2;
    println!("{} {} {}", a, b, c);
    let (d, e, f, _) = data;
    println!("{} {} {}", d, e, f)
}

#[test]
fn hello_test() {
    println!("Hello Test!")
}
