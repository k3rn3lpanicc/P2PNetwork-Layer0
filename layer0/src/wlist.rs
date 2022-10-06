pub struct Wlist{
    list : Vec<String>,
    size : usize
}
impl Wlist{
    pub fn new(size : usize) -> Wlist{
        Wlist{
            list : Vec::new(),
            size
        }
    }
    pub fn add(&mut self, name : String){
        self.list.push(name);
        if self.list.len() > self.size{
            self.list.remove(0);
        }
    }
    pub fn remove(&mut self, index : usize){
        self.list.remove(index);
    }
    pub fn to_vec(&self) -> Vec<String>{
        self.list.iter().map(|x| x.to_string()).collect()
    }
    pub fn print(&self){
        for name in &self.list{
            println!("{}", name);
        }
    }
}