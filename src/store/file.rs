use std::io::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
const PAGE_SIZE: usize = 1024;

pub struct FileManager {
    file: Option<File>,
    pages_sum: i64,
}


impl FileManager {
    pub fn new() -> Self{
        FileManager {
            file: None,
            pages_sum: 0,
        }
    }
    pub fn open_file(&mut self, file_path: &str) -> Result<(), io::Error>{
        self.file = Some(File::open(file_path)?);
        Ok(())
    }

    pub fn create_file(&mut self, file_path: &str) -> Result<(), io::Error>{
        self.file = Some(File::create(file_path).expect("file can not be created!"));
        Ok(())
    }

    pub fn allocted_page(&mut self) {
        self.pages_sum = self.pages_sum + 1;
    }

    pub fn read_page(&mut self, page_id: i64) -> io::Result<Vec<u8>> {
        let mut buf = vec![0u8; PAGE_SIZE];
        self.file.as_mut().unwrap().seek(io::SeekFrom::Current(page_id * (PAGE_SIZE as i64)))?;
        self.file.as_mut().unwrap().read(buf.as_mut_slice())?;
        Ok(buf)
    }

    pub fn write_page(&mut self, page_id: i64, data: &[u8]) -> io::Result<()> { 
        self.file.as_mut().unwrap().seek(io::SeekFrom::Current(page_id * (PAGE_SIZE as i64)))?;
        self.file.as_mut().unwrap().write_all(data)?;
        Ok(())
    }

}