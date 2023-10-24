----- DROP TRIGGERS -----
DROP TRIGGER IF EXISTS set_created_at_insert ON settings;
DROP TRIGGER IF EXISTS set_updated_at_insert ON settings;
DROP TRIGGER IF EXISTS set_updated_at_update ON settings;

----- DROP INDEXES -----
DROP INDEX IF EXISTS idx_settings_module;

----- DROP TABLE -----
DROP TABLE IF EXISTS settings;