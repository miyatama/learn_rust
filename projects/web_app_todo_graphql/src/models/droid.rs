pub struct Droid<'a>(&'a StarWarsChar);

/// A mechanical creature in the Star Wars universe.
#[async_graphql::Object]
impl<'a> Droid<'a> {
    /// The id of the droid.
    async fn id(&self) -> &str {
        self.0.id
    }

    /// The name of the droid.
    async fn name(&self) -> &str {
        self.0.name
    }

    /// The friends of the droid, or an empty list if they have none.
    async fn friends<'ctx>(&self, ctx: &async_graphql::Context<'ctx>) -> Vec<super::characetr::Character<'ctx>> {
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
    async fn appears_in(&self) -> &[super::episode::Episode] {
        &self.0.appears_in
    }

    /// The primary function of the droid.
    async fn primary_function(&self) -> &Option<&str> {
        &self.0.primary_function
    }
}