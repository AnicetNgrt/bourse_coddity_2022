use async_trait::async_trait;

#[derive(Clone, Copy)]
pub enum UserRole {
    Regular,
    Editor,
}

pub struct UserData {
    id: i64,
    username: String,
    handle: String,
    email: String,
    role: UserRole,
    password_hash: String,
}

pub struct UserCreateData {
    username: String,
    email: String,
    password: String,
}

pub struct UserPersonalData {
    email: String,
}

pub struct UserPublicData {
    username: String,
    handle: String,
    role: UserRole,
}

pub struct AccountsService {
    user_dl: Box<dyn UserDl>,
    password_hasher: Box<dyn PasswordHasher>,
    email_checker: Box<dyn EmailChecker>,
    handle_generator: Box<dyn HandleGenerator>,
    token_handler: Box<dyn TokenHandler>
}

impl AccountsService {
    pub async fn authenticate_basic(&self, login: &String, password: &String) -> Result<AuthSuccessPayload, BasicAuthError> {
        match self.user_dl.find_user_id_by_login(login).await {
            Ok(id) => match self.user_dl.find_user_data(id).await {
                Ok(UserData { password_hash, .. }) => {
                    match self.password_hasher.verify(password, &password_hash) {
                        true => Ok(AuthSuccessPayload {
                            user_id: id,
                            token: self.token_handler.generate(id)
                        }),
                        false => Err(BasicAuthError::BadPassword),
                    }
                }
                Err(DlFindError::NotFound) => panic!("Found user's data is then not found."),
                Err(DlFindError::Failure) => Err(BasicAuthError::DlFailure),
            },
            Err(DlFindError::NotFound) => Err(BasicAuthError::UserNotFound),
            Err(DlFindError::Failure) => Err(BasicAuthError::DlFailure),
        }
    }

    pub async fn authenticate_token(&self, token: &String) -> Result<AuthSuccessPayload, TokenAuthError> {
        match self.token_handler.check(token) {
            Ok(id) => match self.user_dl.find_user_data(id).await {
                Ok(_) => Ok(AuthSuccessPayload {
                    user_id: id,
                    token: token.clone() // TODO : Refresh token on every token auth ?
                }),
                Err(DlFindError::NotFound) => Err(TokenAuthError::UserNotFound),
                Err(DlFindError::Failure) => Err(TokenAuthError::DlFailure),
            },
            Err(_) => Err(TokenAuthError::BadToken),
        }
    }

    pub async fn create_account(&self, creation_data: &UserCreateData) -> Result<i64, UserCreateError> {
        if !self.email_checker.is_valid(&creation_data.email).await {
            return Err(UserCreateError::EmailInvalid);
        }

        if creation_data.username.len() < 3 || creation_data.username.len() > 32 {
            return Err(UserCreateError::UsernameInvalid)
        }

        if creation_data.password.len() < 8 {
            return Err(UserCreateError::PasswordInvalid)
        }

        match self.user_dl.create_user(&UserData {
            id: 0,
            username: creation_data.username.clone(),
            email: creation_data.email.clone(),
            password_hash: self.password_hasher.hash(&creation_data.password),
            role: UserRole::Regular,
            handle: self.handle_generator.generate(&creation_data.username)
        }).await {
            Ok(id) => Ok(id),
            Err(DlCreateUserError::EmailNotUnique) => Err(UserCreateError::EmailNotUnique),
            Err(DlCreateUserError::Failure) => Err(UserCreateError::DlFailure)
        }
    }

    pub async fn get_user_personal_and_public_data(
        &self,
        id: i64,
    ) -> Result<(UserPersonalData, UserPublicData), FindError> {
        match self.user_dl.find_user_data(id).await {
            Ok(data) => Ok((
                Self::user_data_to_personal_data(data),
                Self::user_data_to_public_data(data),
            )),
            Err(DlFindError::Failure) => Err(FindError::DlFailure),
            Err(DlFindError::NotFound) => Err(FindError::NotFound),
        }
    }

    pub async fn get_user_public_data(&self, id: i64) -> Result<UserPublicData, FindError> {
        match self.user_dl.find_user_data(id).await {
            Ok(data) => Ok(Self::user_data_to_public_data(data)),
            Err(DlFindError::Failure) => Err(FindError::DlFailure),
            Err(DlFindError::NotFound) => Err(FindError::NotFound),
        }
    }

    fn user_data_to_public_data(data: &UserData) -> UserPublicData {
        UserPublicData {
            handle: data.handle.clone(),
            username: data.username.clone(),
            role: data.role.clone(),
        }
    }

    fn user_data_to_personal_data(data: &UserData) -> UserPersonalData {
        UserPersonalData {
            email: data.email.clone(),
        }
    }
}

pub struct AuthSuccessPayload {
    token: String,
    user_id: i64
}

pub enum BasicAuthError {
    DlFailure,
    BadPassword,
    UserNotFound,
}

pub enum TokenAuthError {
    DlFailure,
    BadToken,
    UserNotFound,
}

pub enum FindError {
    DlFailure,
    NotFound,
}

pub enum UserCreateError {
    DlFailure,
    EmailNotUnique,
    EmailInvalid,
    UsernameInvalid,
    PasswordInvalid,
}

#[async_trait]
pub trait UserDl {
    async fn find_user_id_by_login(&self, login: &String) -> Result<i64, DlFindError>;
    async fn find_user_data(&self, user_id: i64) -> Result<&UserData, DlFindError>;
    async fn create_user(&self, user_data: &UserData) -> Result<i64, DlCreateUserError>;
}

pub enum DlFindError {
    Failure,
    NotFound,
}

pub enum DlCreateUserError {
    Failure,
    EmailNotUnique,
}

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