use zxcvbn::zxcvbn;

pub fn check_password_strength(password: &str) -> zxcvbn::Entropy {
    zxcvbn(password, &[]).unwrap()
}