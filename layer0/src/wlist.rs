#[allow(dead_code)]

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
    #[allow(dead_code)]
    pub fn add(&mut self, name : String){
        self.list.push(name);
        if self.list.len() > self.size{
            self.list.remove(0);
        }
    }
    #[allow(dead_code)]

    pub fn remove(&mut self, index : usize){
        self.list.remove(index);
    }

    pub fn remove_node(&mut self, node:&str){
        let index : i32 = self.search(node);
        self.remove(index as usize);
    }

    pub fn search(&self, node:&str)->i32{
        for (cnt,name) in self.list.iter().enumerate(){
            if (*name) == node.to_string(){
                return cnt as i32;
            }
        }
        return -1;
    }

    #[allow(dead_code)]

    pub fn to_vec(&self) -> Vec<String>{
        self.list.iter().map(|x| x.to_string()).collect()
    }
    #[allow(dead_code)]

    pub fn print(&self){
        for name in &self.list{
            println!("{}", name);
        }
    }
    #[allow(dead_code)]

    pub fn clone(&self) -> Wlist{
        Wlist{
            list : self.list.clone(),
            size : self.size
        }
    }
}