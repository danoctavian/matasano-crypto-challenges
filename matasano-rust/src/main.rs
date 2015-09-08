extern crate rustc_serialize as serialize;

use serialize::base64::{self, ToBase64};
use serialize::hex::FromHex;
use std::str;

/// S1;C1
fn hexToBase64(hexStr: &str) -> Result<String, serialize::hex::FromHexError> {
	hexStr.from_hex().map(|s| s.to_base64(base64::STANDARD))
}

/// S1;C2
fn fixedXOR(x: &[u8], y: &[u8]) -> Vec<u8> {
	let len = x.len();
	let mut vec = Vec::with_capacity(len);

	for i in 0..len {
		vec.push(x[i] ^ y[i]);
	}
	vec
}

/// S1;C3
fn singleCharXOR(hexStr: &str) -> u8 {
	let bs = hexStr.from_hex().unwrap();
	let len = bs.len();
	let mut secret = Vec::with_capacity(len);
	unsafe { secret.set_len(len);}

	let (score, ch) = (0..255).map(|c| {
		for i in 0..len {secret[i] = c;}
		let plain = fixedXOR(&bs, &secret);

		str::from_utf8(&plain).map(|str| (englishScore(str), c))
	}).filter_map(resultToOpt)
		.max().unwrap();
	ch
}

fn englishScore(str: &str) -> i64 {
	println!("{}", str);
	0
}

// english letter frequencies
const freqVals: [f64; 26] = [8.2, 1.5, 2.8, 4.3, 12.7, 2.2, 2.0, 6.1, 7.0, 0.2, 0.8, 4.0, 2.4, 6.7, 7.5, 2.0, 0.1, 6.0, 6.3, 9.1, 2.8, 1.0, 2.4, 0.2, 2.0, 0.1];

fn resultToOpt<T, E>(r: Result<T, E>) -> Option<T> {
	match r {
		Ok(v) => Some(v),
		_ => None
	}
}

fn main() {
	println!("matasano crypto challenges...");
	let i = (1..10).next();
	//singleCharXOR("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

	let s = "Cooking MC's like a pound of bacon";
	let cs = s.chars();

	//let v = Vec::<char>::from_iter(cs);
	//println!("{}", v);
}

#[test]
fn hexToBase64Test() {
	let actual = hexToBase64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
	let expected = Some("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
	assert_eq!(actual.unwrap(), expected.unwrap());
}

#[test]
fn testFixedXOR() {
	let x = "1c0111001f010100061a024b53535009181c".from_hex().unwrap();
	let y = "686974207468652062756c6c277320657965".from_hex().unwrap();
	let actual = fixedXOR(&x, &y);
	let expected = "746865206b696420646f6e277420706c6179".from_hex().unwrap();
	assert_eq!(actual, expected);
}

// "Cooking MC's like a pound of bacon"