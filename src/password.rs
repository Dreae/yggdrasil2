extern crate crypto;
extern crate rand;

use self::rand::Rng;
use self::rand::os::OsRng;

use self::crypto::bcrypt_pbkdf;
use rustc_serialize::base64;
use rustc_serialize::base64::{ToBase64, FromBase64};

pub fn hash_password(password: String) -> String {
    let mut rng = OsRng::new().unwrap();
    let mut salt = [0u8; 24];
    rng.fill_bytes(&mut salt);

    salt.to_base64(base64::STANDARD) + &"$".to_owned() + &hash_password_ex(password, &salt)
}

pub fn verify_password(password: String, hash: String) -> bool {
    let parts: Vec<&str> = hash.split('$').collect();
    let salt = Box::into_raw(parts[0].from_base64().unwrap().into_boxed_slice());
    let new_hash = unsafe { hash_password_ex(password, &*salt) };
    new_hash == parts[1]
}

fn hash_password_ex(password: String, salt: &[u8]) -> String {
    let mut out = [0u8; 256];
    let pass_u8 = Box::into_raw(password.into_bytes().into_boxed_slice());
    unsafe {
        bcrypt_pbkdf::bcrypt_pbkdf(&*pass_u8, salt, 6, &mut out);
    }
    out.to_base64(base64::STANDARD)
}

#[cfg(test)]
mod tests {
    use password;

    #[test]
    fn test_verify() {
        assert!(password::verify_password("password".to_owned(), password::hash_password("password".to_owned())));
    }
}
