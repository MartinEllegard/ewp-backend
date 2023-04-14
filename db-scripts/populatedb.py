import json
import requests
import random

def unique_full_name(first_names, last_names, used_names):
    while True:
        full_name = f"{random.choice(first_names)} {random.choice(last_names)}"
        if full_name not in used_names:
            used_names.add(full_name)
            return full_name

def generate_random_people(n: int):
    # Define possible values for various fields
    first_names = ["John", "Jane", "Michael", "Emily", "William", "Olivia", "Sophia", "Ella", "Charlotte", "Amelia"]
    last_names = ["Smith", "Johnson", "Williams", "Jones", "Brown", "Davis", "Miller", "Wilson", "Taylor", "Thomas"]
    descriptions = ["Software developer", "Web designer", "Data analyst"]
    companies = ["Company A", "Company B", "Company C"]
    schools = ["School A", "School B", "School C"]
    degrees = ["Bachelor", "Master", "PhD"]
    skill_names = ["C#", "JavaScript", "Java", "Go", "C++", "Ruby", "Rust", "Html", "React"]
    project_names = ["Project A", "Project B", "Project C"]
    certificate_names = ["Certificate A", "Certificate B", "Certificate C"]

    people = []
    used_names = set()

    # Generate 50 random people
    for i in range(n):
        person = {}

        # Randomly select values for various fields
        person["name"] = unique_full_name(first_names, last_names, used_names)
        person["description"] = random.choice(descriptions)
        person["age"] = random.randint(18, 65)
        person["email"] = f"{person['name'].lower().replace(' ', '.')}.{i+1}@example.com"
        person["company"] = random.choice(companies)

        # Generate a unique random set of skills
        skills = []
        num_skills = random.randint(1, 5)
        chosen_skills = random.sample(skill_names, num_skills)
        for skill_name in chosen_skills:
            skill = {}
            skill["name"] = skill_name
            skill["level"] = random.randint(1, 5)
            skill["description"] = f"{skill['name']} is a programming language."
            skills.append(skill)
        person["skills"] = skills

        # Generate a random set of work experience
        experience = []
        for j in range(random.randint(1, 3)):
            exp = {}
            exp["company"] = random.choice(companies)
            exp["position"] = random.choice(descriptions)
            exp["description"] = "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
            experience.append(exp)
        person["experience"] = experience

        # Generate a random set of education
        education = []
        for j in range(random.randint(1, 2)):
            edu = {}
            edu["school"] = random.choice(schools)
            edu["degree"] = random.choice(degrees)
            edu["description"] = "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
            education.append(edu)
        person["education"] = education

        # Generate a random set of projects
        projects = []
        for j in range(random.randint(1, 2)):
            project = {}
            project["name"] = random.choice(project_names)
            project["description"] = "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
            projects.append(project)
        person["projects"] = projects

        # Generate a random set of certificates
        certificates = []
        for j in range(random.randint(0, 2)):
            certificate = {}
            certificate["name"] = random.choice(certificate_names)
            certificate["description"] = "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
            certificates.append(certificate)
        person["certificates"] = certificates

        # Add the person to the list of people
        people.append(person)

    return people

def read_json_file(file_path):
    with open(file_path, 'r') as f:
        data = json.load(f)
    return data

def send_post_request(api_url, data):
    headers = {'Content-Type': 'application/json'}
    response = requests.post(api_url, json=data, headers=headers)
    return response

def main():
    # file_path = 'data.json'
    api_url = 'http://localhost:8080/api/profiles'

    json_data = generate_random_people(50)#read_json_file(file_path)

    for obj in json_data:
        response = send_post_request(api_url, obj)
        print(f"POST request status: {response.status_code} - {response.text}")

if __name__ == '__main__':
    main()
