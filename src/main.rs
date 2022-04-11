extern crate frasi;

pub use frasi::inglese::{saluti, commiati};
pub use frasi::giapponese;

fn main() {
    println!("Ciao in inglese: {}", saluti::ciao());
    println!("Arrivederci in inglese: {}", commiati::arrivederci());

    println!("Ciao in giapponese: {}", giapponese::ciao());
    println!("Arrivederci in giapponese: {}", giapponese::arrivederci());
}
