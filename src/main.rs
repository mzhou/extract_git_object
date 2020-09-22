use std::io::prelude::*;
use std::io::{Error, ErrorKind};

use flate2::read::ZlibDecoder;

fn generic_error() -> Error {
    Error::new(ErrorKind::Other, "extract_git_object")
}

fn main() -> Result<(), std::io::Error> {
    let mut d = ZlibDecoder::new(std::io::stdin());
    let mut buf = [0u8; 512 * 1024];

    // check and skip header
    let init_size = d.read(&mut buf)?;
    if init_size < 7 {
        eprintln!("first chunk too small");
        return Err(generic_error());
    }
    if buf[0] != b'b' || buf[1] != b'l' || buf[2] != b'o' || buf[3] != b'b' || buf[4] != b' ' {
        eprintln!("invalid blob magic");
        return Err(generic_error());
    }

    // parse the blob size
    let mut i = 5;
    let mut blob_size = 0usize;
    while i < init_size {
        if i != 5 && buf[i] == b'\0' {
            i += 1;
            break;
        }
        if buf[i] < b'0' || buf[i] > b'9' {
            eprintln!("invalid character {} in blob size at {}", buf[i], i);
            return Err(generic_error());
        }
        blob_size = 10 * blob_size + (buf[i] - b'0') as usize;
        i += 1;
    }
    // i points past the NUL
    if blob_size == 0 {
        eprintln!("empty blob");
        return Ok(());
    }
    eprintln!("blob size {}", blob_size);

    let mut written_size = 0usize;

    // output remainder of first chunk
    if i < init_size {
        std::io::stdout().write_all(&buf[i..init_size])?;
        written_size += init_size - i;
    }

    loop {
        let read_size = d.read(&mut buf)?;
        if read_size == 0 {
            break;
        }
        std::io::stdout().write_all(&buf[..read_size])?;
        written_size += read_size;
    }

    if written_size != blob_size {
        eprintln!("blob size {} but wrote {}", blob_size, written_size);
        return Err(generic_error());
    }

    Ok(())
}
