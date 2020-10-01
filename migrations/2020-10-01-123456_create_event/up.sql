CREATE TABLE events (
  id VARCHAR(36) NOT NULL PRIMARY KEY,
  family_id VARCHAR(36) NOT NULL REFERENCES families,
  subscription_id VARCHAR(36) NOT NULL REFERENCES subscriptions,
  place_id VARCHAR(36) NOT NULL REFERENCES places,
  user_id VARCHAR(36) NOT NULL REFERENCES users,
  message VARCHAR(150) NOT NULL,
  day VARCHAR(150) NOT NULL,
  created_by VARCHAR(36) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_by VARCHAR(36) NOT NULL,
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
)