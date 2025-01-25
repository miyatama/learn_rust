use crate::models::episode::Episode;

pub struct StarWarsChar {
    pub id: &'static str,
    pub name: &'static str,
    pub is_human: bool,
    pub friends: Vec<usize>,
    pub appears_in: Vec<Episode>,
    pub home_planet: Option<&'static str>,
    pub primary_function: Option<&'static str>,
}
