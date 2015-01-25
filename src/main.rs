use std::os;
use std::str::FromStr;
use std::fmt;

const NUM_OF_CURRENCIES: usize = 1;

#[derive(Show)]
#[derive(Hash)]
#[derive(PartialEq)]
#[derive(Eq)]
enum Currency {
    UAH,
    USD,
}


#[derive(Debug)]
enum Error {
    InvalidValue,
    InvalidCurrency,
    ConnectionError,
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            &Error::InvalidValue => "Invalid value. Must be float number.",
            &Error::InvalidCurrency => "Invalid currency.",
            &Error::ConnectionError => "Connection error.",
        };
        fmt::Display::fmt(msg, f)
    }
}


type Exchange = (f32, Currency);

fn render(exs: &[Exchange]) -> String {
    exs.iter().
    map(|&(ref v, ref c)| format!("{} {:?}", v, c)).
    collect::<Vec<String>>().
    connect(" ")
}

fn convert(strvalue: &str, currency: &str) -> Result<[Exchange; NUM_OF_CURRENCIES], Error> {
    match FromStr::from_str(strvalue) {
        None => Err(Error::InvalidValue),
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
