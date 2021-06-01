#[cfg(target_os = "macos")]
use security_framework::key::SecKey;
#[cfg(target_os = "macos")]
use security_framework::os::macos::keychain::SecKeychain;
#[cfg(target_os = "macos")]
use security_framework::os::macos::passwords::*;

fn main() {
    #[cfg(target_os = "macos")]
    {
        println!("hello world");
        let res = SecKey::random_secure_enclave();
        match res {
            Ok(_) => {
                println!("success!")
            }
            Err(err) => {
                eprintln!("Could not create key: {:?}", err);
            }
        }
    }
}
