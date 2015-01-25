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


impl FromStr for Currency {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "UAH" => Some(Currency::UAH),
            "USD" => Some(Currency::USD),
            _ => None,
        }
    }
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
            &Error::InvalidValue => "Invalid value. Must be positive float number.",
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


fn convert(value_str: &str, currency_str: &str) -> Result<[Exchange; NUM_OF_CURRENCIES], Error> {
    fn read_value(value_str: &str) -> Result<f32, Error> {
        match FromStr::from_str(value_str) {
            None => Err(Error::InvalidValue),
            Some(value) if value >= 0.0 => Ok(value),
            _ => Err(Error::InvalidValue)
        }
    }

    fn read_currency(currency_str: &str) -> Result<Currency, Error> {
        match FromStr::from_str(currency_str) {
            None => Err(Error::InvalidCurrency),
            Some(currency) => Ok(currency),
        }
    }

    let value = try!(read_value(value_str));
    let currency = try!(read_currency(currency_str));
    Ok([(value, currency)])
}


fn main() {
    match &os::args()[] {
        [_, ref val, ref curr] =>
            match convert(&val[], &curr[]) {
                Ok(exs) => println!("{}", render(&exs)),
                Err(why) => println!("{}", why)
            },
        _ => println!("Wrong usage"),
    }
}
