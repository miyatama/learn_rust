use super::human::Human;
use super::droid::Droid;

#[derive(async_graphql::Interface)]
#[allow(clippy::duplicated_attributes)]
#[graphql(
    field(name = "id", ty = "&str"),
    field(name = "name", ty = "&str"),
    field(name = "friends", ty = "Vec<Character<'ctx>>"),
    field(name = "appears_in", ty = "&[Episode]")
)]
pub enum Character<'a> {
    Human(Human<'a>),
    Droid(Droid<'a>),
}