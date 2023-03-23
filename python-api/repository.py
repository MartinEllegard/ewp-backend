from pymongo import MongoClient
from pymongo.collection import Collection
from pymongo.errors import DuplicateKeyError
from uuid import UUID
from typing import List, Optional
import bcrypt

from schemas import User, Profile

DB_NAME = "ewp-db"


class Repository:
    def __init__(self, connection_string: str):
        self.client = MongoClient(connection_string)

    def init_db(self, collection_name: str) -> Collection:
        db = self.client[DB_NAME]
        return db[collection_name]

    async def register_user(self, user: User) -> Optional[str]:
        coll = self.init_db("users")

        existing_user = coll.find_one({"username": user.username})
        if existing_user:
            return "User already exists"

        hashed_password = bcrypt.hashpw(user.password.encode(), bcrypt.gensalt())
        user_dict = user.__dict__
        user_dict["password"] = hashed_password

        coll.insert_one(user_dict)

        return None

    async def authenticate_user(self, username: str, password: str) -> Optional[User]:
        coll = self.init_db("users")
        user_data = coll.find_one({"username": username})

        if user_data and bcrypt.checkpw(password.encode(), user_data["password"]):
            user = User(**user_data)
            return user

        return None

    async def find_user_by_id(self, user_id: str) -> Optional[User]:
        coll = self.init_db("users")
        user_data = coll.find_one({"id": user_id})

        if user_data:
            user = User(**user_data)
            return user

        return None

    async def create_profile(self, profile: Profile) -> Optional[str]:
        coll = self.init_db("profiles")

        try:
            coll.insert_one(profile.__dict__)
        except DuplicateKeyError:
            return "Profile already exists"

        return None

    async def get_all_profiles(self) -> List[Profile]:
        coll = self.init_db("profiles")
        cursor = coll.find()
        profiles = [Profile(**doc) for doc in cursor]

        return profiles

    async def get_profiles_by_id(self, profile_id: UUID) -> Optional[Profile]:
        coll = self.init_db("profiles")
        profile_data = coll.find_one({"id": str(profile_id)})

        if profile_data:
            profile = Profile(**profile_data)
            return profile

        return None

    async def get_profiles_by_skill(self, skill_name: str) -> List[Profile]:
        coll = self.init_db("profiles")
        cursor = coll.find({"skills": {"$elemMatch": {"name": skill_name}}})

        profiles = [Profile(**doc) for doc in cursor]

        return profiles

    async def update_profile(self, profile_id: UUID, update_doc: dict) -> None:
        coll = self.init_db("profiles")
        coll.update_one({"id": str(profile_id)}, {"$set": update_doc})

    async def delete_profile(self, profile_id: UUID) -> None:
        coll = self.init_db("profiles")
        coll.delete_one({"id": str(profile_id)})

    async def check_profile_exist(self, profile: Profile) -> bool:
        coll = self.init_db("profiles")
        existing_profile = coll.find_one({"name": profile.name})

        return existing_profile is not None
