use std::fs::OpenOptions;
use std::io::{self, Seek, SeekFrom, Write};

fn main() -> io::Result<()> {
  let mut file = OpenOptions::new()
    .read(true)
    .write(true)
    .open(r"..\Fusion364\BetterKega.exe")?;

  file.seek(SeekFrom::Start(0xE77B0))?;
  file.write_all(&[0x00; 0xE77C8 - 0xE77B0 + 1])?;
  let mut buffer = [0x00; 0xE77D8 - 0xE77C8 + 1];
  let text = b"BetterKega ";
  buffer[..text.len()].copy_from_slice(text);
  file.seek(SeekFrom::Start(0xE77C8))?;
  file.write_all(&buffer)?;

  Ok(())
}
