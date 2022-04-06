pub mod accounts;
pub mod countries;
pub mod dependencies;

pub enum Error {
    DlFailure
}

pub enum FindError {
    DlFailure,
    NotFound,
}