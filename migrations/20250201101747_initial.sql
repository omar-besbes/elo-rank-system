-- Add migration script here
CREATE TABLE players (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "name" TEXT NOT NULL,
    "total_games" BIGINT NOT NULL DEFAULT 0,
    "wins" BIGINT NOT NULL DEFAULT 0,
    "losses" BIGINT NOT NULL DEFAULT 0,
    "rating" DOUBLE PRECISION NOT NULL DEFAULT 1000,
    "best_rating" DOUBLE PRECISION NOT NULL DEFAULT 1000
);

-- Create matches table
CREATE TABLE matches (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "winner_id" UUID NOT NULL REFERENCES players(id),
    "loser_id" UUID NOT NULL REFERENCES players(id),
    "k_factor" BIGINT NOT NULL DEFAULT 32
);
