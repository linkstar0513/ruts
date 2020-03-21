// 二进制类型
pub enum ChunkType {
    Binary,
    Lib,
}
// 二进制格式
pub struct Chunk{
    chunk_type: ChunkType,
    main: usize,
}
impl Chunk {
    pub fn new() -> Self{
        Chunk{
            chunk_type: ChunkType::Binary,
            main: 0,
        }
    }
    pub fn as_bytes_vec(&mut self) -> Vec<u8>{
        let mut chunk = vec![];
        let mut chunk_string = "ChunkType:\n".to_string();
        chunk_string.push_str("main:0");
        chunk = chunk_string.into_bytes();
        chunk
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
