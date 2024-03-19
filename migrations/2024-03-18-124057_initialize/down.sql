-- This file should undo anything in `up.sql`
DROP FUNCTION IF EXISTS diesel_manage_updated_at(_tbl regclass);
DROP FUNCTION IF EXISTS diesel_set_updated_at();

DROP TABLE IF EXISTS tweets CASCADE;
DROP TABLE IF EXISTS likes CASCADE;
