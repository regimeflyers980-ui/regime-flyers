use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

fn main() {
    let password = std::env::args().nth(1).expect("Please provide a password");
    
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("failed to hash password")
        .to_string();

    println!("\n--- Copy this SQL and run it in psql ---\n");
    println!("INSERT INTO users (email, password_hash, role, full_name) VALUES ('admin@regimeflyers.com', '{}', 'ADMIN', 'Regime Admin');\n", hash);
}
