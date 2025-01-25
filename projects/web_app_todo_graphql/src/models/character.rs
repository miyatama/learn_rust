use crate::models::droid::Droid;
use crate::models::episode::Episode;
use crate::models::human::Human;
use crate::models::star_wars_char::StarWarsChar;

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

pub async fn query_characters<'a, F, T>(
    after: Option<String>,
    before: Option<String>,
    first: Option<i32>,
    last: Option<i32>,
    characters: &[&'a StarWarsChar],
    map_to: F,
) -> async_graphql::Result<async_graphql::connection::Connection<usize, T>>
where
    F: Fn(&'a StarWarsChar) -> T,
    T: async_graphql::OutputType,
{
    async_graphql::connection::query(
        after,
        before,
        first,
        last,
        |after, before, first, last| async move {
            let mut start = 0usize;
            let mut end = characters.len();
            if let Some(after) = after {
                if after >= characters.len() {
                    return Ok(async_graphql::connection::Connection::new(false, false));
                }
                start = after + 1;
            }
            if let Some(before) = before {
                if before == 0 {
                    return Ok(async_graphql::connection::Connection::new(false, false));
                }
                end = before;
            }
            let mut slice = &characters[start..end];
            if let Some(first) = first {
                slice = &slice[..first.min(slice.len())];
                end -= first.min(slice.len());
            } else if let Some(last) = last {
                slice = &slice[slice.len() - last.min(slice.len())..];
                start = end - last.min(slice.len());
            }

            let mut connection =
                async_graphql::connection::Connection::new(start > 0, end < characters.len());
            connection
                .edges
                .extend(slice.iter().enumerate().map(|(idx, item)| {
                    async_graphql::connection::Edge::new(start + idx, (map_to)(item))
                }));
            Ok::<_, async_graphql::Error>(connection)
        },
    )
    .await
}
