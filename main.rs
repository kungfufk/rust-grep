use std::env;
use std::fs::File;
use std::io::Read;
fn main(){
let a:Vec<String>=env::args().collect();
if a.len() < 3 {
    println!("Usage: mygrep <word> <file>");
    return;
}
let file = File::open(&a[2]);
match file{
    Ok(mut file)=>{
        let mut contents = String::new();
        file.read_to_string(&mut contents);
        let mut s=0;
        for i in contents.lines(){
            s+=1;
            for j in i.split_whitespace(){
            if j==a[1]{
                println!("{}:{}",s,i);
                break;
            }} 
        }
    }
    Err(err)=>println!("Error Happened: {}",err),

}
}







