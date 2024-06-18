# main.py
from fastapi import FastAPI, WebSocket, WebSocketDisconnect
import asyncio

app = FastAPI()

@app.websocket("/ws")
async def websocket_endpoint(websocket: WebSocket):
    await websocket.accept()
    await websocket.send_text("Connected successfully")

    try:
        while True:
            data = await websocket.receive_text()
            if data == "start":
                await websocket.send_text("Initializing...")
                await asyncio.sleep(5)

    finally:
        await websocket.close()

