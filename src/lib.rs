use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
enum HttpVersion {
	Version0_9,
	Version1_0,
	Version1_1,
	Version2_0,
}

#[derive(Debug)]
enum HttpMethod {
	GET,
	PUT,
	POST,
	HEAD,
	DELETE,
	PATCH,
	OPTIONS,
}

#[derive(Debug)]
pub struct HttpMessage {
	request: bool,
	version: HttpVersion,
	method: Option<HttpMethod>,
	resource: Option<String>,
	status_code: Option<[u8; 3]>,
	status_message: Option<String>,
	headers: HashMap<String, String>,
}

fn increment_position(bytes: &Vec<u8>, position: &mut usize, increment_by: usize) -> Result<(), Box<dyn Error>> {
	// Check if new position is past the end of the json string
	if *position + increment_by > bytes.len() {
		return Err("Unexpectedly reached end of request".into());
	}

	*position += increment_by;

	Ok(())
}

fn get_byte(bytes: &Vec<u8>, position: &mut usize) -> Result<u8, Box<dyn Error>> {
	let b = bytes[*position];
	increment_position(bytes, position, 1)?;

	return Ok(b);
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

fn is_white_space(b: u8) -> bool {
	b == 32u8
}

fn skip_white_space(bytes: &Vec<u8>, position: &mut usize) -> Result<(), Box<dyn Error>> {
	while is_white_space(bytes[*position]) {
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
			_ => Err("Malformed HTTP Message".into()),
		},
		71u8 | 80u8 | 68u8 | 79u8 => Ok(true),
		_ => Err("Malformed HTTP Message".into()),
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
					if check_bytes(bytes, position, vec![83u8, 34u8, 32u8])? {
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

	Err("Malformed HTTP Message".into())
}

fn get_resource(bytes: &Vec<u8>, position: &mut usize) -> Result<String, Box<dyn Error>> {
	if bytes[*position] == 32u8 {
		return Err("Malformed HTTP Message".into());
	}

	let mut resource = String::new();

	while get_byte(bytes, position)? != 32u8 {
		resource.push(bytes[*position] as char);
	}

	increment_position(bytes, position, 1)?;

	Ok(resource)
}

fn get_version(bytes: &Vec<u8>, position: &mut usize) -> Result<HttpVersion, Box<dyn Error>> {
	// H, T, T, P, /
	check_bytes(bytes, position, vec![72u8, 84u8, 84u8, 80u8, 47u8])?;

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
				_ => ()
			};
		}
		// 2
		50u8 => return Ok(HttpVersion::Version2_0),
		_ => ()
	}

	return Err("Malformed HTTP Message".into());
}

fn check_and_go_past_end_line(bytes: &Vec<u8>, position: &mut usize) -> Result<(), Box<dyn Error>> {
	if check_bytes(bytes, position, vec![13u8, 10u8])? {
		return Ok(());
	}

	return Err("Malformed HTTP Message".into());
}

fn get_header_key(bytes: &Vec<u8>, position: &mut usize) -> Result<String, Box<dyn Error>> {
	let mut header_key: String = String::new();

	while get_byte(bytes, position)? != 58u8 {
		header_key.push(bytes[*position] as char);
	}

	increment_position(bytes, position, 1)?;

	Ok(header_key)
}

fn get_header_value(bytes: &Vec<u8>, position: &mut usize) -> Result<String, Box<dyn Error>> {
	let mut header_value: String = String::new();

	skip_white_space(bytes, position)?;

	while get_byte(bytes, position)? != 13u8 {
		header_value.push(bytes[*position] as char);
	}

	Ok(header_value)
}

fn determine_headers(bytes: &Vec<u8>, position: &mut usize) -> Result<HashMap<String, String>, Box<dyn Error>> {
	let mut headers: HashMap<String, String> = HashMap::new();
	let mut done: bool = false;

	while !done {
		let header_key: String = get_header_key(bytes, position)?;
		let header_value: String = get_header_value(bytes, position)?;
		check_and_go_past_end_line(bytes, position)?;

		headers.insert(header_key, header_value);

		if bytes[*position] == 13u8 && bytes[*position + 1] == 10u8 {
			done = true;
		}
	}

	Ok(headers)
}

impl HttpMessage {
	fn new(bytes: &Vec<u8>) -> Result<HttpMessage, Box<dyn Error>> {
		if bytes.len() < 2 {
			return Err("Malformed HTTP Message".into());
		}

		let mut position: usize = 0;
		let is_request: bool = determine_request(&bytes)?;
		let mut method: Option<HttpMethod> = None;
		let mut resource: Option<String> = None;
		let version: HttpVersion;
		let status_code: Option<[u8; 3]> = None;
		let status_message: Option<String> = None;

		// Start Line
		if is_request {
			method = Some(get_method(bytes, &mut position)?);
			resource = Some(get_resource(bytes, &mut position)?);
			version = get_version(bytes, &mut position)?;
			check_and_go_past_end_line(bytes, &mut position)?;
		} else {
			version = get_version(bytes, &mut position)?;
			// get_status_code
			// get_reason_phrase
			check_and_go_past_end_line(bytes, &mut position)?;
		}

		let headers: HashMap<String, String> = determine_headers(bytes, &mut position)?;

		let http_message: HttpMessage = HttpMessage {
			request: is_request,
			version,
			method,
			resource,
			status_code,
			status_message,
			headers,
		};

		Ok(http_message)
	}
}

pub fn parse_http(bytes: &Vec<u8>) -> Result<HttpMessage, Box<dyn Error>> {
	HttpMessage::new(&bytes)
}
