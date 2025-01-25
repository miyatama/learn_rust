use crate::models::character::{query_characters, Character};
use crate::models::droid::Droid;
use crate::models::episode::Episode;
use crate::models::human::Human;
use crate::models::star_wars::StarWars;
pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn hero<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        episode: Option<Episode>,
    ) -> Character<'a> {
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
        #[graphql(desc = "id of human")] id: String,
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
    ) -> async_graphql::Result<async_graphql::connection::Connection<usize, Human<'a>>> {
        let humans = ctx.data_unchecked::<StarWars>().humans().to_vec();
        query_characters(after, before, first, last, &humans, Human).await
    }

    async fn droid<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        #[graphql(desc = "id of droid")] id: String,
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
    ) -> async_graphql::Result<async_graphql::connection::Connection<usize, Droid<'a>>> {
        let droids = ctx.data_unchecked::<StarWars>().droids().to_vec();
        query_characters(after, before, first, last, &droids, Droid).await
    }
}
