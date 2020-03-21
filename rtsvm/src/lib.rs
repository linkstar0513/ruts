
pub struct VM{
    stack: Vec<u8>,
    pc: usize,
    op:Vec<u8>,

}
impl VM{
    pub fn pc(&mut self) -> usize{
        self.pc   
    }
    pub fn jmp(&mut self, position: usize){
        self.pc = position;
    }
    pub fn run(&mut self){
        let mut next_op = self.pc + 1;
        loop{
            if let Some(op) = self.op.get_mut(self.pc){
                println!("{}", *op);
                next_op = self.pc + 1;
                self.jmp(next_op);
            }else{
                break
            } 
        }
        
    }
}
impl VM {
    pub fn new() -> Self {
        VM{
            stack: Vec::with_capacity(1024),
            pc: 0,
            op:vec![],
        }
    }
    // 加载内存值
    pub fn load(&mut self, ptr: usize, value: Vec<u8>){
        let mut target = self.stack.as_mut_slice();
        for (index, uvalue) in value.iter().enumerate(){
           target[ptr + index] = *uvalue;   
        }
       
    }
    // 加载代码段
    pub fn load_op_code(&mut self, ptr: usize, value: Vec<u8>){
        self.op = value;       
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
