use std::fs;
use std::path::Path;

fn main() {
    let source_dir = "/home/proposer-commitment-network-dev/sidecar/keystores.example";
    let dest_dir = "/home/web3signer-25.2.0";
    let source_keys = source_dir.clone().to_owned() + "/keys";
    let source_secrets = source_dir.clone().to_owned() + "/secrets";
    let dest_keys = dest_dir.clone().to_owned() + "/keystore/keys";
    let dest_secrets = dest_dir.clone().to_owned() + "/keystore/secrets";
    let dest_tls = dest_dir.clone().to_owned() + "/tls";
    let dest_keystore = dest_dir.clone().to_owned() + "/tls/keystore";

    // Ensure destination directories exist
    fs::create_dir_all(&dest_keys).expect("Failed to create keys directory");
    fs::create_dir_all(&dest_secrets).expect("Failed to create secrets directory");
    fs::create_dir_all(&dest_tls).expect("Failed to create tls directory");
    fs::create_dir_all(&dest_keystore).expect("Failed to create tls directory");

    // Move and rename keystore files
    for entry in fs::read_dir(source_keys).expect("Failed to read keys directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        if path.is_dir() {
            let dir_name = path.file_name().unwrap().to_str().unwrap();
            let keystore_file = path.join("voting-keystore.json");
            if keystore_file.exists() {
                let new_path = format!("{}/{}.json", dest_keys, dir_name);
                fs::copy(&keystore_file, &new_path).expect("Failed to move keystore file");
            }
        }
    }

    // Move and rename secret files
    for entry in fs::read_dir(source_secrets).expect("Failed to read secrets directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        if path.is_file() {
            let filename = path.file_name().unwrap().to_str().unwrap();
            let new_path = format!("{}/{}.txt", dest_secrets, filename);
            fs::copy(&path, &new_path).expect("Failed to move secret file");
        }
    }

    // Copying tls
    for entry in fs::read_dir("src/tls").expect("Failed to read tls") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        if path.is_file() {
            let filename = path.file_name().unwrap().to_str().unwrap();
            let new_path = format!("{}/tls/{}", dest_dir, filename);
            fs::copy(&path, &new_path);
        }
    }

    // Copying keystore for proxy settings
    for entry in fs::read_dir("src/keystore").expect("Failed to read keystores") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        if path.is_file() {
            let filename = path.file_name().unwrap().to_str().unwrap();
            let new_path = format!("{}/tls/keystore/{}", dest_dir, filename);
            fs::copy(&path, &new_path);
        }
    }
    
    println!("Files moved successfully!");
}
