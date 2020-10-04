CREATE TABLE geolocs (
  id VARCHAR(36) NOT NULL PRIMARY KEY,
  user_id VARCHAR(36) NOT NULL REFERENCES users,
  latitude double precision NOT NULL, 
  longitude double precision NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW()
)