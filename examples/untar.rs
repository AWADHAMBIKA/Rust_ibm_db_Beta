use std::fs;
use std::io;
use flate2::read::GzDecoder;
use tar::Archive;
use fs_extra::dir::copy;
use fs_extra::dir::CopyOptions;
fn main() {
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    let fname = std::path::Path::new("C:\\Personal\\Rust\\CLI\\macos64_odbc_cli.tar.gz");
    let file = fs::File::open(&fname).unwrap();
    let tar = GzDecoder::new(file);

    let mut archive = Archive::new(tar);
    archive.unpack("C:\\Personal\\Rust\\CLI\\dir1\\");
    //let options = CopyOptions::new(); //Initialize default values for CopyOptions
    //options.copy_inside = true; // To mirror copy the whole structure of the source directory


    // copy source/dir1 to target/dir1
    //copy("C:\\Personal\\Rust\\Rust_C_Sample\\clidriver", "C:\\Personal\\Rust\\CLI\\dir1\\clidriver", &options);

    return 0;
}