use crate::store::file;

const PAGE_SIZE: usize = 1024;
type Bytes = Vec<u8>;
//统一实现序列化接口
pub trait ToBytes {
    fn to_Bytes(&self) -> Bytes;
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
    fn to_Bytes(&self) -> Bytes {
        let mut buf: Bytes =  vec![];
        buf.append(&mut self.page_id.to_be_bytes().to_vec());
        buf.append(&mut self.page_size.to_be_bytes().to_vec());
        buf.append(&mut self.check_sum.to_be_bytes().to_vec());
        buf.append(&mut self.transaction_id.to_be_bytes().to_vec());
        buf
    }
}


pub struct DirectoryPage{
    header: PageHeader,
    num_pages: i32, //记录当前表的page总数
    record: Vec<u8>, //记录每一个Page的free slot number
}
impl DirectoryPage {
    pub fn new(page_header: PageHeader) -> Self {
        let dir_page = DirectoryPage {
            header: page_header,
            num_pages: 0,
            record: vec![0; PAGE_SIZE - 20],
        };
        dir_page
    }
}


impl ToBytes for DirectoryPage {
    fn to_Bytes(&self) -> Bytes{
        let mut bytes: Vec<u8> = Vec::new();
        bytes.append(&mut self.header.to_Bytes());
        bytes.append(&mut self.num_pages.to_be_bytes().to_vec());
        bytes.append(&mut self.record.clone());
        bytes
    }
}
pub struct TablePage {
    header: PageHeader,
    slot: Vec<u8>,
    data: Vec<u8>,
}

impl TablePage {
    pub fn new(header: PageHeader, size_tuple: usize) -> Self {
        let num_tuple = get_num_tuple(size_tuple);
        let table_page = TablePage {
            header: header,
            slot: vec![0; num_tuple],
            data: vec![0; PAGE_SIZE - 20 - num_tuple],
        };
        table_page
    }

    pub fn insert_tuple(&mut self, tuple_data: Vec<u8>) {
        let tuple_size = tuple_data.len();
        let iter = tuple_data.clone().into_iter().enumerate();
        //slot[i] == 1 代表对应位置存了tuple
        for (i, ele) in iter {
            if ele == 0 {
                let temp: &mut [u8] = &mut self.data[tuple_size * i..tuple_size * (i + 1)];
                temp.clone_from_slice(&tuple_data[..]);
            }
        }
    }

    pub fn delete_tuple(&mut self, tuple_id: usize) {
        //不需要真的清空对应位置的tuple数据，只需要把对应位置slot置0
        self.slot[tuple_id] = 0;
    }
}

///计算在一个page里可以放多少个tuple
fn get_num_tuple(size_tuple: usize) -> usize{
    (PAGE_SIZE - 20) / (size_tuple + 1)
}
