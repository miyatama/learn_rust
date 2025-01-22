
pub struct QueryRoot;

use super::episode::Episode;
use super::human::Human;
use super::starwars::StarWars;

#[async_graphql::Object]
impl QueryRoot {
    async fn hero<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        #[graphql(
            desc = "If omitted, returns the hero of the whole saga. If provided, returns the hero of that particular episode."
        )]
        episode: Option<Episode>,
    ) -> super::character::Character<'a> {
        let star_wars = ctx.data_unchecked::<StarWars>();
        match episode {
            Some(episode) => {
                if episode == Episode::Empire {
                    Human(star_wars.chars.get(star_wars.luke).unwrap()).into()
                } else {
                    Droid(star_wars.chars.get(star_wars.artoo).unwrap()).into()
                }
            }
            None => Human(star_wars.chars.get(star_wars.luke).unwrap()).into(),
        }
    }

    async fn human<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        #[graphql(desc = "id of the human")] id: String,
    ) -> Option<Human<'a>> {
        ctx.data_unchecked::<StarWars>().human(&id).map(Human)
    }

    async fn humans<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<async_graphql::connection::Connection<usize, Human<'a>>> {
        let humans = ctx.data_unchecked::<StarWars>().humans().to_vec();
        query_characters(after, before, first, last, &humans, Human).await
    }

    async fn droid<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        #[graphql(desc = "id of the droid")] id: String,
    ) -> Option<Droid<'a>> {
        ctx.data_unchecked::<StarWars>().droid(&id).map(Droid)
    }

    async fn droids<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<async_graphql::connection::Connection<usize, Droid<'a>>> {
        let droids = ctx.data_unchecked::<StarWars>().droids().to_vec();
        query_characters(after, before, first, last, &droids, Droid).await
    }
}