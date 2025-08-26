use std::{fs, str::FromStr};
use std::path::PathBuf;
use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png};

pub fn encode(fpath: &PathBuf, chunkt_code: &String, message: String) -> crate::Result<()> {
    let png = fs::read(fpath)?;

    let mut png = Png::try_from(png.as_slice())?;
    let chunk_type = ChunkType::from_str(chunkt_code).unwrap();
    let chunk = Chunk::new(chunk_type, message.into_bytes());

    png.append_chunk(chunk);
    fs::write(fpath, png.as_bytes())?;

    Ok(())
}

pub fn decode(fpath: &PathBuf, chunkt_code: &String) -> crate::Result<String> {
    let png = fs::read(fpath)?;
    let png = Png::try_from(png.as_slice())?;

    if let Some(chunk) = png.chunk_by_type(chunkt_code) {
        let data = chunk.data_as_string()?;
        Ok(data)
    } else {
        Ok("Chunk doesn't exist".to_string())
    }
}

pub fn remove(fpath: &PathBuf, chunkt_code: &String) -> crate::Result<Chunk> {
    let png = fs::read(&fpath)?;
    let mut png = Png::try_from(png.as_slice())?;

    if let None = png.chunk_by_type(chunkt_code) {
        return Err("Chunk doesn't exist".into());
    }

    let removed_chunk = png.remove_first_chunk(chunkt_code).unwrap();
    fs::write(fpath, png.as_bytes())?;
    Ok(removed_chunk)
}

pub fn print(fpath: &PathBuf) -> crate::Result<()> {
    let png = fs::read(&fpath)?;
    let png = Png::try_from(png.as_slice())?;

    for chunk in png.chunks() {
        println!("{}", chunk);
    }

    Ok(())
}