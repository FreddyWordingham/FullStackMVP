from typeguard import typechecked

from my_database import Database


DATABASE_PATH = "../assets/data/users.json"
DATABASE = Database(DATABASE_PATH)


@typechecked
def get_database() -> Database:
    return DATABASE
