use crate::model::sector::*;

use super::{Error, FindError};

pub struct CountriesService {}

impl CountriesService {
    pub fn new() -> Self {
        CountriesService {}
    }

    pub async fn list_countries(
        skip: i64,
        amount: i64,
        sort: &CountriesSortCriterias,
        sort_asc: bool,
    ) -> Result<Vec<&Country>, Error> {
        todo!()
    }

    pub async fn search_countries(
        query: &String,
        skip: i64,
        amount: i64,
        sort: &CountriesSortCriterias,
        sort_asc: bool,
    ) -> Result<Vec<Country>, Error> {
        todo!()
    }

    pub async fn find_country(id: i64) -> Result<Country, FindError> {
        todo!()
    }

    pub async fn find_neighbors(id: i64) -> Result<Vec<Neighbor>, FindError> {
        todo!()
    }
}

pub struct Neighbor {
    pub neighbor_id: i64,
    pub capital_cities_distance: i64,
    pub capital_cities_travel_time_d: i64
}