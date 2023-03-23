import jwt
import os
from starlette.requests import Request
from starlette.responses import JSONResponse
from pydantic import BaseModel

class Claims(BaseModel):
    sub: str
    exp: int

    @classmethod
    def new(cls, sub: str, exp: int):
        return cls(sub=sub, exp=exp)

    def encode(self, secret: str) -> str:
        return jwt.encode(self.dict(), secret, algorithm="HS256")

    @classmethod
    def decode(cls, token: str, secret: str):
        try:
            decoded = jwt.decode(token, secret, algorithms=["HS256"])
            return cls(**decoded)
        except jwt.InvalidTokenError:
            return None


class Jwt:
    def __init__(self, token: str):
        self.token = token

    @classmethod
    async def from_request(cls, request: Request):
        auth_header = request.headers.get("Authorization")
        if auth_header and auth_header.startswith("Bearer"):
            token = auth_header[7:].strip()
            secret = os.environ["JWT_SECRET"]
            claims = Claims.decode(token, secret)
            if claims:
                return cls(claims.sub)
        return JSONResponse(content={"detail": "Unauthorized"}, status_code=401)