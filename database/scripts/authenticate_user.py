from my_database import Database


path = "../assets/data/users.json"
db = Database(path)

profile = db.authenticate_user("admin2", "password")

print(profile)
