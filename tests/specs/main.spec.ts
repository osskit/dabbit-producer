import {init, close, get} from '../amqp';

describe('main', () => {
    beforeAll(async () => {
        await init();
    });

    afterAll(async () => {
        await close();
    });

    it('readiness', async () => {
        const {ok} = await fetch('http://localhost:3000/ready');

        expect(ok).toBeTruthy();
    });

    it('produce', async () => {
        const {ok} = await fetch('http://localhost:3000/produce', { 
            method: 'POST', 
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({})}
         );

        expect(ok).toBeTruthy();

        const message = await get();

        expect(message).toMatchSnapshot();
    });
});