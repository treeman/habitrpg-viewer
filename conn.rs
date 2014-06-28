use std::io::process::Command;
use std::str;

use id::Id;

// Wrap GET requests and use curl.
// Did not find a stable library for it.
pub fn get(url: &str, id: &Id) -> String {
    let output = match Command::new("curl")
        .arg("-X").arg("GET")
        .arg("-H").arg(format!("x-api-key: {}", id.api_token))
        .arg("-H").arg(format!("x-api-user: {}", id.user_id))
        .arg(url).output()
    {
        Ok(output) => output,
        Err(e) => fail!("failed to execute process: {}", e),
    };

    let out = str::from_utf8_lossy(output.output.as_slice()).to_string();

    //println!("status: {}", output.status);
    //println!("stdout: {}", out);
    //println!("stderr: {}", str::from_utf8_lossy(output.error.as_slice()));

    out
}
