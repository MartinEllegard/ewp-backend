from fastapi import FastAPI
import os
from fastapi.middleware.cors import CORSMiddleware
from api import router as api_router
from dotenv import load_dotenv
from repository import Repository

load_dotenv()

app = FastAPI()

origins = [
    "http://localhost",
    "http://localhost:8080",
]

app.add_middleware(
    CORSMiddleware,
    allow_origins=origins,
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

app.include_router(api_router, prefix="/api")


async def get_repository():
    return app.state.repository


@app.on_event("startup")
async def startup():
    db_url = os.environ["DATABASE_CONNECTION"]
    repository = Repository(db_url)
    app.state.repository = repository


@app.on_event("shutdown")
async def shutdown():
    await app.state.repository.disconnect()
