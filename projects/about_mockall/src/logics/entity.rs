use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct Client {
    id: Uuid,
}

impl Client {
    pub fn new(id: Uuid) -> Self {
        Self { id: id }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}
