from asyncio import Future
import json
import websockets

class Request:
    def __init__(self, payload_object):
        self.payload = json.dumps(payload_object)
        self.response = Future()

class Client:
    def __init__(self, ws):
        self.ws = ws
        self.queue = []
        self.request_in_progress = False

    @classmethod
    async def new(cls):
        ws = await websockets.connect("ws://localhost:6583")
        return Client(ws)
    
    async def _handle_next_request(self):
        self.request_in_progress = True
        next_request = self.queue.pop(0)
        await self.ws.send(next_request.payload)
        while True:
            response = await self.ws.recv()
            response = json.loads(response)
            if response['final'] == True:
                next_request.response.set_result(response['payload'])
                break
        if len(self.queue) > 0:
            self._handle_next_request()
        self.request_in_progress = False

    async def _send(self, payload_object):
        r = Request(payload_object)
        self.queue.append(r)
        if not self.request_in_progress:
            await self._handle_next_request()
        return await r.response

