#[derive(Clone)]
pub struct Todo {
    pub id: u32,
    pub text: String,
}

#[async_graphql::Object]
impl Todo {
    pub async fn id(&self) -> u32 {
        self.id
    }

    pub async fn text(&self) -> String {
        self.text.clone()
    }
}
