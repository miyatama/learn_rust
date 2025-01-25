use crate::models::character::Character;
use crate::models::droid::Droid;
use crate::models::episode::Episode;
use crate::models::star_wars::StarWars;
use crate::models::star_wars_char::StarWarsChar;

pub struct Human<'a>(pub &'a StarWarsChar);

#[async_graphql::Object]
impl<'a> Human<'a> {
    pub async fn id(&self) -> &str {
        self.0.id
    }

    pub async fn name(&self) -> &str {
        self.0.name
    }

    pub async fn friends<'ctx>(&self, ctx: &async_graphql::Context<'ctx>) -> Vec<Character<'ctx>> {
        let star_wars = ctx.data_unchecked::<StarWars>();
        star_wars
            .friends(&self.0)
            .into_iter()
            .map(|ch| {
                if ch.is_human {
                    Human(ch).into()
                } else {
                    Droid(ch).into()
                }
            })
            .collect()
    }

    pub async fn appears_in(&self) -> &[Episode] {
        &self.0.appears_in
    }

    pub async fn home_planet(&self) -> &Option<&str> {
        &self.0.home_planet
    }
}
