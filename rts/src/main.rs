use rtslexer::tokenize;
use std::fs::File;
use std::io::prelude::*;
use rtschunk::Chunk;
use rtsvm::VM;

fn main() -> std::io::Result<()> {
    //加载源码
    let path = "scripts/main.rs";
    let mut source_file = File::open(path)?;

    let mut source_str = String::new();

    source_file.read_to_string(&mut source_str)?;
    
    let mut tokenIter = tokenize(&source_str);
    for token in tokenIter {
        println!("{:?}{}", token.kind, token.len);
    }
    println!("{:?}", source_str);
    // 编译输出二进制
    let mut bin_chunk = Chunk::new();
    let mut chunk_file = File::create("main.chunk")?;
    chunk_file.write(&(bin_chunk.as_bytes_vec()));
    // 运行
    let mut vm =  VM::new();
    // 初始化加载
    vm.load_op_code(0, Chunk::new().as_bytes_vec());
    vm.run();
    Ok(())
}
