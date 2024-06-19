from fastapi import FastAPI
from fastapi.responses import StreamingResponse

app = FastAPI()

async def event_generator():
    for i in range(101):
        yield f"data: Porcentagem do processo: {i}%\n\n".encode()
        #await asyncio.sleep(1)

@app.get("/stream/main")
async def stream():
    return StreamingResponse(event_generator(), media_type="text/event-stream")

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)

