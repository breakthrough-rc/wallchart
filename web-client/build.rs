use std::{
    env,
    process::Command,
    fs,
    io::Write,
};

fn main() {
    println!("Building client scripts");

    let out_dir = env::var("OUT_DIR").unwrap();
    let hash = hash_package_json();

    let should_bun_install = match read_hash_from_out(&out_dir) {
        Ok(old_hash) => old_hash != hash,
        Err(_) => true
    };

    if should_bun_install {
        write_hash_to_out(&out_dir, &hash);
        exec_bun_install();
    }

    exec_build_js_and_css();
}

fn exec_bun_install() {
    let output = Command::new("sh")
        .args(["-c", "bun install"])
        .output()
        .expect("failed to execute `bun install`"); 
    
    println!("{:?}", output);
}

fn exec_build_js_and_css() {
    let output = Command::new("sh")
        .args(["-c", "bunx tailwindcss -i ./src/client/common.css -o ./out/common.css && bun build ./src/client/common.js"])
        .output()
        .expect("failed to execute tailwind and bun build process");

    println!("{:?}", output);
}

fn read_hash_from_out(out_dir: &str) -> Result<String, std::io::Error> {
    let path = format!("{}/package_json.txt", out_dir);
    fs::read_to_string(path)
}

fn write_hash_to_out(out_dir: &str, hash: &str) {
    let file_path = format!("{}/package_json.txt", out_dir);
    let mut file = fs::File::create(file_path).unwrap();
    write!(file, "{}", hash).unwrap();
}

fn hash_package_json() -> String {
    let bytes = fs::read("package.json").unwrap();
    sha256::digest(&bytes)
}
