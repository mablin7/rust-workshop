use std::path::PathBuf;

fn main() {
    let mut path = PathBuf::from("/tmp");
    path.push("rust");
    path.set_extension("txt");

    let other_path = PathBuf::from("/var/log");
    
    let combined = path.join(other_path); // combining paths
    println!("Combined: {:?}", combined);
    
    if let Some(parent) = combined.parent() {
        println!("Parent dir: {:?}", parent);
    }

    println!("File name: {:?}", combined.file_name());
}
