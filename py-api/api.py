from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker
import db

app = FastAPI()

engine = create_engine('postgresql://localhost/mydatabase')
Session = sessionmaker(bind=engine)
session = Session()

class UserIn(BaseModel):
    name: str
    email: str

class UserOut(BaseModel):
    id: int
    name: str
    email: str

# Get all users
@app.get("/users")
def get_users():
    users = session.query(db.User).all()
    return [user.as_dict() for user in users]

# Get a specific user by id
@app.get("/users/{user_id}")
def get_user(user_id: int):
    user = session.query(db.User).get(user_id)
    if user:
        return user.as_dict()
    else:
        raise HTTPException(status_code=404, detail="User not found")

# Create a new user
@app.post("/users")
def create_user(user: UserIn):
    new_user = db.User(name=user.name, email=user.email)
    session.add(new_user)
    session.commit()
    return UserOut(**new_user.as_dict()), 201

# Update a specific user
@app.put("/users/{user_id}")
def update_user(user_id: int, user: UserIn):
    db_user = session.query(db.User).get(user_id)
    if db_user:
        db_user.name = user.name
        db_user.email = user.email
        session.commit()
        return UserOut(**db_user.as_dict())
    else:
        raise HTTPException(status_code=404, detail="User not found")

# Delete a specific user
@app.delete("/users/{user_id}")
def delete_user(user_id: int):
    user = session.query(db.User).get(user_id)
    if user:
        session.delete(user)
        session.commit()
        return {"message": "User deleted"}
    else:
        raise HTTPException(status_code=404, detail="User not found")