use std::io::BufRead;
use randomart::{randomart, Hex, Base64};

fn main() {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    let is_b64 = std::env::args().nth(1) == Some("--base64".to_string());

    for line in stdin.lines() {
	if let Ok(line) = line {
	    if is_b64 {
		let d = Base64::new(&line).unwrap();
		randomart(d);
	    } else {
		let d = Hex::new(&line).unwrap();
		randomart(d);
	    }
	}
    }
}
