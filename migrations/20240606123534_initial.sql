-- Add migration script here
CREATE TABLE IF NOT EXISTS users  
(
    id BIGSERIAL PRIMARY KEY,
    user_id UUID DEFAULT gen_random_uuid() NOT NULL UNIQUE,
    public_id UUID DEFAULT gen_random_uuid() NOT NULL UNIQUE,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS booked_days  
(
    id BIGSERIAL PRIMARY KEY,
    day DATE NOT NULL UNIQUE,
    booked_by UUID DEFAULT NULL REFERENCES users(public_id)
);
