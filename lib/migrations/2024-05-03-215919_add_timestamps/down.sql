-- This file should undo anything in `up.sql`

ALTER TABLE cards DROP COLUMN "created_at";
ALTER TABLE cards DROP COLUMN "updated_at";