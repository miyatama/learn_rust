mod todo2_client;

pub use todo2_client::Todo2Client;

pub trait DomainHandler {
    type Todo2ClientType: Todo2Client;
    fn get_todo2_client(&self) -> &Self::Todo2ClientType;
}
