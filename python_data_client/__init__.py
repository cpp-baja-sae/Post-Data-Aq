import websockets

class Client:
    def __init__(self, ws):
        self.ws = ws
        self.counter = 0

    @classmethod
    async def new(cls):
        ws = await websockets.connect("ws://localhost:6583")
        return Client(ws)

    async def _send(self, message):
        message = '{{"id": {}, "payload": {}}}'.format(self.counter, message)
        await self.ws.send(message)
        self.counter += 1
        return await self.ws.recv()
