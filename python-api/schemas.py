from typing import List
from uuid import UUID
from pydantic import BaseModel
from dataclasses import dataclass


@dataclass
class UserNoId:
    username: str
    password: str


@dataclass
class User:
    id: UUID
    username: str
    password: str


@dataclass
class Skill:
    name: str
    level: int
    description: str


@dataclass
class Experience:
    company: str
    position: str
    description: str


@dataclass
class Education:
    school: str
    degree: str
    description: str


@dataclass
class Project:
    name: str
    description: str


@dataclass
class Certificate:
    name: str
    description: str


@dataclass
class Profile:
    id: UUID
    name: str
    description: str
    age: int
    email: str
    company: str
    skills: List[Skill]
    experience: List[Experience]
    education: List[Education]
    projects: List[Project]
    certificates: List[Certificate]


@dataclass
class ProfileNoId:
    name: str
    description: str
    age: int
    email: str
    company: str
    skills: List[Skill]
    experience: List[Experience]
    education: List[Education]
    projects: List[Project]
    certificates: List[Certificate]
