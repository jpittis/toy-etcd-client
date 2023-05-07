use etcd_tonic::etcdserverpb::kv_client::KvClient;
use etcd_tonic::etcdserverpb::{PutRequest, PutResponse, RangeRequest, RangeResponse};
use tonic::codegen::StdError;
use tonic::transport::channel::Channel;
use tonic::{Response, Status};

pub struct Client {
    client: KvClient<Channel>,
}

impl Client {
    pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
    where
        D: TryInto<tonic::transport::Endpoint>,
        D::Error: Into<StdError>,
    {
        let client = KvClient::connect(dst).await?;
        Ok(Self { client })
    }

    pub async fn get(&mut self, key: Vec<u8>) -> Result<Response<RangeResponse>, Status> {
        self.client
            .range(RangeRequest {
                key: key.to_vec(),
                ..RangeRequest::default()
            })
            .await
    }

    pub async fn put(
        &mut self,
        key: Vec<u8>,
        value: Vec<u8>,
    ) -> Result<Response<PutResponse>, Status> {
        self.client
            .put(PutRequest {
                key: key.to_vec(),
                value: value.to_vec(),
                ..PutRequest::default()
            })
            .await
    }
}
