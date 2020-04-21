use file;

type bytes = Vec<u8>;
trait ToBytes {
    fn to_Bytes(&self) -> bytes;
}
#[derive(Copy, Clone)]
pub struct PageHeader {
    page_id: i32,
    page_size: i32,
    check_sum: i32,
    transaction_id: i32,
}

impl PageHeader {
    fn new(page_id: i32, page_size: i32, check_sum: i32, transaction_id: i32) -> Self {
        PageHeader {
            page_id: page_id,
            page_size: page_size,
            check_sum: check_sum,
            transaction_id: transaction_id,
        }
    }
}

impl ToBytes for PageHeader {
    fn to_Bytes(&self) -> bytes {
        let mut buf: bytes =  vec![];
        buf.append(&mut self.page_id.to_be_bytes().to_vec());
        buf.append(&mut self.page_size.to_be_bytes().to_vec());
        buf.append(&mut self.check_sum.to_be_bytes().to_vec());
        buf.append(&mut self.transaction_id.to_be_bytes().to_vec());
        buf
    }
}
pub struct Page {
    page_header: Vec<u8>,
    data: Vec<u8>,
}

pub struct PageManager {}

impl PageManager {
    fn create_Page(file_name: &str) {
        
    }
}




