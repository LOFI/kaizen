CREATE TABLE "user" (
  id SERIAL PRIMARY KEY,
  email VARCHAR(254) NOT NULL,
  display_name VARCHAR(20) NOT NULL,
  username VARCHAR(15) NOT NULL,
  -- starting out all these values will simply be "bcrypt" so I'm sizing
  -- based on that.
  hasher VARCHAR(6) NOT NULL,
  password VARCHAR(60) NOT NULL,
  email_verified BOOLEAN DEFAULT FALSE NOT NULL,
  staff BOOLEAN DEFAULT FALSE NOT NULL,
  admin BOOLEAN DEFAULT FALSE NOT NULL,
  active BOOLEAN DEFAULT TRUE NOT NULL
);

CREATE UNIQUE INDEX user_email_idx ON "user" ((lower(email)));
CREATE UNIQUE INDEX user_username_idx ON "user" ((lower(username)));
CREATE INDEX user_active_idx ON "user" (active);
