use std::collections::HashMap;
use std::error::Error;

use crate::HttpMessage::*;

#[derive(Debug, PartialEq, Eq)]
pub enum HttpVersion {
	Version0_9,
	Version1_0,
	Version1_1,
	Version2_0,
}

#[derive(Debug, PartialEq, Eq)]
pub enum HttpMethod {
	GET,
	PUT,
	POST,
	HEAD,
	DELETE,
	PATCH,
	OPTIONS,
}

#[derive(Debug, PartialEq, Eq)]
pub enum HttpMessage {
	Request(HttpRequest),
	Response(HttpResponse),
}

#[derive(Debug, PartialEq, Eq)]
pub struct HttpRequest {
	pub version: HttpVersion,
	pub method: HttpMethod,
	pub resource: String,
	pub headers: HashMap<String, String>,
	pub body: Option<Vec<u8>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct HttpResponse {
	pub version: HttpVersion,
	pub status_code: [u8; 3],
	pub reason_phrase: String,
	pub headers: HashMap<String, String>,
	pub body: Option<Vec<u8>>,
}

fn increment_position(bytes: &Vec<u8>, position: &mut usize, increment_by: usize) -> Result<(), Box<dyn Error>> {
	// Check if new position is past the end of the json string
	if *position + increment_by > bytes.len() {
		return Err("Unexpectedly reached end of message".into());
	}

	*position += increment_by;

	Ok(())
}

fn get_byte(bytes: &Vec<u8>, position: &mut usize) -> Result<u8, Box<dyn Error>> {
	let byte = bytes[*position];
	increment_position(bytes, position, 1)?;

	Ok(byte)
}

fn peek_byte_at_offset(bytes: &Vec<u8>, position: &usize, offset: usize) -> Result<u8, Box<dyn Error>> {
	// Check if char is past the end of the json string
	if *position + offset > bytes.len() - 1 {
		return Err("Unexpectedly reached end of message".into());
	}

	Ok(bytes[*position + offset])
}

fn check_bytes(bytes: &Vec<u8>, position: &mut usize, check: Vec<u8>) -> Result<bool, Box<dyn Error>> {
	// Check that each byte matches
	for check_byte in check {
		if get_byte(bytes, position)? != check_byte {
			return Ok(false);
		}
	}

	Ok(true)
}

fn skip_white_space(bytes: &Vec<u8>, position: &mut usize) -> Result<(), Box<dyn Error>> {
	loop {
		if peek_byte_at_offset(bytes, position, 0)? != 32u8 {
			break;
		}

		increment_position(bytes, position, 1)?;
	}

	Ok(())
}

fn determine_request(bytes: &Vec<u8>) -> Result<bool, Box<dyn Error>> {
	// H, T is response
	// H, E | G | P | D | O is request
	match bytes[0] {
		72u8 => match bytes[1] {
			84u8 => Ok(false),
			69u8 => Ok(true),
			_ => Err("1 Malformed HTTP Message".into()),
		},
		71u8 | 80u8 | 68u8 | 79u8 => Ok(true),
		_ => Err("2 Malformed HTTP Message".into()),
	}
}

fn get_method(bytes: &Vec<u8>, position: &mut usize) -> Result<HttpMethod, Box<dyn Error>> {
	match get_byte(bytes, position)? {
		// G
		71u8 => {
			// E, T, space
			if check_bytes(bytes, position, vec![69u8, 84u8, 32u8])? {
				return Ok(HttpMethod::GET);
			}
		}
		// P
		80u8 => {
			match get_byte(bytes, position)? {
				// U
				85u8 => {
					// T, space
					if check_bytes(bytes, position, vec![84u8, 32u8])? {
						return Ok(HttpMethod::PUT);
					}
				}
				// O
				79u8 => {
					// S, T, space
					if check_bytes(bytes, position, vec![83u8, 84u8, 32u8])? {
						return Ok(HttpMethod::POST);
					}
				}
				// A
				65u8 => {
					// T, C, H, space
					if check_bytes(bytes, position, vec![84u8, 67u8, 72u8, 32u8])? {
						return Ok(HttpMethod::PATCH);
					}
				}
				_ => {}
			}
		}
		// H
		72u8 => {
			// E, A, D, space
			if check_bytes(bytes, position, vec![69u8, 65u8, 68u8, 32u8])? {
				return Ok(HttpMethod::HEAD);
			}
		}
		// D
		68u8 => {
			// E, L, E, T, E, space
			if check_bytes(bytes, position, vec![69u8, 76u8, 69u8, 84u8, 69u8, 32u8])? {
				return Ok(HttpMethod::DELETE);
			}
		}
		// O
		79u8 => {
			// P, T, I, O, N, S, space
			if check_bytes(bytes, position, vec![80u8, 84u8, 73u8, 79u8, 78u8, 32u8, 32u8])? {
				return Ok(HttpMethod::OPTIONS);
			}
		}
		_ => {}
	}

	Err("3 Malformed HTTP Message".into())
}

fn get_resource(bytes: &Vec<u8>, position: &mut usize) -> Result<String, Box<dyn Error>> {
	if bytes[*position] == 32u8 {
		return Err("4 Malformed HTTP Message".into());
	}

	let mut resource = String::new();
	let mut byte: u8 = get_byte(bytes, position)?;

	while byte != 32u8 {
		resource.push(byte as char);
		byte = get_byte(bytes, position)?;
	}

	Ok(resource)
}

fn get_version(bytes: &Vec<u8>, position: &mut usize) -> Result<HttpVersion, Box<dyn Error>> {
	// H, T, T, P, /
	if !check_bytes(bytes, position, vec![72u8, 84u8, 84u8, 80u8, 47u8])? {
		return Err("5 Malformed HTTP Message".into());
	};

	match get_byte(bytes, position)? {
		// 0
		48u8 => {
			// ., 9
			if check_bytes(bytes, position, vec![46u8, 47u8])? {
				return Ok(HttpVersion::Version0_9);
			}
		}
		// 1
		49u8 => {
			// .
			check_bytes(bytes, position, vec![46u8])?;

			match get_byte(bytes, position)? {
				// 0
				48u8 => return Ok(HttpVersion::Version1_0),
				// 1
				49u8 => return Ok(HttpVersion::Version1_1),
				_ => {}
			};
		}
		// 2
		50u8 => return Ok(HttpVersion::Version2_0),
		_ => {}
	}

	return Err("6 Malformed HTTP Message".into());
}

fn get_status_code(bytes: &Vec<u8>, position: &mut usize) -> Result<[u8; 3], Box<dyn Error>> {
	let mut status_code: [u8; 3] = [0u8; 3];

	// Will be a space after the version
	if get_byte(bytes, position)? != 32u8 {
		return Err("9.1 Malformed HTTP Message".into());
	}

	for i in 0..3 {
		let byte = get_byte(bytes, position)?;
		// 0-9
		if byte == 48u8
			|| byte == 49u8
			|| byte == 50u8
			|| byte == 51u8
			|| byte == 52u8
			|| byte == 53u8
			|| byte == 54u8
			|| byte == 55u8
			|| byte == 56u8
			|| byte == 57u8
		{
			status_code[i] = byte;
		} else {
			return Err("9 Malformed HTTP Message".into());
		}
	}

	Ok(status_code)
}

fn get_reason_phrase(bytes: &Vec<u8>, position: &mut usize) -> Result<String, Box<dyn Error>> {
	// Will be a space after the code
	if get_byte(bytes, position)? != 32u8 {
		return Err("12 Malformed HTTP Message".into());
	}

	let mut reason_phrase: String = String::new();

	while peek_byte_at_offset(bytes, position, 0)? != 13u8 {
		let byte = get_byte(bytes, position)?;
		reason_phrase.push(byte as char);
	}

	Ok(reason_phrase)
}

fn get_body(bytes: &Vec<u8>, position: &mut usize) -> Result<Option<Vec<u8>>, Box<dyn Error>> {
	check_and_go_past_end_line(bytes, position)?;
	let mut body: Vec<u8> = vec![];

	if bytes.len() == *position {
		return Ok(None);
	}

	for i in *position..bytes.len() {
		body.push(bytes[i]);
	}

	Ok(Some(body))
}

fn check_and_go_past_end_line(bytes: &Vec<u8>, position: &mut usize) -> Result<(), Box<dyn Error>> {
	if check_bytes(bytes, position, vec![13u8, 10u8])? {
		return Ok(());
	}

	return Err("7 Malformed HTTP Message".into());
}

fn get_header_key(bytes: &Vec<u8>, position: &mut usize) -> Result<String, Box<dyn Error>> {
	let mut header_key: String = String::new();
	let mut byte: u8 = get_byte(bytes, position)?;

	while byte != 58u8 {
		header_key.push(byte as char);
		byte = get_byte(bytes, position)?;
	}

	Ok(header_key)
}

fn get_header_value(bytes: &Vec<u8>, position: &mut usize) -> Result<String, Box<dyn Error>> {
	skip_white_space(bytes, position)?;

	let mut header_value: String = String::new();
	let mut byte: u8 = get_byte(bytes, position)?;

	while byte != 13u8 && peek_byte_at_offset(bytes, position, 0)? != 10u8 {
		header_value.push(byte as char);
		byte = get_byte(bytes, position)?;
	}

	increment_position(bytes, position, 1)?;

	Ok(header_value)
}

fn get_headers(bytes: &Vec<u8>, position: &mut usize) -> Result<HashMap<String, String>, Box<dyn Error>> {
	check_and_go_past_end_line(bytes, position)?;

	let mut headers: HashMap<String, String> = HashMap::new();
	let mut done: bool = false;

	while !done {
		let header_key: String = get_header_key(bytes, position)?;
		let header_value: String = get_header_value(bytes, position)?;

		headers.insert(header_key, header_value);

		if peek_byte_at_offset(bytes, position, 0)? == 13u8 && peek_byte_at_offset(bytes, position, 1)? == 10u8 {
			done = true;
		}
	}

	Ok(headers)
}

impl HttpMessage {
	fn new(bytes: &Vec<u8>) -> Result<HttpMessage, Box<dyn Error>> {
		if bytes.len() < 2 {
			return Err("8 Malformed HTTP Message".into());
		}

		let mut position: usize = 0;
		let is_request: bool = determine_request(&bytes)?;

		if is_request {
			let method = get_method(bytes, &mut position)?;
			let resource = get_resource(bytes, &mut position)?;
			let version = get_version(bytes, &mut position)?;
			let headers: HashMap<String, String> = get_headers(bytes, &mut position)?;
			let body: Option<Vec<u8>> = get_body(bytes, &mut position)?;

			let http_message: HttpMessage = Request(HttpRequest {
				version,
				method,
				resource,
				headers,
				body,
			});

			return Ok(http_message);
		} else {
			let version = get_version(bytes, &mut position)?;
			let status_code = get_status_code(bytes, &mut position)?;
			let reason_phrase = get_reason_phrase(bytes, &mut position)?;
			let headers: HashMap<String, String> = get_headers(bytes, &mut position)?;
			let body: Option<Vec<u8>> = get_body(bytes, &mut position)?;

			let http_message: HttpMessage = Response(HttpResponse {
				version,
				status_code,
				reason_phrase,
				headers,
				body,
			});

			return Ok(http_message);
		}
	}
}

pub fn parse_http(bytes: &Vec<u8>) -> Result<HttpMessage, Box<dyn Error>> {
	HttpMessage::new(&bytes)
}
