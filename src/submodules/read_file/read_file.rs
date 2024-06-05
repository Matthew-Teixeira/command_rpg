// use std::io::{ BufRead };
// use std::fs;
// use std::io;
// use std::fs::File;

fn read_file(file_path: &str) -> io::Result<()> {
    println!("{:?}", fs::metadata(file_path));
    match fs::metadata(file_path) {
        Ok(metadata) => {
            if metadata.is_file() {
                let file_contents = fs::read_to_string(file_path)?;
                println!("FILE DATA:\n {}", file_contents);
            } else {
                println!("Path exists, but it's not a file.");
            }
        }
        Err(_) => {
            println!("File does not exist.");
        }
    }
    Ok(())
}

// To read a file line by line using a buffer, you can use the BufReader struct and the BufRead trait:

fn read_file_by_line(file_path: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut lines_vec = Vec::new();

    for line in reader.lines() {
        lines_vec.push(line?);
    }
    Ok(lines_vec)
}

/* 
match read_file_by_line("test_file.txt") {
        Ok(lines) => {
            for (index, line) in lines.iter().enumerate() {
                println!("Line #{}:", index + 1);
                println!("{}\n", line);
            }
        }
        Err(e) => println!("Error reading file: {}", e),
    }
*/