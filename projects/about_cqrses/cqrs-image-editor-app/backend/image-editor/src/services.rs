use async_trait::async_trait;

pub struct ImageEditServices {
    pub services: Box<dyn ImageEditApi>,
}

impl ImageEditServices {
    pub fn new(services: Box<dyn ImageEditApi>) -> Self {
        Self { services }
    }
}

#[async_trait]
pub trait ImageEditApi: Sync + Send {
    async fn atm_withdrawal(&self, atm_id: &str, amount: f64) -> Result<(), AtmError>;
    async fn validate_check(&self, imagee_id: &str, check: &str) -> Result<(), CheckingError>;
}
pub struct AtmError;
pub struct CheckingError;

pub struct HappyPathImageEditServices;

#[async_trait]
impl ImageEditApi for HappyPathImageEditServices {
    async fn atm_withdrawal(&self, _atm_id: &str, _amount: f64) -> Result<(), AtmError> {
        Ok(())
    }

    async fn validate_check(
        &self,
        _image_id: &str,
        _check_number: &str,
    ) -> Result<(), CheckingError> {
        Ok(())
    }
}
