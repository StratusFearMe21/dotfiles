use std::{
    io::{stdout, BufWriter},
    os::unix::net::UnixStream,
    path::Path,
};

fn main() {
    let socket_path = Path::new(&std::env::var_os("XDG_RUNTIME_DIR").unwrap()).join("rustbar-0");
    if let Ok(mut socket) = UnixStream::connect(&socket_path) {
        std::io::copy(&mut socket, &mut BufWriter::new(stdout().lock())).unwrap();
    }
}
