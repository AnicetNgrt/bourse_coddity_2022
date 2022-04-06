use crate::model::{dl, user, utilities::*};

use super::FindError;

pub struct AccountsService {
    user_dl: Box<dyn dl::user::Repository>,
    password_hasher: Box<dyn PasswordHasher>,
    email_checker: Box<dyn EmailChecker>,
    handle_generator: Box<dyn HandleGenerator>,
    token_handler: Box<dyn TokenHandler>,
}

impl AccountsService {
    pub async fn authenticate_basic(
        &self,
        login: &String,
        password: &String,
    ) -> Result<AuthSuccessPayload, BasicAuthError> {
        match self.user_dl.find_user_id_by_login(login).await {
            Ok(id) => match self.user_dl.find_user_data(id).await {
                Ok(user::Data { password_hash, .. }) => {
                    match self.password_hasher.verify(password, &password_hash) {
                        true => Ok(AuthSuccessPayload {
                            user_id: id,
                            token: self.token_handler.generate(id),
                        }),
                        false => Err(BasicAuthError::BadPassword),
                    }
                }
                Err(dl::FindError::NotFound) => panic!("Found user's data is then not found."),
                Err(dl::FindError::Failure) => Err(BasicAuthError::DlFailure),
            },
            Err(dl::FindError::NotFound) => Err(BasicAuthError::UserNotFound),
            Err(dl::FindError::Failure) => Err(BasicAuthError::DlFailure),
        }
    }

    pub async fn authenticate_token(
        &self,
        token: &String,
    ) -> Result<AuthSuccessPayload, TokenAuthError> {
        match self.token_handler.check(token) {
            Ok(id) => match self.user_dl.find_user_data(id).await {
                Ok(_) => Ok(AuthSuccessPayload {
                    user_id: id,
                    token: token.clone(), // TODO : Refresh token on every token auth ?
                }),
                Err(dl::FindError::NotFound) => Err(TokenAuthError::UserNotFound),
                Err(dl::FindError::Failure) => Err(TokenAuthError::DlFailure),
            },
            Err(_) => Err(TokenAuthError::BadToken),
        }
    }

    pub async fn create_account(
        &self,
        creation_data: &user::CreateData,
    ) -> Result<i64, UserCreateError> {
        if !self.email_checker.is_valid(&creation_data.email).await {
            return Err(UserCreateError::EmailInvalid);
        }

        if creation_data.username.len() < 3 || creation_data.username.len() > 32 {
            return Err(UserCreateError::UsernameInvalid);
        }

        if creation_data.password.len() < 8 {
            return Err(UserCreateError::PasswordInvalid);
        }

        match self
            .user_dl
            .create_user(&user::Data {
                id: 0,
                username: creation_data.username.clone(),
                email: creation_data.email.clone(),
                password_hash: self.password_hasher.hash(&creation_data.password),
                role: user::Role::Regular,
                handle: self.handle_generator.generate(&creation_data.username),
            })
            .await
        {
            Ok(id) => Ok(id),
            Err(dl::user::CreateError::EmailNotUnique) => Err(UserCreateError::EmailNotUnique),
            Err(dl::user::CreateError::Failure) => Err(UserCreateError::DlFailure),
        }
    }

    pub async fn get_user_personal_and_public_data(
        &self,
        id: i64,
    ) -> Result<(user::PersonalData, user::PublicData), FindError> {
        match self.user_dl.find_user_data(id).await {
            Ok(data) => Ok((
                Self::user_data_to_personal_data(data),
                Self::user_data_to_public_data(data),
            )),
            Err(dl::FindError::Failure) => Err(FindError::DlFailure),
            Err(dl::FindError::NotFound) => Err(FindError::NotFound),
        }
    }

    pub async fn get_user_public_data(&self, id: i64) -> Result<user::PublicData, FindError> {
        match self.user_dl.find_user_data(id).await {
            Ok(data) => Ok(Self::user_data_to_public_data(data)),
            Err(dl::FindError::Failure) => Err(FindError::DlFailure),
            Err(dl::FindError::NotFound) => Err(FindError::NotFound),
        }
    }

    fn user_data_to_public_data(data: &user::Data) -> user::PublicData {
        user::PublicData {
            handle: data.handle.clone(),
            username: data.username.clone(),
            role: data.role.clone(),
        }
    }

    fn user_data_to_personal_data(data: &user::Data) -> user::PersonalData {
        user::PersonalData {
            email: data.email.clone(),
        }
    }
}

pub struct AuthSuccessPayload {
    token: String,
    user_id: i64,
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

pub enum UserCreateError {
    DlFailure,
    EmailNotUnique,
    EmailInvalid,
    UsernameInvalid,
    PasswordInvalid,
}
