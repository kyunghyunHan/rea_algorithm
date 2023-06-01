extern crate rsa;
use std::io::{BufReader,BufRead,BufWriter,Write,stdout,stdin};
use rsa::{PublicKey,RSAPrivateKey,PaddingScheme};
use rand::rngs::OsRng;
fn main() {

    let mut reader= BufReader::new(stdin().lock());
    let mut input= String::new();
    reader.read_line(&mut input).unwrap();
    let s= input.trim().to_string();
    println!("Hello, world!");
}

//N=PQ
//Pub= (e,N)
//Priv=(d,N)


//Enc=M^e (mod N)


//Dec=Enc^d (modN)


