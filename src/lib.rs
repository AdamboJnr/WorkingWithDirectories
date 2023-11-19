use std::env;
use std::error::Error;
use std::fs;
use std::path;

#[derive(Debug)]
pub struct CreateDir {
    pub argument: String,
    pub dirname: String
}

impl CreateDir {
    pub fn new(args: &[String]) -> Result<CreateDir, &'static str> {
        if args.len() < 3 {
            return Err("Not Enough Arguments");
        }
        
        let argument = args[1].clone();
        let dirname = args[2].clone();

        Ok(CreateDir { argument, dirname })
    }
}
pub fn run(dir: &CreateDir) -> Result<(), Box<dyn Error>>{
    let mut result: Result<(), Box<dyn Error>> = Ok(());
    if  dir.argument == "mkdir" {
        result = create_directory(dir);
    }else if dir.argument == "rmdir" {
        result = remove_directory(dir);
    }
 
    result
 }
 
fn create_directory(dir: &CreateDir) -> Result<(), Box<dyn Error>> {
    let path = path::Path::new(dir.dirname.as_str());
    fs::create_dir_all(path)?;
    Ok(())
}
 
fn remove_directory(dir: &CreateDir) -> Result<(), Box<dyn Error>> { 
    let path = path::Path::new(dir.dirname.as_str());
    fs::remove_dir_all(path)?;
    Ok(()) 
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn make_directory() {
        let args: Vec<String> = env::args().collect();
        let new_dir = CreateDir::new(&args).unwrap();

        run(&new_dir).unwrap();

        let path = path::Path::new(new_dir.dirname.as_str());

        assert!(path.is_dir());
    }

    #[test]
    fn remove_directory() {
        let args: Vec<String> = env::args().collect();
        let new_dir = CreateDir::new(&args).unwrap();

        run(&new_dir).unwrap();

        let path = path::Path::new(new_dir.dirname.as_str());

        assert!(!path.is_dir());
    }
}