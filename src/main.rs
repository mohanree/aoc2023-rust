use std::fs::File;
use std::io::{self, Read};
use std::collections::HashMap;

fn day_1_2 ( contents: String) {

    let mut cont = contents;

    let num_map = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five",'5'),
        ("six",'6'),
        ("seven",'7'),
        ("eight",'8'),
        ("nine",'9'),
    ]);

    let lines: Vec<&str> = cont.split('\n').collect();
    
    for  mut line in lines {
        
        let mut earliest_match = None;
        let mut earliest_index = usize::MAX;
    
        for s in num_map.keys() {
            if let Some(idx) = line.find(s) {
                if idx < earliest_index {
                    earliest_index = idx;
                    earliest_match = Some(s);
                }
            }
        }
        println!("{}", line);
        println!(" -> {}", earliest_match.unwrap());
        println!(" -> {}", earliest_index);
        
        line = line.replacen(earliest_match.unwrap(), num_map[], count)
        
    }
    

    
    //day_1_1(cont);
}

fn day_1_1( contents: String ) {
    println!("File contents:\n{}", contents);

    let lines: Vec<&str> = contents.split('\n').collect();
    
    let mut s: i32 = 0;
    for line in lines {
        let t:String = line.chars().filter(|c| c.is_numeric()).collect();
        
        if  t.len() > 0  {
            let my_last: i32 = t.chars().last().unwrap() as i32 - 0x30;
            let my_first: i32 = t.chars().nth(0).unwrap() as i32 - 0x30;
            println!("{0} {1}", my_first, my_last);

            s += my_first * 10 + my_last;
        }
    }
    println!("{}", s);
}

fn main() -> io::Result<()> {

    let mut file = File::open("/tmp/input2.txt")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    day_1_2(contents);
    
    Ok(())
}


