use std::env;
use std::fs::{ File};
use std::io::{ self,Read, Write};
use std::path::{Path};


fn remove_cpp_comments(code: &str) -> String {
    let mut result = String::new();
    let mut iter = code.chars().peekable();
    let mut in_block_comment = false;

    while let Some(c) = iter.next() {
        if in_block_comment {
            if c == '*' && iter.peek() == Some(&'/') {
                iter.next();                 
                in_block_comment = false;
            }
        } else {
            if c == '/' {
                match iter.peek() {
                    Some(&'/') => {
                        iter.find(|&x| x == '\n');
                    }
                    Some(&'*') => {
                        iter.next();                         
                        in_block_comment = true;
                    }
                    _ => {
                        result.push(c);
                    }
                }
            } else {
                result.push(c);
            }
        }
    }

    result
}

fn remove_python_comments(code: &str) -> String {
    let mut result = String::new();
    let mut iter = code.chars().peekable();
    let mut in_line_comment = false;

    while let Some(c) = iter.next() {
        if in_line_comment {
            if c == '\n' {
                in_line_comment = false;
                result.push(c);
            }
        } else {
            if c == '#' {
                in_line_comment = true;
            } else {
                result.push(c);
            }
        }
    }

    result
}


fn process_file(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);

    if !path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
    }

        let is_overwrite = env::args().nth(1) == Some("-o".to_string());

        let content = read_file(filename)?;

        let updated_content = match path.extension().and_then(|ext| ext.to_str()) {
            
            Some("cpp") => remove_cpp_comments(&content),
            Some("js") => remove_cpp_comments(&content),
            Some("rs") => remove_cpp_comments(&content),
            Some("c") => remove_cpp_comments(&content),
            Some("go") => remove_cpp_comments(&content),
            Some("ts") => remove_cpp_comments(&content),
            Some("lua") => remove_cpp_comments(&content),
            Some("dart") => remove_cpp_comments(&content),
            Some("kt") => remove_cpp_comments(&content),
            Some("nr") => remove_cpp_comments(&content),
            Some("rb") => remove_cpp_comments(&content),
            Some("java") => remove_cpp_comments(&content),
            Some("php") => remove_cpp_comments(&content),
            Some("py") => remove_python_comments(&content),
            _ => content,
         };

    if is_overwrite {
                write_file(filename, &updated_content)?;
        println!("Comments Removed: {}", filename);
    } else {
        let updated_filename = format!("{}_updated.{}", strip_extension(filename), path.extension().unwrap_or_default().to_str().unwrap_or("txt"));
        
        write_file(&updated_filename, &updated_content)?;
        println!("Comments Removed: {}", updated_filename);
    }
    
    Ok(())
}

fn read_file(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn write_file(filename: &str, content: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn strip_extension(filename: &str) -> &str {
    Path::new(filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(filename)
}

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 && (args.len() != 3 || args[1] != "-o") {
        eprintln!("Usage: {} [-o] <filename>", args[0]);
        return;
    }

        if let Err(e) = process_file(&args[args.len() - 1]) {
        eprintln!("Error: {}", e);
    }
}