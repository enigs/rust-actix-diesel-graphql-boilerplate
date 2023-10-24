----- DROP MESSAGE VIEW CONSTRAINTS -----
ALTER TABLE IF EXISTS message_view DROP CONSTRAINT IF EXISTS fk_message_view_actor;
ALTER TABLE IF EXISTS message_view DROP CONSTRAINT IF EXISTS fk_message_view_message;

----- DROP MESSAGE VIEW TRIGGERS -----
DROP TRIGGER IF EXISTS set_created_at_insert ON message_view;

----- DROP MESSAGE VIEW INDEXES -----
DROP INDEX IF EXISTS idx_message_view_message_id;
DROP INDEX IF EXISTS idx_message_view_actor_id;
DROP INDEX IF EXISTS idx_message_view_created_at;

-----------------------------------
----- DROP MESSAGE VIEW TABLE -----
-----------------------------------
DROP TABLE IF EXISTS message_view;

----- DROP MESSAGE CONSTRAINTS -----
ALTER TABLE IF EXISTS message DROP CONSTRAINT IF EXISTS fk_message_actor;
ALTER TABLE IF EXISTS message DROP CONSTRAINT IF EXISTS fk_message_chat_room;
ALTER TABLE IF EXISTS message DROP CONSTRAINT IF EXISTS fk_message_ticket;
ALTER TABLE IF EXISTS message DROP CONSTRAINT IF EXISTS fk_message_file;

----- DROP MESSAGE TRIGGERS -----
DROP TRIGGER IF EXISTS set_created_at_insert ON message;
DROP TRIGGER IF EXISTS set_updated_at_insert ON message;
DROP TRIGGER IF EXISTS set_updated_at_update ON message;

----- DROP MESSAGE INDEXES -----
DROP INDEX IF EXISTS idx_message_chat_room_id;
DROP INDEX IF EXISTS idx_message_ticket_id;
DROP INDEX IF EXISTS idx_message_actor_id;
DROP INDEX IF EXISTS idx_message_file_id;
DROP INDEX IF EXISTS idx_message_status;
DROP INDEX IF EXISTS idx_message_message_type;
DROP INDEX IF EXISTS idx_message_created_at;
DROP INDEX IF EXISTS idx_message_updated_at;
DROP INDEX IF EXISTS idx_message_removed_at;

------------------------------
----- DROP MESSAGE TABLE -----
------------------------------
DROP TABLE IF EXISTS message;

---- DROP TICKET PIPELINE CONSTRAINTS ----
ALTER TABLE IF EXISTS ticket_pipeline DROP CONSTRAINT IF EXISTS fk_ticket_pipeline_actor;
ALTER TABLE IF EXISTS ticket_pipeline DROP CONSTRAINT IF EXISTS fk_ticket_pipeline_ticket;

---- DROP TICKET PIPELINE TRIGGERS ----
DROP TRIGGER IF EXISTS set_created_at_insert ON ticket_pipeline;
DROP TRIGGER IF EXISTS set_updated_at_insert ON ticket_pipeline;
DROP TRIGGER IF EXISTS set_updated_at_update ON ticket_pipeline;

---- DROP TICKET PIPELINE INDEXES ----
DROP INDEX IF EXISTS idx_ticket_pipeline_ticket_id;
DROP INDEX IF EXISTS idx_ticket_pipeline_actor_id;
DROP INDEX IF EXISTS idx_ticket_pipeline_status;
DROP INDEX IF EXISTS idx_ticket_pipeline_created_at;
DROP INDEX IF EXISTS idx_ticket_pipeline_updated_at;
DROP INDEX IF EXISTS idx_ticket_pipeline_included_at;
DROP INDEX IF EXISTS idx_ticket_pipeline_removed_at;
DROP INDEX IF EXISTS idx_ticket_pipeline_unseen;

-----------------------------------
----- DROP TICKET PIPELINE TABLE -----
-----------------------------------
DROP TABLE IF EXISTS ticket_pipeline;

---- DROP CHAT PIPELINE CONSTRAINTS ----
ALTER TABLE IF EXISTS chat_pipeline DROP CONSTRAINT IF EXISTS fk_chat_pipeline_actor;
ALTER TABLE IF EXISTS chat_pipeline DROP CONSTRAINT IF EXISTS fk_chat_pipeline_chat_room;

---- DROP CHAT PIPELINE TRIGGERS ----
DROP TRIGGER IF EXISTS set_created_at_insert ON chat_pipeline;
DROP TRIGGER IF EXISTS set_updated_at_insert ON chat_pipeline;
DROP TRIGGER IF EXISTS set_updated_at_update ON chat_pipeline;

---- DROP CHAT PIPELINE INDEXES ----
DROP INDEX IF EXISTS idx_chat_pipeline_chat_room_id;
DROP INDEX IF EXISTS idx_chat_pipeline_actor_id;
DROP INDEX IF EXISTS idx_chat_pipeline_status;
DROP INDEX IF EXISTS idx_chat_pipeline_created_at;
DROP INDEX IF EXISTS idx_chat_pipeline_updated_at;
DROP INDEX IF EXISTS idx_chat_pipeline_included_at;
DROP INDEX IF EXISTS idx_chat_pipeline_removed_at;
DROP INDEX IF EXISTS idx_chat_pipeline_unseen;

-----------------------------------
---- DROP CHAT PIPELINE TABLE ----
-----------------------------------
DROP TABLE IF EXISTS chat_pipeline;

---- DROP TICKET CONSTRAINTS ----
ALTER TABLE IF EXISTS ticket DROP CONSTRAINT IF EXISTS fk_ticket_actor;
ALTER TABLE IF EXISTS ticket DROP CONSTRAINT IF EXISTS fk_ticket_company;

---- DROP TICKET TRIGGERS ----
DROP TRIGGER IF EXISTS set_created_at_insert ON ticket;
DROP TRIGGER IF EXISTS set_updated_at_insert ON ticket;
DROP TRIGGER IF EXISTS set_updated_at_update ON ticket;

---- DROP TICKET INDEXES ----
DROP INDEX IF EXISTS idx_ticket_actor_id;
DROP INDEX IF EXISTS idx_ticket_company_id;
DROP INDEX IF EXISTS idx_ticket_product_id;
DROP INDEX IF EXISTS idx_ticket_status;
DROP INDEX IF EXISTS idx_ticket_created_at;
DROP INDEX IF EXISTS idx_ticket_updated_at;

-------------------------------
----- DROP TICKET TABLE -------
-------------------------------
DROP TABLE IF EXISTS ticket;

---- DROP CHAT ROOM CONSTRAINTS ----
ALTER TABLE IF EXISTS chat_room DROP CONSTRAINT IF EXISTS fk_chat_room_actor;
ALTER TABLE IF EXISTS chat_room DROP CONSTRAINT IF EXISTS fk_chat_room_company;

---- DROP CHAT ROOM TRIGGERS ----
DROP TRIGGER IF EXISTS set_created_at_insert ON chat_room;
DROP TRIGGER IF EXISTS set_updated_at_insert ON chat_room;
DROP TRIGGER IF EXISTS set_updated_at_update ON chat_room;

---- DROP CHAT ROOM INDEXES ----
DROP INDEX IF EXISTS idx_chat_room_actor_id;
DROP INDEX IF EXISTS idx_chat_room_company_id;
DROP INDEX IF EXISTS idx_chat_room_product_id;
DROP INDEX IF EXISTS idx_chat_room_status;
DROP INDEX IF EXISTS idx_chat_room_created_at;
DROP INDEX IF EXISTS idx_chat_room_updated_at;

--------------------------------
----- DROP CHAT ROOM TABLE -----
--------------------------------
DROP TABLE IF EXISTS chat_room;

---- DROP SESSION CONSTRAINTS ----
ALTER TABLE IF EXISTS session DROP CONSTRAINT IF EXISTS fk_session_actor;

---- DROP SESSION TRIGGERS ----
DROP TRIGGER IF EXISTS set_created_at_insert ON session;
DROP TRIGGER IF EXISTS set_updated_at_insert ON session;
DROP TRIGGER IF EXISTS set_updated_at_update ON session;

---- DROP SESSION INDEXES ----
DROP INDEX IF EXISTS idx_session_actor_id;
DROP INDEX IF EXISTS idx_session_ip;

-------------------------------
----- DROP SESSION TABLE -----
-------------------------------
DROP TABLE IF EXISTS session;

---- DROP ACTOR CONSTRAINTS ----
ALTER TABLE IF EXISTS actor DROP CONSTRAINT IF EXISTS fk_actor_company;
ALTER TABLE IF EXISTS actor DROP CONSTRAINT IF EXISTS fk_actor_file;
ALTER TABLE IF EXISTS company DROP CONSTRAINT IF EXISTS fk_company_actor;

---- DROP ACTOR TRIGGERS ----
DROP TRIGGER IF EXISTS slugify_actor_name ON actor;
DROP TRIGGER IF EXISTS set_created_at_insert ON actor;
DROP TRIGGER IF EXISTS set_updated_at_insert ON actor;
DROP TRIGGER IF EXISTS set_updated_at_update ON actor;

---- DROP ACTOR INDEXES ----
DROP INDEX IF EXISTS idx_actor_first_name;
DROP INDEX IF EXISTS idx_actor_last_name;
DROP INDEX IF EXISTS idx_actor_email;
DROP INDEX IF EXISTS idx_actor_alternate_email;
DROP INDEX IF EXISTS idx_actor_slug;
DROP INDEX IF EXISTS idx_actor_role;
DROP INDEX IF EXISTS idx_actor_status;
DROP INDEX IF EXISTS idx_actor_street;
DROP INDEX IF EXISTS idx_actor_city;
DROP INDEX IF EXISTS idx_actor_state;
DROP INDEX IF EXISTS idx_actor_zip;
DROP INDEX IF EXISTS idx_actor_country;
DROP INDEX IF EXISTS idx_actor_coordinates;
DROP INDEX IF EXISTS idx_actor_image_id;
DROP INDEX IF EXISTS idx_actor_import_id;
DROP INDEX IF EXISTS idx_actor_created_by_id;
DROP INDEX IF EXISTS idx_actor_created_at;
DROP INDEX IF EXISTS idx_actor_updated_at;

-------------------------------
---- DROP ACTOR TABLE ----
-------------------------------
DROP TABLE IF EXISTS actor;

---- DROP ADDRESS CONSTRAINTS ----
ALTER TABLE IF EXISTS address DROP CONSTRAINT IF EXISTS fk_address_company;

---- DROP ADDRESS TRIGGERS ----
DROP TRIGGER IF EXISTS set_created_at_insert ON address;
DROP TRIGGER IF EXISTS set_updated_at_insert ON address;
DROP TRIGGER IF EXISTS set_updated_at_update ON address;

---- DROP ADDRESS INDEXES ----
DROP INDEX IF EXISTS idx_address_company_id;
DROP INDEX IF EXISTS idx_address_name;
DROP INDEX IF EXISTS idx_address_street;
DROP INDEX IF EXISTS idx_address_city;
DROP INDEX IF EXISTS idx_address_state;
DROP INDEX IF EXISTS idx_address_zip;
DROP INDEX IF EXISTS idx_address_country;
DROP INDEX IF EXISTS idx_address_coordinates;
DROP INDEX IF EXISTS idx_address_is_default;
DROP INDEX IF EXISTS idx_address_created_at;
DROP INDEX IF EXISTS idx_address_updated_at;

-------------------------------
----- DROP ADDRESS TABLE -----
-------------------------------
DROP TABLE IF EXISTS address;

---- DROP COMPANY CONSTRAINTS ----
ALTER TABLE IF EXISTS company DROP CONSTRAINT IF EXISTS fk_company_banner;
ALTER TABLE IF EXISTS company DROP CONSTRAINT IF EXISTS fk_company_logo;

---- DROP COMPANY TRIGGERS ----
DROP TRIGGER IF EXISTS slugify_company_name ON company;
DROP TRIGGER IF EXISTS set_created_at_insert ON company;
DROP TRIGGER IF EXISTS set_updated_at_insert ON company;
DROP TRIGGER IF EXISTS set_updated_at_update ON company;

---- DROP COMPANY INDEXES ----
DROP INDEX IF EXISTS idx_company_name;
DROP INDEX IF EXISTS idx_company_slug;
DROP INDEX IF EXISTS idx_company_role;
DROP INDEX IF EXISTS idx_company_status;
DROP INDEX IF EXISTS idx_company_street;
DROP INDEX IF EXISTS idx_company_city;
DROP INDEX IF EXISTS idx_company_state;
DROP INDEX IF EXISTS idx_company_zip;
DROP INDEX IF EXISTS idx_company_country;
DROP INDEX IF EXISTS idx_company_coordinates;
DROP INDEX IF EXISTS idx_company_banner_id;
DROP INDEX IF EXISTS idx_company_logo_id;
DROP INDEX IF EXISTS idx_company_import_id;
DROP INDEX IF EXISTS idx_company_created_by_id;
DROP INDEX IF EXISTS idx_company_org_admin_id;
DROP INDEX IF EXISTS idx_company_is_admin_featured;
DROP INDEX IF EXISTS idx_company_is_platform_featured;
DROP INDEX IF EXISTS idx_company_created_at;
DROP INDEX IF EXISTS idx_company_updated_at;

-------------------------------
---- DROP COMPANY TABLE ----
-------------------------------
DROP TABLE IF EXISTS company;

---- DROP CATEGORY CONSTRAINTS ----
ALTER TABLE IF EXISTS category DROP CONSTRAINT IF EXISTS fk_category_file;

---- DROP CATEGORY TRIGGERS ----
DROP TRIGGER IF EXISTS slugify_category_name ON category;
DROP TRIGGER IF EXISTS set_created_at_insert ON category;
DROP TRIGGER IF EXISTS set_updated_at_insert ON category;
DROP TRIGGER IF EXISTS set_updated_at_update ON category;

---- DROP CATEGORY INDEXES ----
DROP INDEX IF EXISTS idx_category_name;
DROP INDEX IF EXISTS idx_category_slug;
DROP INDEX IF EXISTS idx_category_status;
DROP INDEX IF EXISTS idx_category_parent_id;
DROP INDEX IF EXISTS idx_category_image_id;
DROP INDEX IF EXISTS idx_category_import_id;
DROP INDEX IF EXISTS idx_category_created_by_id;
DROP INDEX IF EXISTS idx_category_is_featured;
DROP INDEX IF EXISTS idx_category_created_at;
DROP INDEX IF EXISTS idx_category_updated_at;

-------------------------------
---- DROP CATEGORY TABLE ----
-------------------------------
DROP TABLE IF EXISTS category;

----- DROP FILE TRIGGERS -----
DROP TRIGGER IF EXISTS set_created_at_insert ON file;
DROP TRIGGER IF EXISTS set_updated_at_insert ON file;
DROP TRIGGER IF EXISTS set_updated_at_update ON file;
DROP TRIGGER IF EXISTS set_expires_at_insert ON file;

----- DROP FILE INDEXES -----
DROP INDEX IF EXISTS idx_file_filename;
DROP INDEX IF EXISTS idx_file_description;
DROP INDEX IF EXISTS idx_file_status;
DROP INDEX IF EXISTS idx_file_mime_type;
DROP INDEX IF EXISTS idx_file_is_attached;
DROP INDEX IF EXISTS idx_file_expires_at;
DROP INDEX IF EXISTS idx_file_created_by_id;
DROP INDEX IF EXISTS idx_file_message_id;
DROP INDEX IF EXISTS idx_file_created_at;
DROP INDEX IF EXISTS idx_file_updated_at;

-------------------------------
------- DROP FILE TABLE -------
-------------------------------
DROP TABLE IF EXISTS file;

----- DROP IMPORT TRIGGERS -----
DROP TRIGGER IF EXISTS set_created_at_insert ON import;
DROP TRIGGER IF EXISTS set_updated_at_insert ON import;
DROP TRIGGER IF EXISTS set_updated_at_update ON import;

----- DROP IMPORT INDEXES -----
DROP INDEX IF EXISTS idx_import_created_by_id;
DROP INDEX IF EXISTS idx_import_created_at;
DROP INDEX IF EXISTS idx_import_updated_at;
DROP INDEX IF EXISTS idx_import_filename;
DROP INDEX IF EXISTS idx_import_module;
DROP INDEX IF EXISTS idx_import_status;

-------------------------------
------ DROP IMPORT TABLE ------
-------------------------------
DROP TABLE IF EXISTS import;
