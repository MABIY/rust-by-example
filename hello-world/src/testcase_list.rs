use std::fmt;
use std::fmt::Formatter;
use std::ptr::write;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ",")?;
            }
            write!(f, "{index}: {}", v,index=count)?;
        }

        write!(f, "]")
    }
}

fn main() {
    let  v = List(vec![1,2,3]);
    println!("{}",v);
}