fn connect() {
    let conn = Connection::connect(
        &addr,
        ConnectionProperties::default(),
    )
    .await?;

    info!("CONNECTED");

    let channel_a = conn.create_channel().await?;
    let channel_b = conn.create_channel().await?;

    let queue = channel_a
        .queue_declare(
            "hello",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    info!(?queue, "Declared queue");

    let mut consumer = channel_b
        .basic_consume(
            "hello",
            "my_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;
    async_global_executor::spawn(async move {
        info!("will consume");
        while let Some(delivery) = consumer.next().await {
            let delivery = delivery.expect("error in consumer");
            delivery
                .ack(BasicAckOptions::default())
                .await
                .expect("ack");
        }
    }).detach();

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