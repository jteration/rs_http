#[cfg(test)]
mod tests {
	use std::fs;
	use rs_http::parse_http;
	use rs_http::HttpMessage;

	#[test]
	fn test_response_one() {
		let path = "./tests/responses/response1.txt".to_string();
		let bytes: Vec<u8> = fs::read(path).unwrap();
		println!("{:?}", bytes);

		let http_message: HttpMessage = parse_http(&bytes).unwrap();

		println!("{:?}", http_message);

		assert_eq!(true, false);
	}
}
