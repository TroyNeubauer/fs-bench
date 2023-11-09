use std::io::Read;

pub fn test() {
    let mut f = std::fs::File::open("Cargo.toml").unwrap();

    let mut buf = [0u8; 1024];

    f.read(&mut buf).unwrap();
}
