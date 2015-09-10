extern crate rustc_serialize as serialize;
extern crate num;

use serialize::base64::{self, ToBase64};
use serialize::hex::FromHex;
use std::str;

use serialize::hex::ToHex;

use std::collections::HashMap;
/// S1;C1
fn hex_to_base64(hexStr: &str) -> Result<String, serialize::hex::FromHexError> {
	hexStr.from_hex().map(|s| s.to_base64(base64::STANDARD))
}

/// S1;C2
fn fixed_XOR(x: &[u8], y: &[u8]) -> Vec<u8> {
	let len = x.len();
	let mut vec = Vec::with_capacity(len);

	for i in 0..len {
		vec.push(x[i] ^ y[i]);
	}
	vec
}

/// S1;C3
fn single_char_XOR(hexStr: &str) -> (u8, String) {
	let bs = hexStr.from_hex().unwrap();
	let len = bs.len();
	let mut secret = Vec::with_capacity(len);
	unsafe { secret.set_len(len);}

	let (score, ch, str) = (0..255).map(|c| {
		for i in 0..len {secret[i] = c;}
		let plain = fixed_XOR(&bs, &secret);

		str::from_utf8(&plain).map(|str| (english_score(str), c, str.to_string()))
	}).filter_map(result_to_opt)
		.max().unwrap();

	(ch, str)
}

fn english_score(s: &str) -> i64 {
	let begin = 'a' as u32;
	let end = 'z' as u32;
	let len = (end - begin + 1) as usize;
	let mut charCount = Vec::with_capacity(len);
	for i in 0..len {charCount.push(0);}

	for c in s.chars().filter(|c| c.is_alphabetic()) {
		let lower = c.to_lowercase().next().unwrap();
		let index = ((lower as u32) - begin) as usize;
		charCount[index] += 1;
	}

	let total: i64 = charCount.iter().fold(0, |acc, &item| acc + item);
	let freqs = charCount.iter().map(|&x| (x as f64) / (total as f64) * 100.0);


/*
	println!("the string is {}", s);
	for f in charCount.iter().map(|&x| (x as f64) / (total as f64) * 100.0) {
		print!("{},", f);
	}
	println!("");
*/

	let score: f64 = freqs.zip(freqVals.iter())
												.map(|(x, y)| (num::pow(x - y, 2)))
												.fold(0.0, |acc, item| acc + item).sqrt();
	(score * 1000.0) as i64 
}

// english letter frequencies
const freqVals: [f64; 26] = [8.2, 1.5, 2.8, 4.3, 12.7, 2.2, 2.0, 6.1, 7.0, 0.2, 0.8, 4.0, 2.4, 6.7, 7.5, 2.0, 0.1, 6.0, 6.3, 9.1, 2.8, 1.0, 2.4, 0.2, 2.0, 0.1];


/// S1;C5
fn encrypt_repeat_XOR(plain: &str, key: &str) -> String {
	let enc = plain.bytes().zip(key.bytes().cycle()).map(|(b, k)| b ^ k);

	let bs: Vec<u8> = enc.collect();
	(bs[..]).to_hex()
}

fn result_to_opt<T, E>(r: Result<T, E>) -> Option<T> {
	match r {
		Ok(v) => Some(v),
		_ => None
	}
}

/// S2;C9 PKCS#7 padding
fn PKCS_pad(bytes: &mut Vec<u8>, blockSize: usize) {
	for x in 0..((blockSize - bytes.len() % blockSize) % blockSize) {
		bytes.push(4);
	}
}

fn main() {
	println!("matasano crypto challenges...");
}

#[test]
fn hex_to_base64_test() {
	let actual = hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
	let expected = Some("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
	assert_eq!(actual.unwrap(), expected.unwrap());
}

#[test]
fn test_Fixed_XOR() {
	let x = "1c0111001f010100061a024b53535009181c".from_hex().unwrap();
	let y = "686974207468652062756c6c277320657965".from_hex().unwrap();
	let actual = fixed_XOR(&x, &y);
	let expected = "746865206b696420646f6e277420706c6179".from_hex().unwrap();
	assert_eq!(actual, expected);
}

#[test]
fn test_find_key() {
	let (k, str) = single_char_XOR("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
	assert_eq!(str, "Cooking MC's like a pound of bacon");
}

#[test]
fn test_encrypt_repeat_XOR() {
	let stanza = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
	let key = "ICE";
	let actual = encrypt_repeat_XOR(&stanza, &key);								
	assert_eq!(actual, "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
}

#[test]
fn test_PKCS_pad() {
	let mut v = "YELLOW SUBMARINE".to_string().into_bytes();
	PKCS_pad(&mut v, 10); 
	let expected = "YELLOW SUBMARINE\x04\x04\x04\x04".to_string().into_bytes();
	assert_eq!(v, expected);
}