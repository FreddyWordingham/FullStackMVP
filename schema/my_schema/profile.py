from datetime import datetime

from pydantic import BaseModel


class Profile(BaseModel):
    creation_timestamp: datetime
    username: str
