from my_database import Database


path = "../assets/data/users.json"
db = Database(path)

db.create_user("admin2", "password")


db.save(path)
