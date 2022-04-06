pub struct Country {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub capital_city: String,
    pub capital_city_latitude: i64,
    pub capital_city_longitude: i64,
    pub population: i64,
    pub area_sqkm: Option<i64>,
    pub continent: Option<String>,
    pub min_height_m: Option<i64>,
    pub max_height_m: Option<i64>,
    pub avg_height_m: Option<i64>,
}

pub struct Language {
    pub name: String,
    pub percentage_spoken: i32,
    pub country_id: i64,
}

pub struct Religion {
    pub name: String,
    pub percentage_followers: i32,
    pub country_id: i64,
}

pub struct Neighbors {
    pub country1_id: i64,
    pub country2_id: i64,
    pub capital_cities_distance: i64,
    pub capital_cities_travel_time_d: i64
}

pub enum CountriesSortCriterias {
    Name,
    Population,
}