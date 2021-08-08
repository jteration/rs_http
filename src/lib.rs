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
	headers: HashMap<String, String>
}

fn increment_position(bytes: &Vec<u8>, position: &mut usize, increment_by: usize) -> Result<(), Box<dyn Error>> {
	// Check if new position is past the end of the json string
	if *position + increment_by > bytes.len() {
			return Err("Unexpectedly reached end of request".into());
	}

	*position += increment_by;

	Ok(())
}

fn get_byte_at_offset(bytes: &Vec<u8>, position: &usize, offset: usize) -> Result<u8, Box<dyn Error>> {
	// Check if char is past the end of the json string
	if *position + offset > bytes.len() - 1 {
		return Err("Unexpectedly reached end of request".into());
	}

	Ok(bytes[*position + offset])
}

fn determine_request(bytes: &Vec<u8>) -> Result<bool, Box<dyn Error>> {
	match bytes[0] {
		72u8 => {
			match bytes[1] {
				84u8 => Ok(false),
				69u8 => Ok(true),
				_ => Err("Malformed HTTP Message".into())
			}
		},
		71u8 | 80u8 | 68u8 | 79u8 => Ok(true),
		_ => Err("Malformed HTTP Message".into())
	}
}

fn get_method(bytes: &Vec<u8>, position: &mut usize) -> Result<HttpMethod, Box<dyn Error>> {
	match get_byte_at_offset(bytes, position, 0)? {
		// G
		71u8 => {
			// E, T, space
			if
				get_byte_at_offset(bytes, position, 1)? == 69u8 &&
				get_byte_at_offset(bytes, position, 2)? == 84u8 &&
				get_byte_at_offset(bytes, position, 3)? == 32u8
			{
				increment_position(bytes, position, 4)?;

				Ok(HttpMethod::GET)
			} else {
				Err("Malformed HTTP Message".into())
			}
		},
		// P
		80u8 => {
			match get_byte_at_offset(bytes, position, 1)? {
				// U
				85u8 => {
					// T, space
					if
						get_byte_at_offset(bytes, position, 2)?  == 84u8 &&
						get_byte_at_offset(bytes, position, 3)?  == 32u8
					{
						increment_position(bytes, position, 4)?;
		
						Ok(HttpMethod::PUT)
					} else {
						Err("Malformed HTTP Message".into())
					}
				},
				// O
				79u8 => {
					// S, T, space
					if
						get_byte_at_offset(bytes, position, 2)?  == 83u8 &&
						get_byte_at_offset(bytes, position, 3)?  == 84u8 &&
						get_byte_at_offset(bytes, position, 4)?  == 32u8
					{
						increment_position(bytes, position, 5)?;
		
						Ok(HttpMethod::POST)
					} else {
						Err("Malformed HTTP Message".into())
					}
				},
				// A
				65u8 => {
					// T, C, H, space
					if
						get_byte_at_offset(bytes, position, 2)?  == 84u8 &&
						get_byte_at_offset(bytes, position, 3)?  == 67u8 &&
						get_byte_at_offset(bytes, position, 4)?  == 72u8 &&
						get_byte_at_offset(bytes, position, 5)?  == 32u8
					{
						increment_position(bytes, position, 6)?;
		
						Ok(HttpMethod::PATCH)
					} else {
						Err("Malformed HTTP Message".into())
					}
				},
				_ => Err("Malformed HTTP Message".into())
			}
		},
		// H
		72u8 => {
			// E, A, D, space
			if
				get_byte_at_offset(bytes, position, 2)?  == 69u8 &&
				get_byte_at_offset(bytes, position, 3)?  == 65u8 &&
				get_byte_at_offset(bytes, position, 4)?  == 68u8 &&
				get_byte_at_offset(bytes, position, 5)?  == 32u8 
			{
				increment_position(bytes, position, 6)?;

				Ok(HttpMethod::HEAD)
			} else {
				Err("Malformed HTTP Message".into())
			}
		},
		// D
		68u8 => {
			// E, L, E, T, E, space
			if
				get_byte_at_offset(bytes, position, 2)?  == 69u8 &&
				get_byte_at_offset(bytes, position, 3)?  == 76u8 &&
				get_byte_at_offset(bytes, position, 4)?  == 69u8 &&
				get_byte_at_offset(bytes, position, 5)?  == 84u8 &&
				get_byte_at_offset(bytes, position, 6)?  == 69u8 &&
				get_byte_at_offset(bytes, position, 7)?  == 32u8 
			{
				increment_position(bytes, position, 8)?;

				Ok(HttpMethod::DELETE)
			} else {
				Err("Malformed HTTP Message".into())
			}
		},
		// O
		79u8 => {
			// P, T, I, O, N, S, space
			if
				get_byte_at_offset(bytes, position, 2)?  == 80u8 &&
				get_byte_at_offset(bytes, position, 3)?  == 84u8 &&
				get_byte_at_offset(bytes, position, 4)?  == 73u8 &&
				get_byte_at_offset(bytes, position, 5)?  == 79u8 &&
				get_byte_at_offset(bytes, position, 6)?  == 78u8 &&
				get_byte_at_offset(bytes, position, 7)?  == 83u8 &&
				get_byte_at_offset(bytes, position, 8)?  == 32u8
			{
				increment_position(bytes, position, 9)?;

				Ok(HttpMethod::OPTIONS)
			} else {
				Err("Malformed HTTP Message".into())
			}
		},
		_ => Err("Malformed HTTP Message".into())
	}
}

fn get_resource(bytes: &Vec<u8>, position: &mut usize) -> Result<String, Box<dyn Error>> {
	if bytes[*position] != 47u8 {
		return Err("Malformed HTTP Message".into());
	}

	let mut resource = String::new();

	while bytes[*position] != 32u8 {
		resource.push(bytes[*position] as char);
		increment_position(bytes, position, 1)?;
	}

	increment_position(bytes, position, 1)?;

	Ok(resource)
}

fn get_version(bytes: &Vec<u8>, position: &mut usize) -> Result<HttpVersion, Box<dyn Error>> {
	let http_check: [u8; 5] = [72u8, 84u8, 84u8, 80u8, 47u8];

	for byte in http_check {
		if bytes[*position] != byte {
			return Err("Malformed HTTP Message".into());
		}

		increment_position(bytes, position, 1)?;
	}

	match bytes[*position] {
		// 0
		48u8 => {
			// ., 9
			if
				get_byte_at_offset(bytes, position, 1)? == 46u8 &&
				get_byte_at_offset(bytes, position, 2)? == 57u8
			{
				increment_position(bytes, position, 3)?;
				return Ok(HttpVersion::Version0_9);
			}

			return Err("Malformed HTTP Message".into());
		},
		// 1
		49u8 => {
			// .
			if get_byte_at_offset(bytes, position, 1)? != 46u8 {
				return Err("Malformed HTTP Message".into());
			}

			match get_byte_at_offset(bytes, position, 2)? {
				// 0
				48u8 => {
					increment_position(bytes, position, 3)?;
					return Ok(HttpVersion::Version1_0);
				},
				// 1
				49u8 => {
					increment_position(bytes, position, 3)?;
					return Ok(HttpVersion::Version1_1);
				},
				_ => return Err("Malformed HTTP Message".into())
			}
		},
		// 2
		50u8 => {
			// ., 0
			if
				get_byte_at_offset(bytes, position, 1)? == 46u8 &&
				get_byte_at_offset(bytes, position, 2)? == 48u8
			{
				increment_position(bytes, position, 3)?;
				return Ok(HttpVersion::Version2_0);
			} else {
				return Err("Malformed HTTP Message".into());
			}
		},
		_ => return Err("Malformed HTTP Message".into())
	}
}

fn check_and_go_past_end_line(bytes: &Vec<u8>, position: &mut usize) -> Result<(), Box<dyn Error>> {
	if
		bytes[*position] == 13u8 &&
		get_byte_at_offset(bytes, position, 1)? == 10u8
	{
		increment_position(bytes, position, 2)?;
		return Ok(());
	}

	return Err("Malformed HTTP Message".into());
}

fn get_header_key(bytes: &Vec<u8>, position: &mut usize) -> Result<String, Box<dyn Error>> {
	let mut header_key: String = String::new();

	while bytes[*position] != 58u8 {
		header_key.push(bytes[*position] as char);
		increment_position(bytes, position, 1)?;
	}

	increment_position(bytes, position, 1)?;

	if bytes[*position] == 32u8 {
		increment_position(bytes, position, 1)?;
	}

	Ok(header_key)
}

fn get_header_value(bytes: &Vec<u8>, position: &mut usize) -> Result<String, Box<dyn Error>> {
	let mut header_value: String = String::new();

	while bytes[*position] != 13u8 {
		header_value.push(bytes[*position] as char);
		increment_position(bytes, position, 1)?;
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
		let mut position: usize = 0;
		let is_request: bool = determine_request(&bytes)?;
		let mut method: Option<HttpMethod> = None;
		let mut resource: Option<String> = None;
		let version: HttpVersion;
		let mut status_code: Option<[u8; 3]> = None;
		let mut status_message: Option<String> = None;

		// Start Line
		if is_request {
			method = Some(get_method(bytes, &mut position)?);
			resource = Some(get_resource(bytes, &mut position)?);
			version = get_version(bytes, &mut position)?;
			check_and_go_past_end_line(bytes, &mut position)?;
		} else {
			version = get_version(bytes, &mut position)?;
			check_and_go_past_end_line(bytes, &mut position)?;
			// get_status_code
			// get_status_message
		}

		let headers: HashMap<String, String> = determine_headers(bytes, &mut position)?;

		let http_message: HttpMessage = HttpMessage {
			request: is_request,
			version,
			method,
			resource,
			status_code,
			status_message,
			headers
		};

		Ok(http_message)
	}
}

pub fn parse_http(bytes: &Vec<u8>) -> Result<HttpMessage, Box<dyn Error>> {
	HttpMessage::new(&bytes)
}
