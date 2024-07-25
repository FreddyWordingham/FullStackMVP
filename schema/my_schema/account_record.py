from datetime import datetime

from pydantic import BaseModel
from typeguard import typechecked

from .profile import Profile


class AccountRecord(BaseModel):
    creation_timestamp: datetime = datetime.now()
    username: str
    hashed_password: str

    @typechecked
    def generate_profile(self) -> Profile:
        return Profile(
            creation_timestamp=self.creation_timestamp,
            username=self.username,
        )
