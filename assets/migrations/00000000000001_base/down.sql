-- Drop triggers if they were used (You need to drop triggers on individual tables first)
-- DROP TRIGGER IF EXISTS trigger_name ON table_name;

-- Drop functions
DROP FUNCTION IF EXISTS __gl_updated_at_now() CASCADE;
DROP FUNCTION IF EXISTS __gl_created_at_now() CASCADE;
DROP FUNCTION IF EXISTS __gl_expires_at_30days() CASCADE;
DROP FUNCTION IF EXISTS __gl_slugify_name() CASCADE;
DROP FUNCTION IF EXISTS __gl_slugify_actor() CASCADE;

-- Drop numeric collation
DROP COLLATION IF EXISTS __gl_numeric;