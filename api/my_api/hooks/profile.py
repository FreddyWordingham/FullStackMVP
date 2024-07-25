from fastapi import Depends
from fastapi.security import OAuth2PasswordBearer
from typeguard import typechecked

from my_schema import Profile

from ..utils.token import read_token_data


OAUTH2_SCHEME = OAuth2PasswordBearer(tokenUrl="token")


@typechecked
def get_profile(token: str = Depends(OAUTH2_SCHEME)) -> Profile:
    profile = read_token_data(token).profile
    return profile
