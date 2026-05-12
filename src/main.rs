use borsh::{BorshDeserialize, BorshSerialize};
use lapin::{Connection, ConnectionProperties, options::*, types::FieldTable};
#[allow(deprecated)]
use tokio_amqp::LapinTokioExt;
use futures::StreamExt;

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

#[tokio::main]
async fn main() {
    let conn = Connection::connect(
        "amqp://guest:guest@localhost:5672",
        #[allow(deprecated)]
        ConnectionProperties::default().with_tokio(),
    ).await.unwrap();

    let channel = conn.create_channel().await.unwrap();

    channel.queue_declare(
        "user_created",
        QueueDeclareOptions {
            durable: true,
            auto_delete: false,
            ..Default::default()
        },
        FieldTable::default(),
    ).await.unwrap();

    let mut consumer = channel.basic_consume(
        "user_created",
        "subscriber",
        BasicConsumeOptions::default(),
        FieldTable::default(),
    ).await.unwrap();

    println!("Waiting for messages...");

    while let Some(delivery) = consumer.next().await {
        let delivery = delivery.unwrap();
        let message = UserCreatedEventMessage::try_from_slice(&delivery.data).unwrap();
        println!("In Fathir's Computer [2406495640]. Message received: {:?}", message);
        delivery.ack(BasicAckOptions::default()).await.unwrap();
    }
}
