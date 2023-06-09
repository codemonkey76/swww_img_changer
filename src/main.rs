use std::process::Command;
use std::fs;
use std::env;
use rand::seq::SliceRandom; // Include this in your Cargo.toml: rand = "0.8.4"
use regex::Regex; // Include this in your Cargo.toml: regex = "1.5.4"

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Usage: {} /path/to/your/backgrounds", args[0]);
    }
    let current_bg = match get_current_background() {
        Ok(bg) => bg,
        Err(e) => {
            eprintln!("Failed to get current background: {}", e);
            return;
        },
    };

    let bg_dir = &args[1];
    println!("{}", bg_dir);

    // List all files in the backgrounds directory
    let paths = fs::read_dir(bg_dir).unwrap();
    let mut all_bgs = Vec::new();
    for path in paths {
        all_bgs.push(path.unwrap().file_name().into_string().unwrap());
    }

    // Filter out the current backgrounds
    let new_bgs: Vec<_> = all_bgs.iter().filter(|&bg| bg != &current_bg).collect();

    // if there are no new images, panic
    if new_bgs.is_empty() {
        panic!("No new backgrounds found!");
    }

    // Select a new backgrounds
    let new_bg = new_bgs.choose(&mut rand::thread_rng()).unwrap();

    // Set the new background
    Command::new("swww")
        .arg("img")
        .arg("--transition-type")
        .arg("grow")
        .arg("--transition-pos")
        .arg("top-right")
        .arg("--transition-angle")
        .arg("45")
        .arg(format!("{}/{}", bg_dir, new_bg))
        .output()
        .expect("Failed to set new background");
}

fn get_current_background() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("swww")
        .arg("query")
        .output()?;

    let re = Regex::new(r#"image: "(.*)""#)?;
    let output_str = String::from_utf8(output.stdout)?;

    match re.captures(&output_str) {
        Some(caps) => Ok(caps.get(1).map_or(String::new(), |m| m.as_str().to_string())),
        None => Ok(String::new()),
    }
}

