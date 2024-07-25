from datetime import datetime, timedelta, timezone
import json

from fastapi import HTTPException, status
from pydantic import BaseModel
from typeguard import typechecked
import jwt

from my_schema import Profile


SERVER_SECRET_KEY = "secret"
ALGORITHM = "HS256"


class TokenData(BaseModel):
    exp: int  # Expiration time
    profile: Profile


@typechecked
def create_token(duration: timedelta, profile: Profile) -> str:
    expire = int((datetime.now(timezone.utc) + duration).timestamp())
    to_encode = TokenData(exp=expire, profile=profile).model_dump_json()
    encoded_dict = json.loads(to_encode)
    return jwt.encode(encoded_dict, SERVER_SECRET_KEY, algorithm=ALGORITHM)


@typechecked
def read_token_data(token: str) -> TokenData:
    try:
        decoded = jwt.decode(token, SERVER_SECRET_KEY, algorithms=[ALGORITHM])
    except jwt.ExpiredSignatureError:
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Refresh token has expired.",
        )
    except jwt.InvalidTokenError:
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED, detail="Invalid token."
        )

    return TokenData(**decoded)
