from fastapi import APIRouter, Depends, HTTPException
from typing import List
from uuid import UUID, uuid4
import time
import os
from dotenv import load_dotenv

from auth import Claims
from repository import Repository
from schemas import User, UserNoId, ProfileNoId, Profile

load_dotenv()

repository = Repository(os.environ["DATABASE_CONNECTION"])

router = APIRouter()

@router.post("/register")
async def register_user(new_user_no_id: UserNoId):
    new_user = User(
        id=uuid4(),
        username=new_user_no_id.username,
        password=new_user_no_id.password,
    )
    result = await repository.register_user(new_user)
    if result:
        return {"status": "Created"}
    else:
        raise HTTPException(status_code=500, detail="Error registering user")

@router.post("/auth")
async def authenticate_user(user: UserNoId):
    result = await repository.authenticate_user(user.username, user.password)
    if result:
        secret = os.environ["JWT_SECRET"]
        expiration = int(time.time()) + 3600  # 1 hour
        claims = Claims(sub="{}".format(result.id), exp=expiration)
        try:
            token = claims.encode(secret)
            return {"token": token}
        except Exception as e:
            raise HTTPException(status_code=500, detail="Error encoding token")
    else:
        raise HTTPException(status_code=401, detail="Unauthorized")

@router.get("/profiles", response_model=List[Profile])
async def get_profiles():
    profiles = await repository.get_all_profiles()
    if profiles:
        return profiles
    else:
        raise HTTPException(status_code=500, detail="Error getting profiles")

@router.post("/profiles")
async def post_profile(profile_no_id: ProfileNoId):
    profile = Profile(
        id=uuid4(),
        name=profile_no_id.name,
        email=profile_no_id.email,
        description=profile_no_id.description,
        company=profile_no_id.company,
        age=profile_no_id.age,
        projects=profile_no_id.projects,
        certificates=profile_no_id.certificates,
        experience=profile_no_id.experience,
        education=profile_no_id.education,
        skills=profile_no_id.skills,
    )
    result = await repository.create_profile(profile)
    if result:
        return {"status": "OK"}
    else:
        raise HTTPException(status_code=500, detail="Error creating profile")

@router.get("/profiles/{profile_id}", response_model=Profile)
async def get_profile_by_id(profile_id: UUID):
    profile = await repository.get_profiles_by_id(profile_id)
    if profile:
        return profile
    else:
        raise HTTPException(status_code=500, detail="Error getting profile by ID")
