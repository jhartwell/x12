mod common;
mod ins;
mod mis;

use ins::TwoSevenSeven;
use mis::NineNineNine;

fn temp() {
    let a = TwoSevenSeven {};
    println!("{:?}", a);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        temp();
        assert!(false);
    }
}
