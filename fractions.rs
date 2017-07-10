// no-shebang
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables,unused_must_use)]
#![allow(unused_mut)]

type SString = &'static str;

#[derive(Debug, Clone, Copy)]
struct Fraction {
    n: i32,
    d: i32
}

impl std::fmt::Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter)->std::fmt::Result {
        match *self {
            Fraction { n, d } if n==1 && d== 1 => write!(f, "1"),
            Fraction {n,d} =>write!(f, "{}/{}", self.n, self.d)
        }
    }
}

impl Fraction {
    fn new(n: i32, d: i32)->Fraction {
        Fraction{n:n,d:d}.reduce()
    }

    fn gcd(a: i32,b: i32)->i32 {
        if b==0 {
            return a;
        } 
        Self::gcd(b, a % b)
    }
    fn reduce(self)->Fraction {
        let gcd = Self::gcd(self.n, self.d);
        Fraction{n:self.n/gcd,d: self.d/gcd}
    }
}

impl std::ops::Add for Fraction {
    type Output = Fraction;
    fn add(self, with:Fraction)->Self::Output {
        let d = self.d * with.d;
        let n = (d / self.d * self.n) + (d/with.d*with.n);
        Fraction::new(n,d)
    }
}

fn main() {
    println!("begin.");
    let fa = Fraction::new(1,2);
    let fb = Fraction::new(1,2);
    let fc = fa + fb;
    println!("fc: {}", fc);
    println!("done.");
}
