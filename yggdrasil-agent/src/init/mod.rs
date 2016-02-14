use std::fs;
use std::path::Path;
use std::io;
use std::process::*;

use hyper::Client;

pub fn run(register_url: String) -> Result<(), String> {
    try!(get_steamcmd());
    return Ok(());
}

#[cfg(windows)]
fn get_steamcmd() -> Result<(), String> {
    let steamcmd = Path::new("./steamcmd.exe");
    if !fs::metadata(steamcmd).is_ok() {
        let client = Client::new();
        let mut res = match client.get("https://steamcdn-a.akamaihd.net/client/installer/steamcmd.zip").send() {
            Ok(steamcmd_zip) => { steamcmd_zip }
            Err(msg) => { return Err(format!("Error getting steamcmd {}", msg)) }
        };

        let mut zip_file = match fs::File::create("steamcmd.zip") {
            Ok(file) => { file }
            Err(e) => { return Err(format!("Error saving steamcmd {}", e)) }
        };

        match io::copy(&mut res, &mut zip_file) {
            Ok(_) => { }
            Err(e) => { return Err(format!("Error saving steamcmd {}", e)) }
        };

        zip_file.sync_all().unwrap();

        let output = match Command::new("unzip.exe").arg("steamcmd.zip").output() {
            Ok(out) => { out }
            Err(e) => { return Err(format!("Failed to extract steamcmd {}", e)) }
        };

        if !output.status.success() {
            return Err(format!("Error extracting steamcmd {}", String::from_utf8(output.stderr).unwrap()));
        }
    }
    return Ok(());
}

#[cfg(not(windows))]
fn get_steamcmd() -> Result<(), String> {
    let steamcmd = Path::new("./steamcmd.sh");
    if !fs::metadata(steamcmd).is_ok() {
        let client = Client::new();
        let mut res = match client.get("https://steamcdn-a.akamaihd.net/client/installer/steamcmd_linux.tar.gz").send() {
            Ok(steamcmd_zip) => { steamcmd_zip }
            Err(msg) => { return Err(format!("Error getting steamcmd {}", msg)) }
        };

        let mut tarball = match fs::File::create("steamcmd.tar.gz") {
            Ok(file) => { file }
            Err(e) => { return Err(format!("Error saving steamcmd {}", e)) }
        };

        match io::copy(&mut res, &mut tarball) {
            Ok(_) => { }
            Err(e) => { return Err(format!("Error saving steamcmd {}", e)) }
        };

        tarball.sync_all().unwrap();

        let output = match Command::new("tar").arg("-xvzf").arg("steamcmd.tar.gz").output() {
            Ok(out) => { out }
            Err(e) => { return Err(format!("Failed to extract steamcmd {}", e)) }
        };

        if !output.status.success() {
            return Err(format!("Error extracting steamcmd {}", String::from_utf8(output.stderr).unwrap()));
        }
    }
    return Ok(());
}
