use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::Path,
};

fn main() {
    let path: &Path = Path::new("./Geneva.ttf");
    let mut f: File = std::fs::File::open(path).expect("Could not open the file");
    let slice = read_n(&mut f, 4, 2);
    println!("{:?}", slice);
    let num_tables = (slice[0] as u16) << 8 | slice[1] as u16;
    println!("{}", num_tables);
}

fn read_n<R>(reader: &mut R, offset: u64, bytes_to_read: u64) -> Vec<u8>
where
    R: Read + Seek,
{
    let mut buf: Vec<u8> = vec![];
    reader
        .seek(SeekFrom::Start(offset))
        .expect("This went wrong");
    let mut chunk = reader.take(bytes_to_read);
    let n = chunk.read_to_end(&mut buf).expect("This went wrong");
    assert_eq!(bytes_to_read as usize, n);
    buf
}
