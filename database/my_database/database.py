from typing import Optional
import json
import uuid

from typeguard import typechecked
import bcrypt

from my_schema import AccountRecord, Profile


class Database:
    @typechecked
    def __init__(self, path: str):
        with open(path, "r") as file:
            data = json.loads(file.read())
            self.users = {}
            for id, user in data.items():
                self.users[id] = AccountRecord(**user)

    @typechecked
    def save(self, path: str):
        with open(path, "w") as file:
            output = {}
            for id, user in self.users.items():
                output[id] = user.model_dump_json()
            file.write(json.dumps(output, indent=4))

    @typechecked
    def authenticate_user(self, username: str, password: str) -> Optional[Profile]:
        for account_record in self.users.values():
            if account_record.username == username:
                if bcrypt.checkpw(
                    password.encode("utf-8"),
                    account_record.hashed_password.encode("utf-8"),
                ):
                    return account_record.generate_profile()
        return None

    @typechecked
    def create_user(self, username: str, password: str):

        hashed_password = bcrypt.hashpw(
            password.encode("utf-8"), bcrypt.gensalt()
        ).decode("utf-8")

        id = str(uuid.uuid4())
        account_record = AccountRecord(
            username=username, hashed_password=hashed_password
        )

        self.users[id] = account_record
