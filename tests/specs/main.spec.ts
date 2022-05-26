describe('main', () => {
    it('readiness', async () => {
        const {ok} = await fetch('http://localhost:3000/ready');

        expect(ok).toBeTruthy();
    });
});