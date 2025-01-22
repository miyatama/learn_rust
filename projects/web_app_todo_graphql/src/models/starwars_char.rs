use super::episode::Episode;

pub struct StarWarsChar {
    id: &'static str,
    name: &'static str,
    is_human: bool,
    friends: Vec<usize>,
    appears_in: Vec<Episode>,
    home_planet: Option<&'static str>,
    primary_function: Option<&'static str>,
}
