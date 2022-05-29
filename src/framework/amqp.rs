use lapin::{
    BasicProperties, Connection,
    ConnectionProperties, Error
};

async fn connect(amqp_addr: &str) -> Result<Connection, Error> {
    Connection::connect(
        &amqp_addr,
        ConnectionProperties::default(),
    )
    .await
}

async fn create_channel(connection: Connection) -> Result<(), Error>{
    let channel_a = connection.create_channel().await?;

    let queue = channel_a
        .queue_declare(
            "hello",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    let payload = b"Hello world!";

    loop {
        let confirm = channel_a
            .basic_publish(
                "",
                "hello",
                BasicPublishOptions::default(),
                payload,
                BasicProperties::default(),
            )
            .await?
            .await?;
        assert_eq!(confirm, Confirmation::NotRequested);
    }
}