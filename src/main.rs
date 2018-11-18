extern crate rand;

use rand::prelude::*;
use std::env;
use std::process;
use std::cmp::min;

// Accepts two prime numbers and prints out the various components of an RSA 
// encryption scheme.
fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3{
        println!("Usage: rsa Prime1 Prime2");
        process::exit(1);
    }
    let mut rng = thread_rng();
    let p: i64 = args[1].parse().expect("failed to parse Prime1");
    let q: i64 = args[2].parse().expect("failed to parse Prime2");

    let N = p*q;
    println!("N = {}", N);

    let Phi = (p-1)*(q-1);
    println!("Phi = {}", Phi);

    let mut e = rng.gen_range(2,min(p,q));
    let (mut s, _, mut t, _, mut gcd) = extended_gcd(e,Phi);
    while gcd != 1 {
        e=rng.gen_range(2,min(p,q));
        match extended_gcd(e, Phi){
            (x, _, y, _, z) => {
                gcd = z;
                t = y;
                s = x;
            }
        };
    }
    
    println!("e = {}", e);

    let d = if s<0 {
        s + Phi
    }else {
        s
    };
    println!("d = {}", d);

    println!("{}*{} + {}*{} = {}", s, e, t, Phi, gcd);
}

// Uses the Extended Euclidean algorithm to calculate sx + ty = gcd(x, y)
// 
// Returns a 5-tuple: (s, x, t, y, gcd)
//
// Based on https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Pseudocode
fn extended_gcd(x: i64, y:i64) -> (i64, i64, i64, i64, i64) {
    let mut s = 0;
    let mut old_s = 1;

    let mut t = 1;
    let mut old_t = 0;

    let mut r = y;
    let mut old_r = x;

    while r != 0{
        let quotient = old_r / r;

        let temp = r;
        r = old_r - quotient*temp;
        old_r = temp;

        let temp = s;
        s = old_s - quotient*temp;
        old_s = temp;

        let temp = t;
        t = old_t - quotient*temp;
        old_t = temp;
    }
//     s  * x +  t  * y =  gcd
    (old_s, s, old_t, t, old_r)
}
