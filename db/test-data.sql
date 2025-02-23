INSERT INTO events (id, user_id, type, data)
VALUES 
  (1, 0, 'Added', '{"parent": null, "text": "make lunch", "id": "8722655e-f231-11ef-8932-1f1e2ee24d96" }'),
  (2, 0, 'Edited', '{"text": "make pasta for lunch", "id": "8722655e-f231-11ef-8932-1f1e2ee24d96" }'),
  (3, 0, 'Added', '{"text": "cook pasta", "parent": "8722655e-f231-11ef-8932-1f1e2ee24d96", "id": "1302f702-f23a-11ef-a65c-63c495723c09" }'),
  (4, 0, 'Added', '{"text": "add pesto", "parent": "8722655e-f231-11ef-8932-1f1e2ee24d96", "id": "3ae41864-f23a-11ef-885e-5763caa334a4" }'),
  (5, 0, 'MarkedAsDone', '{"id": "8722655e-f231-11ef-8932-1f1e2ee24d96" }');

INSERT INTO states (user_id, after_event_id, state)
VALUES
    (0, 1, '{ "children": [{ "id": "8722655e-f231-11ef-8932-1f1e2ee24d96", "text": "make lunch" }] }'),
    (0, 2, '{ "children": [{ "id": "8722655e-f231-11ef-8932-1f1e2ee24d96", "text": "make pasta for lunch" }] }'),
    (0, 5, '{ "children": [] }');