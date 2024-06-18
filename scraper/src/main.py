# main.py
from fastapi import FastAPI, WebSocket
import asyncio

app = FastAPI()

@app.websocket("/ws")
async def websocket_endpoint(websocket: WebSocket):
    await websocket.accept()

    await asyncio.sleep(5)
    await websocket.send_text("Connected")
    await websocket.close()
