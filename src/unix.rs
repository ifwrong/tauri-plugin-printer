use std::io::Write;
use std::env;
use std::process::Command;
use crate::declare::PrintOptions;

/**
 * Create sm file to temp
 */
fn create_file(path: String, bin: &[u8]) -> std::io::Result<()> {
    let mut f = std::fs::File::create(format!("{}sm", path))?;
    f.write_all(bin)?;
    f.sync_all()?;
    Ok(())
}

/**
 * Init sm file
 */
pub fn init_unix() {
    let sm = include_bytes!("bin/sm");
    let dir: std::path::PathBuf = env::temp_dir();
    let result: Result<(), std::io::Error>  = create_file(dir.display().to_string(),sm);
    if result.is_err() {
        panic!("Failed to create sm file")
    }
}

/**
 * Get printers on macOS using lpstat
 */
pub fn get_printers() -> String {
    let output = Command::new("lpstat").arg("-p").output().unwrap();
    return String::from_utf8(output.stdout).unwrap();
}

/**
 * Get printers by name on macOS using lpstat
 */
pub fn get_printers_by_name(printername: String) -> String {
    let output = Command::new("lpstat").args(["-p", printername.as_str()]).output().unwrap();
    return String::from_utf8(output.stdout).unwrap();
}

/**
 * Print pdf file 
 */
pub fn print_pdf (options: PrintOptions) -> String {
    let dir = env::temp_dir();
    let print_setting = format!(
        "-o media={},{} -o scaling={} -o orientation-requested={} -o fit-to-page",
        options.print_setting.paper,
        options.print_setting.method,
        options.print_setting.scale,
        options.print_setting.orientation,
    );
    let shell_command = format!("{}sm -t {} {} -q {}", dir.display(), options.id, print_setting, options.path);
    let output = Command::new("sh").arg("-c").arg(shell_command).output().unwrap();
    return String::from_utf8(output.stdout).unwrap();
}

/**
 * Get printer job on macOS using lpq
 */
pub fn get_jobs(printername: String) -> String {
    let output = Command::new("lpq").args([printername.as_str()]).output().unwrap();
    return String::from_utf8(output.stdout).unwrap();
}

/**
 * Get printer job by id on macOS using lpq
 */
pub fn get_jobs_by_id(printername: String, jobid: String) -> String {
    let output = Command::new("lpq").args([format!("-P{}", printername), jobid.to_string()]).output().unwrap();
    return String::from_utf8(output.stdout).unwrap();
}

/**
 * Resume printer job on macOS using lprm
 */
pub fn resume_job(printername: String, jobid: String) -> String {
    let output = Command::new("lprm").args([format!("-P{}", printername), jobid.to_string()]).output().unwrap();
    return String::from_utf8(output.stdout).unwrap();
}

/**
 * Restart printer job on macOS using lprm
 */
pub fn restart_job(printername: String, jobid: String) -> String {
    let output = Command::new("lprm").args([format!("-P{}", printername), "-i".to_string(), jobid.to_string()]).output().unwrap();
    return String::from_utf8(output.stdout).unwrap();
}

/**
 * Pause printer job on macOS using lprm
 */
pub fn pause_job(printername: String, jobid: String) -> String {
    let output = Command::new("lprm").args([format!("-P{}", printername), "-H".to_string(), jobid.to_string()]).output().unwrap();
    return String::from_utf8(output.stdout).unwrap();
}

/**
 * Remove printer job on macOS using lprm
 */
pub fn remove_job(printername: String, jobid: String) -> String {
    let output = Command::new("lprm").args([format!("-P{}", printername), "-U".to_string(), jobid.to_string()]).output().unwrap();
    return String::from_utf8(output.stdout).unwrap();
}