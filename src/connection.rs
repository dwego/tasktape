use console_api::instrument::{
    instrument_client::InstrumentClient,
    InstrumentRequest,
    Update,
};
use tonic::{
    transport::Channel,
    Streaming,
};

pub struct Connection {
    client: InstrumentClient<Channel>,
}

impl Connection {
    pub async fn connect(
        address: &str,
    ) -> Result<Self, tonic::transport::Error> {
        let client = InstrumentClient::connect(address.to_owned()).await?;

        Ok(Self { client })
    }

    pub async fn updates(
        &mut self,
    ) -> Result<Streaming<Update>, tonic::Status> {
        let request = tonic::Request::new(InstrumentRequest {});

        let response = self.client.watch_updates(request).await?;

        Ok(response.into_inner())
    }
}