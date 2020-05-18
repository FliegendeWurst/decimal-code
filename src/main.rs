use itertools::join;

use std::env;
use std::error::Error;
use std::{convert::{TryInto, TryFrom}, io::{self, BufRead, stdin}, iter::once};

fn main() -> Result<(), Box<dyn Error>> {
	let args = env::args().skip(1).collect::<Vec<_>>();
	if args.is_empty() {
		// batch mode
		for line in stdin().lock().lines() {
			let line = line?;
			let mut parts = line.split_ascii_whitespace();
			let format = parts.next().ok_or(io::Error::new(io::ErrorKind::Other, "no encoding"))?;
			let value = parts.next().ok_or(io::Error::new(io::ErrorKind::Other, "no value"))?;
			let number = match format {
				"decimal" => parse_decimal(value)?,
				"bcd" => parse_bcd(value)?,
				"aiken" => parse_aiken(value)?,
				"stibitz" => parse_stibitz(value)?,
				_ => Err("unknown encoding")?
			};
			let x = convert(&number);
			println!("{}\t{}\t{}\t{}", x.0, x.1, x.2, x.3);
		}
		Ok(())
	} else {
		Err("too many arguments".into())
	}
}

fn convert(number: &Decimal) -> (String, String, String, String) {
	(
		number.format_decimal(),
		number.format_bcd(),
		number.format_aiken(),
		number.format_stibitz()
	)
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Decimal {
	digits: Vec<Digit>,
	digits_fractional: Vec<Digit>
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Digit {
	Zero, One, Two, Three, Four, Five, Six, Seven, Eight, Nine
}

use Digit::*;

impl TryFrom<char> for Digit {
	type Error = Box<dyn Error>;

	fn try_from(x: char) -> Result<Self, Self::Error> {
		match x {
			'0' => Ok(Zero),
			'1' => Ok(One),
			'2' => Ok(Two),
			'3' => Ok(Three),
			'4' => Ok(Four),
			'5' => Ok(Five),
			'6' => Ok(Six),
			'7' => Ok(Seven),
			'8' => Ok(Eight),
			'9' => Ok(Nine),
			x => Err(format!("unknown digit {:?}", x).into())
		}
	}
}

impl Digit {
	fn value(&self) -> usize {
		match self {
			Zero => 0,
			One => 1,
			Two => 2,
			Three => 3,
			Four => 4,
			Five => 5,
			Six => 6,
			Seven => 7,
			Eight => 8,
			Nine => 9
		}
	}
}

fn parse_decimal(value: &str) -> Result<Decimal, Box<dyn Error>> {
	let mut digits = Vec::new();
	let mut digits_fractional = Vec::new();

	let mut fractional = false;
	for c in value.chars() {
		if !fractional && (c == '.' || c == ',') {
			fractional = true;
			continue;
		}
		let digit: Digit = c.try_into()?;
		if !fractional {
			digits.push(digit);
		} else {
			digits_fractional.push(digit);
		}
	}

	Ok(Decimal {
		digits, digits_fractional
	})
}

#[test]
fn parse_decimal_test() {
	assert_eq!(parse_decimal("42.35").unwrap(), Decimal { digits: vec![Four, Two], digits_fractional: vec![Three, Five] });
}

fn parse_bcd(value: &str) -> Result<Decimal, Box<dyn Error>> {
	todo!("BCD parsing");
}

fn parse_aiken(value: &str) -> Result<Decimal, Box<dyn Error>> {
	todo!("AIKEN parsing");
}

fn parse_stibitz(value: &str) -> Result<Decimal, Box<dyn Error>> {
	todo!("STIBITZ parsing");
}

impl Decimal {
	fn format_decimal(&self) -> String {
		self.format_specified(&["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"])
	}

	fn format_bcd(&self) -> String {
		self.format_specified(&["0000", "0001", "0010", "0011", "0100", "0101", "0110", "0111", "1000", "1001"])
	}

	fn format_aiken(&self) -> String {
		self.format_specified(&["0000", "0001", "0010", "0011", "0100", "1011", "1100", "1101", "1110", "1111"])
	}

	fn format_stibitz(&self) -> String {
		self.format_specified(&["0011", "0100", "0101", "0110", "0111", "1000", "1001", "1010", "1011", "1100"])
	}

	fn format_specified(&self, table: &[&str]) -> String {
		if self.digits_fractional.is_empty() {
			join(self.digits.iter().map(|x| table[x.value()]), " ")
		} else {
			join(
				self.digits.iter().map(|x| table[x.value()])
					.chain(once("."))
					.chain(self.digits_fractional.iter().map(|x| table[x.value()])),
				" "
			)
		}
	}
}

#[test]
fn format_decimal_test() {
	let number = Decimal { digits: vec![Four, Four], digits_fractional: vec![Five, One] };
	assert_eq!(convert(&number), ("4 4 . 5 1".into(), "0100 0100 . 0101 0001".into(), "0100 0100 . 1011 0001".into(), "0111 0111 . 1000 0100".into()));
}
