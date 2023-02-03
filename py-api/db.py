from sqlalchemy import create_engine, Column, Integer, String, ForeignKey
from sqlalchemy.orm import sessionmaker
from sqlalchemy.ext.declarative import declarative_base

Base = declarative_base()

class User(Base):
    __tablename__ = 'users'
    id = Column(Integer, primary_key=True)
    name = Column(String)
    email = Column(String)
    company_id = Column(Integer, ForeignKey('companies.id'), nullable=True)

class Company(Base):
    __tablename__ = 'companies'
    id = Column(Integer, primary_key=True)
    name = Column(String)
    website = Column(String)

class Skill(Base):
    __tablename__ = 'skills'
    id = Column(Integer, primary_key=True)
    name = Column(String)
    proficiency = Column(Integer)
    level = Column(String)
    use = Column(String)
    user_id = Column(Integer, ForeignKey('users.id'))

engine = create_engine('postgresql://localhost/mydatabase')
Base.metadata.create_all(engine)

Session = sessionmaker(bind=engine)
session = Session()

# Add a user to the database
new_user = User(name='John Doe', email='jdoe@example.com')
session.add(new_user)
session.commit()

new_company = Company(name='Acme Inc', website='https://acme.com')
session.add(new_company)
session.commit()

new_skill = Skill(name='Python', proficiency=80, level='Intermediate', use='Programming', user_id=1)
session.add(new_skill)
session.commit()

# Query the database for users
users = session.query(User).all()
for user in users:
    print(user.id, user.name, user.email)

# Query the database for companies
companies = session.query(Company).all()
for company in companies:
    print(company.id, company.name, company.website)

# Query the database for skills for a specific user
user_skills = session.query(Skill).filter_by(user_id=1).all()
for skill in user_skills:
    print(skill.id, skill.name, skill.proficiency, skill.level, skill.use)