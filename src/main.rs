use std::os;
use std::str::FromStr;

const NUM_OF_CURRENCIES: usize = 1;

#[derive(Show)]
#[derive(Hash)]
#[derive(PartialEq)]
#[derive(Eq)]
enum Currency {
    UAH,
    USD,
}

type Exchange = (f32, Currency);

fn render(exs: &[Exchange]) -> String {
    exs.iter().
    map(|&(ref v, ref c)| format!("{} {:?}", v, c)).
    collect::<Vec<String>>().
    connect(" ")
}

fn convert(strvalue: &str, currency: &str) -> Result<[Exchange; NUM_OF_CURRENCIES], &'static str> {
    //Box::new([(5.1, Currency::UAH)])
    match FromStr::from_str(strvalue) {
        None => Err("Invalid value"),
        Some(value) => Ok([(value, Currency::UAH)])
    }
}

fn main() {
    match os::args().as_slice() {
        [_, ref val, ref curr] =>
            match convert(val.as_slice(), curr.as_slice()) {
                Ok(exs) => println!("{}", render(&exs)),
                Err(why) => println!("{}", why)
            },
        _ => println!("Wrong usage"),
    }
}
