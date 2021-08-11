#[cfg(test)]
mod tests {
	use rs_http::parse_http;
	use rs_http::HttpMessage;
	use rs_http::HttpResponse;
	use rs_http::HttpVersion;
	use rs_http::HttpMessage::*;
	use std::collections::HashMap;
	use std::fs;

	#[test]
	fn test_response_one() {
		let path = "./tests/responses/response1.txt".to_string();
		let bytes: Vec<u8> = fs::read(path).unwrap();
		println!("{:?}", bytes);

		let mut headers: HashMap<String, String> = HashMap::new();
		headers.insert(String::from("Date"), String::from("Tue, 10 Aug 2021 21:35:12 GMT"));
		headers.insert(String::from("Content-Type"), String::from("application/json"));
		headers.insert(String::from("Content-Length"), String::from("89"));
		headers.insert(String::from("Connection"), String::from("keep-alive"));
		headers.insert(String::from("x-powered-by"), String::from("PHP/7.3.17"));
		headers.insert(String::from("cache-control"), String::from("no-cache, private"));
		headers.insert(String::from("access-control-allow-origin"), String::from("*"));
		headers.insert(String::from("via"), String::from("1.1 varnish (Varnish/6.3), 1.1 varnish (Varnish/6.3)"));
		headers.insert(String::from("x-cache-hits"), String::from("0"));
		headers.insert(String::from("x-cache"), String::from("MISS"));
		headers.insert(String::from("accept-ranges"), String::from("bytes"));
		headers.insert(String::from("age"), String::from("0"));
		headers.insert(String::from("vary"), String::from(""));
		headers.insert(String::from("CF-Cache-Status"), String::from("DYNAMIC"));
		headers.insert(
			String::from("Expect-CT"),
			String::from("max-age=604800, report-uri=\"https://report-uri.cloudflare.com/cdn-cgi/beacon/expect-ct\""),
		);
		headers.insert(String::from("Report-To"), String::from("{\"endpoints\":[{\"url\":\"https:\\/\\/a.nel.cloudflare.com\\/report\\/v3?s=lFwSiru%2B1geWCkhFpObAz4w3VgTwzwZkTl2IsXkTjRLcrMcMlVmK7wkJQjYsM0tk%2B2dzSdeTIli2648SWOwglZ%2FU0Vrp%2BW2OXfJCPsmV8yZzpHplu%2FLTqKX1\"}],\"group\":\"cf-nel\",\"max_age\":604800}"));
		headers.insert(String::from("NEL"), String::from("{\"success_fraction\":0,\"report_to\":\"cf-nel\",\"max_age\":604800}"));
		headers.insert(String::from("Server"), String::from("cloudflare"));
		headers.insert(String::from("CF-RAY"), String::from("67cc5a46fc002ae2-ORD"));
		headers.insert(
			String::from("alt-svc"),
			String::from("h3-27=\":443\"; ma=86400, h3-28=\":443\"; ma=86400, h3-29=\":443\"; ma=86400, h3=\":443\"; ma=86400"),
		);

		let body: Vec<u8> = vec![
			123u8, 34u8, 109u8, 101u8, 115u8, 115u8, 97u8, 103u8, 101u8, 34u8, 58u8, 34u8, 104u8, 116u8, 116u8, 112u8, 115u8, 58u8, 92u8, 47u8, 92u8, 47u8, 105u8,
			109u8, 97u8, 103u8, 101u8, 115u8, 46u8, 100u8, 111u8, 103u8, 46u8, 99u8, 101u8, 111u8, 92u8, 47u8, 98u8, 114u8, 101u8, 101u8, 100u8, 115u8, 92u8, 47u8,
			109u8, 97u8, 115u8, 116u8, 105u8, 102u8, 102u8, 45u8, 101u8, 110u8, 103u8, 108u8, 105u8, 115u8, 104u8, 92u8, 47u8, 49u8, 46u8, 106u8, 112u8, 103u8, 34u8,
			44u8, 34u8, 115u8, 116u8, 97u8, 116u8, 117u8, 115u8, 34u8, 58u8, 34u8, 115u8, 117u8, 99u8, 99u8, 101u8, 115u8, 115u8, 34u8, 125u8,
		];

		let http_message: HttpMessage = Response(HttpResponse {
			version: HttpVersion::Version1_1,
			status_code: [50u8, 48u8, 48u8],
			reason_phrase: String::from("OK"),
			headers,
			body: Some(body),
		});

		let parsed_http_message: HttpMessage = parse_http(&bytes).unwrap();

		assert_eq!(http_message, parsed_http_message);
	}
}
