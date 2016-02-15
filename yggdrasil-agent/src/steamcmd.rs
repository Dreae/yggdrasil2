use std::process::*;

#[cfg(windows)]
#[inline(always)]
fn steamcmd() -> String {
    "steamcmd.exe".to_owned()
}

#[cfg(not(windows))]
#[inline(always)]
fn steamcmd() -> String {
    "steamcmd.sh".to_owned()
}

pub fn install_game(app_id: u32, install_dir: String) -> Result<Child, String> {
    match Command::new(steamcmd())
            .arg("+login anonymous")
            .arg(format!("+force_install_dir {}", install_dir))
            .arg(format!("+app_update {}", app_id))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn() {
        Ok(child) => { Ok(child) }
        Err(msg) => { Err(format!("Error spawning steamcmd {}", msg)) }
    }
}
