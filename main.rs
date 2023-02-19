use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;

/*fn get_file_as_byte_vec(filename: &'static str) -> Vec<u8> {
    let buffer = fs::read(filename).unwrap();

    return buffer;
}*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let filename = "ips_plaintext_rust.txt";
    let ips_url = "https://github.com/mat-1/minecraft-scans/blob/main/ips?raw=true";

    let print = false;

    let _ = fs::remove_file(filename);

    //let buf = get_file_as_byte_vec("ips.bin");

    let buf = reqwest::get(ips_url)
        .await?
        .bytes()
        .await?;

    let mut write_out = String::new();

    for chunk in buf.chunks(6) {
        let port = u16::from(chunk[4]) * 256 + u16::from(chunk[5]);

        if print {
            println!(
                "IP: {}.{}.{}.{} | Port: {}",
                chunk[0], chunk[1], chunk[2], chunk[3], port
            );
        }

        let append = format!("{}.{}.{}.{}:{}\n", chunk[0], chunk[1], chunk[2], chunk[3], port);
        write_out.push_str(&append);
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(filename)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", write_out) {
        eprintln!("Couldn't write to file: {}", e);
    }

    Ok(())
}
