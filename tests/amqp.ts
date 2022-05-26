import { AMQPClient } from '@cloudamqp/amqp-client'

const amqp = new AMQPClient("amqp://localhost");

export const init = async () => {
    await amqp.connect();
};

export const close = async () => {
    await amqp.close();
};

export const get = async () => {
    const ch = await amqp.channel();
    const q = await ch.queue();

    return q.get({noAck: true});
        // const consumer = await q.subscribe({noAck: true}, async (msg) => {
        //   console.log(msg.bodyToString())
        //   await consumer.cancel()
        // })
        // await q.publish("Hello World", {deliveryMode: 2})
        // await consumer.wait()
        // await conn.close()
};