CREATE EXTENSION IF NOT EXISTS pg_trgm;
CREATE INDEX idx_common_plant_trgm ON common_plant USING GIN (common_english_name gin_trgm_ops, common_danish_name gin_trgm_ops, description gin_trgm_ops);
