use async_trait::async_trait;

use super::super::sector::*;
use super::FindError;

#[async_trait]
pub trait Repository {
    async fn find_country_by_id(&self, id: i64) -> Result<Country, FindError>;
    async fn find_neighbors_by_country_id(&self, id: i64) -> Result<Vec<Neighbors>, FindError>;
    async fn search_countries(
        &self, 
        query: &String,
        skip: i64,
        amount: i64,
        sort: &CountriesSortCriterias,
        sort_asc: bool,
    ) -> Result<Vec<Countries>, ()>;
}