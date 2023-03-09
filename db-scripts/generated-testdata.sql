INSERT INTO companies (name) VALUES
('Acme Inc.'),
('Globex Corporation'),
('Initech');

INSERT INTO users (firstname, lastname, description, email, company_id) VALUES
('John', 'Doe', 'Software Developer', 'johndoe@example.com', 1),
('Jane', 'Smith', 'Project Manager', 'janesmith@example.com', 1),
('Bob', 'Johnson', 'Data Analyst', 'bobjohnson@example.com', 2),
('Alice', 'Williams', 'Marketing Specialist', 'alicewilliams@example.com', 3);

INSERT INTO oauth_access_tokens (user_id, access_token, expires_at) VALUES
(1, 'abc123', '2023-03-28 00:00:00'),
(2, 'def456', '2023-03-28 00:00:00'),
(3, 'ghi789', '2023-03-28 00:00:00');

INSERT INTO projects (name, description) VALUES
('Project A', 'Development of new software product'),
('Project B', 'Data analysis for marketing campaign'),
('Project C', 'Implementation of new HR system');

INSERT INTO skills (name, skill_use) VALUES
('Java', 'Software Development'),
('Python', 'Data Analysis'),
('Marketing', 'Marketing'),
('SQL', 'Data Analysis'),
('Project Management', 'Project Management'),
('HR', 'HR Management');

INSERT INTO project_skills (project_id, skill_id) VALUES
(1, 1),
(1, 2),
(2, 3),
(2, 4),
(3, 5),
(3, 6);

INSERT INTO project_users (project_id, user_id) VALUES
(1, 1),
(1, 2),
(2, 3),
(3, 4);

INSERT INTO skills_users (skill_id, user_id, proficiency) VALUES
(1, 1, 4),
(1, 2, 5),
(2, 1, 3),
(2, 2, 4),
(2, 3, 5),
(3, 4, 4),
(4, 3, 3),
(4, 4, 5),
(5, 2, 4),
(5, 4, 3),
(6, 1, 5),
(6, 4, 4);

INSERT INTO skills_correlation (skill_primary, skill_secondary, correlation) VALUES
(1, 2, 0.8),
(1, 3, 0.6),
(1, 4, 0.5),
(2, 3, 0.7),
(2, 4, 0.6),
(3, 4, 0.4),
(5, 6, 0.9);