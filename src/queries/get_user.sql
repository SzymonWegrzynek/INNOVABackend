SELECT user_id, password, role as "role: _" 
FROM "user"
WHERE email = $1;