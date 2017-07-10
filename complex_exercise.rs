// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

struct Complex {
    real: f32,
    imaginary: f32
}

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter)->std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
impl Complex {
    fn new(real: f32, imaginary: f32)-> Complex {
        Complex { real: real, imaginary: imaginary}
    }
    fn to_string(&self)->String {
        format!("{} + {}i", self.real, self.imaginary)
    }

    fn add(&self, c: &Complex)-> Complex {
        let rc = self.real + c.real;
        let ic = self.imaginary + c.imaginary;
        Complex::new(rc, ic)
    }

    fn times_ten_mutable(&mut self) {
        self.imaginary = self.imaginary * 10.0;
        self.real = self.real * 10.0;
    }

    fn abs(&self)->Complex {
        let ar = match self.real {n if n>0.0=>n, e=> e * -1.0};
        let ai = match self.imaginary {n if n>0.0=>n, e=> e * -1.0};
        Complex { real: ar, imaginary: ai}
    }
}

fn main() {
    let mut c = Complex::new(10.0,20.0);
    c.times_ten_mutable();
    let res0 = c.to_string();
    println!("res0: {}", res0);

    let d = Complex::new(-10.0,20.0);
    println!("d.abs(): {}", d.abs());
}
