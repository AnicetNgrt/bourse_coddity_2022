use async_trait::async_trait;

pub trait PasswordHasher {
    fn verify(&self, password: &String, password_hash: &String) -> bool;
    fn fake_verify(&self);
    fn hash(&self, password: &String) -> String;
}

#[async_trait]
pub trait EmailChecker {
    async fn is_valid(&self, email: &String) -> bool;
}

pub trait HandleGenerator {
    fn generate(&self, username: &String) -> String;
}

pub trait TokenHandler {
    fn generate(&self, id: i64) -> String;
    fn check(&self, token: &String) -> Result<i64, ()>;
}