use crate::{critical, pretext};
use crate as fox;

use ureq;

pub fn http_code_to_string(code: u16) -> String {
    match code {
        100 => "Continue".to_string(),
        101 => "Switching Protocols".to_string(),
        102 => "Processing".to_string(),
        103 => "Early Hints".to_string(),
        200 => "OK".to_string(),
        201 => "Created".to_string(),
        202 => "Accepted".to_string(),
        203 => "Non-Authoritative Information".to_string(),
        204 => "No Content".to_string(),
        205 => "Reset Content".to_string(),
        206 => "Partial Content".to_string(),
        207 => "Multi-Status".to_string(),
        208 => "Already Reported".to_string(),
        226 => "IM Used".to_string(),
        300 => "Multiple Choices".to_string(),
        301 => "Moved Permanently".to_string(),
        302 => "Found".to_string(),
        303 => "See Other".to_string(),
        304 => "Not Modified".to_string(),
        305 => "Use Proxy".to_string(),
        306 => "Switch Proxy".to_string(),
        307 => "Temporary Redirect".to_string(),
        308 => "Permanent Redirect".to_string(),
        400 => "Bad Request".to_string(),
        401 => "Unauthorized".to_string(),
        402 => "Payment Required".to_string(),
        403 => "Forbidden".to_string(),
        404 => "Not Found".to_string(),
        405 => "Method Not Allowed".to_string(),
        406 => "Not Acceptable".to_string(),
        407 => "Proxy Authentication Required".to_string(),
        408 => "Request Timeout".to_string(),
        409 => "Conflict".to_string(),
        410 => "Gone".to_string(),
        411 => "Length Required".to_string(),
        412 => "Precondition Failed".to_string(),
        413 => "Payload Too Large".to_string(),
        414 => "URI Too Long".to_string(),
        415 => "Unsupported Media Type".to_string(),
        416 => "Range Not Satisfiable".to_string(),
        417 => "Expectation Failed".to_string(),
        418 => "I'm a teapot".to_string(),
        421 => "Misdirected Request".to_string(),
        422 => "Unprocessable Entity".to_string(),
        423 => "Locked".to_string(),
        424 => "Failed Dependency".to_string(),
        425 => "Too Early".to_string(),
        426 => "Upgrade Required".to_string(),
        428 => "Precondition Required".to_string(),
        429 => "Too Many Requests".to_string(),
        431 => "Request Header Fields Too Large".to_string(),
        451 => "Unavailable For Legal Reasons".to_string(),
        500 => "Internal Server Error".to_string(),
        501 => "Not Implemented".to_string(),
        502 => "Bad Gateway".to_string(),
        503 => "Service Unavailable".to_string(),
        504 => "Gateway Timeout".to_string(),
        505 => "HTTP Version Not Supported".to_string(),
        506 => "Variant Also Negotiates".to_string(),
        507 => "Insufficient Storage".to_string(),
        508 => "Loop Detected".to_string(),
        510 => "Not Extended".to_string(),
        511 => "Network Authentication Required".to_string(),
        _ => "Unknown".to_string(),
    }
}

pub fn get(url: &str) -> ureq::Response {
    let response = ureq::get(url).call();

    match response {
        Ok(response) => response,
        Err(error) => {
            match error {
                ureq::Error::Status(code, response) => {
                    let code_string = http_code_to_string(code);
                    match code_string.as_str() {
                        "Unknown" => {
                            critical!("HTTP request failed with status code {code}:\n{:?}", response);
                            std::process::exit(1);
                        }
                        _ => {
                            critical!("HTTP request failed: {code} {code_string}");
                            std::process::exit(1);
                        }
                    }
                }
                ureq::Error::Transport(error) => {
                    critical!("HTTP request failed with transport error: {}", error);
                    std::process::exit(1);
                },
            }
        }
    }
}
