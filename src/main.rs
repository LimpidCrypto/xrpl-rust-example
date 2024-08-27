use rand::rngs::ThreadRng;
use tokio::net::TcpStream;
use xrpl::{
    asynch::clients::{AsyncWebsocketClient, SingleExecutorMutex, WebsocketOpen, XRPLWebsocketIO},
    models::requests::Fee,
};

#[tokio::main]
async fn main() {
    let url = "wss://s.altnet.rippletest.net:51233";
    let rng = rand::thread_rng();
    let stream = TcpStream::connect(url).await.unwrap();
    let mut client: AsyncWebsocketClient<
        4096,
        TcpStream,
        _,
        _,
        ThreadRng,
        SingleExecutorMutex,
        WebsocketOpen,
    > = AsyncWebsocketClient::open(rng, stream, url.parse().unwrap())
        .await
        .unwrap();
    let req = Fee::new(None);
    client.xrpl_send(req.into()).await.unwrap();

    while let Ok(Some(res)) = client.xrpl_receive().await {
        println!("{:?}", res);
    }
}
