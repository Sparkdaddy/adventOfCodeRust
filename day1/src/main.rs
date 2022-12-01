// Program goal: Reading the input txt, output the most number of calories an elf is carrying. 
// input description: Each elf has various rations on each line and every elf is separated by a blank line.
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn find_max() -> Result<(), Box<dyn std::error::Error>> {
    let (mut max, mut curr) = (0, 0);
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    max = if max < curr {
                        curr
                    } else {
                        max
                    };
                    curr = 0
                } else {
                    curr = curr + ip.parse::<i32>().unwrap();
                }            
            }
        }
    }
    println! ("{}", max);

    Ok(())
}

fn find_top_three_total() -> Result<(), Box<dyn std::error::Error>> {
    let (mut max, mut max1, mut max2, mut curr) = (0, 0, 0, 0);
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    if max < curr {
                        max2 = max1;
                        max1 = max;
                        max = curr;
                    } else if max1 < curr{
                        max2 = max1;
                        max1 = curr;
                    } else if max2 < curr{
                        max2 = curr
                    }
                    curr = 0
                } else {
                    curr = curr + ip.parse::<i32>().unwrap();
                }            
            }
        }
    }
    println! ("{}", max+max1+max2);

    Ok(())
}

fn main() {
    find_max();
    find_top_three_total();
}

