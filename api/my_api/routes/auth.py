from datetime import timedelta

from fastapi import APIRouter, Depends, HTTPException, status
from fastapi.security import OAuth2PasswordRequestForm
from pydantic import BaseModel


from my_schema import Profile

from ..hooks import get_database
from ..utils.token import create_token


ACCESS_TOKEN_DURATION = timedelta(minutes=15)
REFRESH_TOKEN_DURATION = timedelta(hours=1)


auth_router = APIRouter()


class AccessTokenResponse(BaseModel):
    token_type: str
    access_token: str
    refresh_token: str
    profile: Profile


@auth_router.post(
    "/token",
    operation_id="generateAccessToken",
    tags=["Authentication"],
    response_model=AccessTokenResponse,
)
async def generate_access_token(
    form_data: OAuth2PasswordRequestForm = Depends(),
    db=Depends(get_database),
) -> AccessTokenResponse:

    profile = db.authenticate_user(form_data.username, form_data.password)
    if profile is None:
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Incorrect username or password",
            headers={"WWW-Authenticate": "Bearer"},
        )

    access_token = create_token(ACCESS_TOKEN_DURATION, profile)
    refresh_token = create_token(REFRESH_TOKEN_DURATION, profile)

    return AccessTokenResponse(
        token_type="bearer",
        access_token=access_token,
        refresh_token=refresh_token,
        profile=profile,
    )
