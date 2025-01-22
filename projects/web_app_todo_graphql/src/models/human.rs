pub struct Human<'a>(&'a StarWarsChar);

use super::droid::Droid;

/// A humanoid creature in the Star Wars universe.
#[async_graphql::Object]
impl<'a> Human<'a> {
    /// The id of the human.
    async fn id(&self) -> &str {
        self.0.id
    }

    /// The name of the human.
    async fn name(&self) -> &str {
        self.0.name
    }

    /// The friends of the human, or an empty list if they have none.
    async fn friends<'ctx>(&self, ctx: &async_graphql::Context<'ctx>) -> Vec<Character<'ctx>> {
        let star_wars = ctx.data_unchecked::<StarWars>();
        star_wars
            .friends(self.0)
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

    /// Which movies they appear in.
    async fn appears_in(&self) -> &[Episode] {
        &self.0.appears_in
    }

    /// The home planet of the human, or null if unknown.
    async fn home_planet(&self) -> &Option<&str> {
        &self.0.home_planet
    }
}