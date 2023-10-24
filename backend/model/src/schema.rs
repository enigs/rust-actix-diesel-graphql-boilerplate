// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    actor (id) {
        #[max_length = 32]
        id -> Varchar,
        cursor -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        #[max_length = 32]
        created_by_id -> Nullable<Varchar>,
        #[max_length = 32]
        import_id -> Nullable<Varchar>,
        import_index -> Nullable<Int4>,
        #[max_length = 32]
        company_id -> Nullable<Varchar>,
        #[max_length = 32]
        image_id -> Nullable<Varchar>,
        registration_info -> Nullable<Jsonb>,
        account_actions -> Nullable<Jsonb>,
        account_verification -> Nullable<Jsonb>,
        account_reset_password -> Nullable<Jsonb>,
        account_sign_in -> Nullable<Jsonb>,
        wizard -> Nullable<Jsonb>,
        #[max_length = 50]
        account_type -> Nullable<Varchar>,
        #[max_length = 50]
        org_ownership_type -> Nullable<Varchar>,
        #[max_length = 300]
        email -> Nullable<Varchar>,
        #[max_length = 300]
        alternate_email -> Nullable<Varchar>,
        password -> Nullable<Jsonb>,
        #[max_length = 300]
        first_name -> Nullable<Varchar>,
        #[max_length = 300]
        last_name -> Nullable<Varchar>,
        #[max_length = 260]
        slug -> Nullable<Varchar>,
        #[max_length = 50]
        role -> Nullable<Varchar>,
        #[max_length = 50]
        status -> Nullable<Varchar>,
        #[max_length = 500]
        street -> Nullable<Varchar>,
        #[max_length = 500]
        city -> Nullable<Varchar>,
        #[max_length = 50]
        state -> Nullable<Varchar>,
        #[max_length = 15]
        zip -> Nullable<Varchar>,
        #[max_length = 100]
        country -> Nullable<Varchar>,
        coordinates -> Nullable<Geometry>,
        #[max_length = 500]
        website -> Nullable<Varchar>,
        #[max_length = 500]
        facebook -> Nullable<Varchar>,
        #[max_length = 500]
        linkedin -> Nullable<Varchar>,
        landline -> Nullable<Array<Nullable<Text>>>,
        mobile -> Nullable<Array<Nullable<Text>>>,
        is_subscribed_to_newsletter -> Nullable<Bool>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    address (id) {
        #[max_length = 32]
        id -> Varchar,
        cursor -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        #[max_length = 32]
        created_by_id -> Nullable<Varchar>,
        #[max_length = 32]
        import_id -> Nullable<Varchar>,
        import_index -> Nullable<Int4>,
        #[max_length = 32]
        company_id -> Nullable<Varchar>,
        #[max_length = 260]
        name -> Nullable<Varchar>,
        description -> Nullable<Text>,
        #[max_length = 500]
        street -> Nullable<Varchar>,
        #[max_length = 500]
        city -> Nullable<Varchar>,
        #[max_length = 50]
        state -> Nullable<Varchar>,
        #[max_length = 15]
        zip -> Nullable<Varchar>,
        #[max_length = 100]
        country -> Nullable<Varchar>,
        coordinates -> Nullable<Geometry>,
        is_default -> Nullable<Bool>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    category (id) {
        #[max_length = 32]
        id -> Varchar,
        cursor -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        #[max_length = 32]
        created_by_id -> Nullable<Varchar>,
        #[max_length = 32]
        import_id -> Nullable<Varchar>,
        import_index -> Nullable<Int4>,
        #[max_length = 32]
        parent_id -> Nullable<Varchar>,
        #[max_length = 32]
        image_id -> Nullable<Varchar>,
        #[max_length = 260]
        name -> Nullable<Varchar>,
        #[max_length = 260]
        slug -> Nullable<Varchar>,
        #[max_length = 50]
        status -> Nullable<Varchar>,
        is_admin_featured -> Nullable<Bool>,
        is_platform_featured -> Nullable<Bool>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    chat_pipeline (id) {
        #[max_length = 32]
        id -> Varchar,
        cursor -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        included_at -> Nullable<Timestamptz>,
        removed_at -> Nullable<Timestamptz>,
        unseen -> Nullable<Int8>,
        #[max_length = 32]
        chat_room_id -> Nullable<Varchar>,
        #[max_length = 32]
        actor_id -> Nullable<Varchar>,
        #[max_length = 50]
        status -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    chat_room (id) {
        #[max_length = 32]
        id -> Varchar,
        cursor -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        #[max_length = 32]
        actor_id -> Nullable<Varchar>,
        #[max_length = 32]
        company_id -> Nullable<Varchar>,
        #[max_length = 32]
        product_id -> Nullable<Varchar>,
        #[max_length = 50]
        status -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    company (id) {
        #[max_length = 32]
        id -> Varchar,
        cursor -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        #[max_length = 32]
        created_by_id -> Nullable<Varchar>,
        #[max_length = 32]
        import_id -> Nullable<Varchar>,
        import_index -> Nullable<Int4>,
        #[max_length = 32]
        banner_id -> Nullable<Varchar>,
        #[max_length = 32]
        logo_id -> Nullable<Varchar>,
        #[max_length = 32]
        org_admin_id -> Nullable<Varchar>,
        #[max_length = 260]
        name -> Nullable<Varchar>,
        #[max_length = 260]
        slug -> Nullable<Varchar>,
        #[max_length = 50]
        role -> Nullable<Varchar>,
        #[max_length = 50]
        status -> Nullable<Varchar>,
        business_description -> Nullable<Text>,
        #[max_length = 500]
        street -> Nullable<Varchar>,
        #[max_length = 500]
        city -> Nullable<Varchar>,
        #[max_length = 50]
        state -> Nullable<Varchar>,
        #[max_length = 15]
        zip -> Nullable<Varchar>,
        #[max_length = 100]
        country -> Nullable<Varchar>,
        coordinates -> Nullable<Geometry>,
        #[max_length = 500]
        website -> Nullable<Varchar>,
        #[max_length = 500]
        facebook -> Nullable<Varchar>,
        #[max_length = 500]
        linkedin -> Nullable<Varchar>,
        landline -> Nullable<Array<Nullable<Text>>>,
        mobile -> Nullable<Array<Nullable<Text>>>,
        is_admin_featured -> Nullable<Bool>,
        is_platform_featured -> Nullable<Bool>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    file (id) {
        #[max_length = 32]
        id -> Varchar,
        cursor -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        expires_at -> Nullable<Timestamptz>,
        #[max_length = 32]
        created_by_id -> Nullable<Varchar>,
        #[max_length = 32]
        message_id -> Nullable<Varchar>,
        #[max_length = 75]
        filename -> Nullable<Varchar>,
        video_url -> Nullable<Text>,
        #[max_length = 50]
        video_source -> Nullable<Varchar>,
        #[max_length = 260]
        video_thumbnail_s_m -> Nullable<Varchar>,
        #[max_length = 260]
        video_thumbnail_m_d -> Nullable<Varchar>,
        #[max_length = 260]
        video_thumbnail_l_g -> Nullable<Varchar>,
        #[max_length = 260]
        video_thumbnail_x_l -> Nullable<Varchar>,
        #[max_length = 75]
        thumbnail_s_m -> Nullable<Varchar>,
        #[max_length = 75]
        thumbnail_m_d -> Nullable<Varchar>,
        #[max_length = 75]
        thumbnail_l_g -> Nullable<Varchar>,
        #[max_length = 75]
        thumbnail_x_l -> Nullable<Varchar>,
        #[max_length = 75]
        landscape_s_m -> Nullable<Varchar>,
        #[max_length = 75]
        landscape_m_d -> Nullable<Varchar>,
        #[max_length = 75]
        landscape_l_g -> Nullable<Varchar>,
        #[max_length = 75]
        landscape_x_l -> Nullable<Varchar>,
        #[max_length = 75]
        landscape_x_x_l -> Nullable<Varchar>,
        #[max_length = 75]
        landscape_x_x_x_l -> Nullable<Varchar>,
        #[max_length = 50]
        module -> Nullable<Varchar>,
        label -> Nullable<Text>,
        #[max_length = 5]
        extension -> Nullable<Varchar>,
        description -> Nullable<Text>,
        #[max_length = 50]
        status -> Nullable<Varchar>,
        #[max_length = 260]
        mime_type -> Nullable<Varchar>,
        #[max_length = 50]
        file_size -> Nullable<Varchar>,
        #[max_length = 260]
        file_type -> Nullable<Varchar>,
        #[max_length = 50]
        height -> Nullable<Varchar>,
        #[max_length = 50]
        width -> Nullable<Varchar>,
        is_attached -> Nullable<Bool>,
        is_thumbnail -> Nullable<Bool>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    import (id) {
        #[max_length = 32]
        id -> Varchar,
        cursor -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        #[max_length = 32]
        created_by_id -> Nullable<Varchar>,
        #[max_length = 75]
        filename -> Nullable<Varchar>,
        #[max_length = 260]
        label -> Nullable<Varchar>,
        #[max_length = 260]
        sheet_name -> Nullable<Varchar>,
        #[max_length = 50]
        module -> Nullable<Varchar>,
        #[max_length = 50]
        status -> Nullable<Varchar>,
        created_rows -> Nullable<Int8>,
        updated_rows -> Nullable<Int8>,
        deleted_rows -> Nullable<Int8>,
        mapped_fields -> Nullable<Int8>,
        unmapped_fields -> Nullable<Int8>,
        total_cells -> Nullable<Int8>,
        total_non_empty_cells -> Nullable<Int8>,
        total_rows -> Nullable<Int8>,
        total_cols -> Nullable<Int8>,
        current_row -> Nullable<Int8>,
        failed_rows -> Nullable<Int8>,
        ignored_rows -> Nullable<Int8>,
        rejected_rows -> Nullable<Int8>,
        mapping -> Nullable<Jsonb>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    message (id) {
        #[max_length = 32]
        id -> Varchar,
        cursor -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        removed_at -> Nullable<Timestamptz>,
        #[max_length = 32]
        chat_room_id -> Nullable<Varchar>,
        ticket_id -> Nullable<Int8>,
        #[max_length = 32]
        actor_id -> Nullable<Varchar>,
        #[max_length = 32]
        file_id -> Nullable<Varchar>,
        content -> Nullable<Text>,
        #[max_length = 50]
        message_type -> Nullable<Varchar>,
        #[max_length = 50]
        status -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    message_view (id) {
        #[max_length = 32]
        id -> Varchar,
        cursor -> Int8,
        created_at -> Nullable<Timestamptz>,
        #[max_length = 32]
        message_id -> Nullable<Varchar>,
        #[max_length = 32]
        actor_id -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    session (id) {
        #[max_length = 32]
        id -> Varchar,
        cursor -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        expires_at -> Nullable<Timestamptz>,
        #[max_length = 32]
        actor_id -> Nullable<Varchar>,
        #[max_length = 260]
        product_name -> Nullable<Varchar>,
        #[max_length = 260]
        product_major -> Nullable<Varchar>,
        #[max_length = 260]
        product_minor -> Nullable<Varchar>,
        #[max_length = 260]
        product_patch -> Nullable<Varchar>,
        #[max_length = 260]
        os_name -> Nullable<Varchar>,
        #[max_length = 260]
        os_major -> Nullable<Varchar>,
        #[max_length = 260]
        os_minor -> Nullable<Varchar>,
        #[max_length = 260]
        os_patch -> Nullable<Varchar>,
        #[max_length = 260]
        os_patch_minor -> Nullable<Varchar>,
        #[max_length = 260]
        device_name -> Nullable<Varchar>,
        #[max_length = 260]
        device_brand -> Nullable<Varchar>,
        #[max_length = 260]
        device_model -> Nullable<Varchar>,
        #[max_length = 260]
        cpu_architecture -> Nullable<Varchar>,
        #[max_length = 260]
        engine_name -> Nullable<Varchar>,
        #[max_length = 260]
        engine_major -> Nullable<Varchar>,
        #[max_length = 260]
        engine_minor -> Nullable<Varchar>,
        #[max_length = 260]
        engine_patch -> Nullable<Varchar>,
        #[max_length = 260]
        ip -> Nullable<Varchar>,
        data -> Nullable<Jsonb>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    settings (id) {
        #[max_length = 32]
        id -> Varchar,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        #[max_length = 100]
        module -> Nullable<Varchar>,
        content -> Jsonb,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    spatial_ref_sys (srid) {
        srid -> Int4,
        #[max_length = 256]
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        #[max_length = 2048]
        srtext -> Nullable<Varchar>,
        #[max_length = 2048]
        proj4text -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    ticket (id) {
        id -> Int8,
        cursor -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        #[max_length = 32]
        actor_id -> Nullable<Varchar>,
        #[max_length = 32]
        company_id -> Nullable<Varchar>,
        #[max_length = 32]
        product_id -> Nullable<Varchar>,
        #[max_length = 50]
        status -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    ticket_pipeline (id) {
        #[max_length = 32]
        id -> Varchar,
        cursor -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        included_at -> Nullable<Timestamptz>,
        removed_at -> Nullable<Timestamptz>,
        unseen -> Nullable<Int8>,
        ticket_id -> Nullable<Int8>,
        #[max_length = 32]
        actor_id -> Nullable<Varchar>,
        #[max_length = 50]
        status -> Nullable<Varchar>,
    }
}

diesel::joinable!(actor -> file (image_id));
diesel::joinable!(address -> company (company_id));
diesel::joinable!(category -> file (image_id));
diesel::joinable!(chat_pipeline -> actor (actor_id));
diesel::joinable!(chat_pipeline -> chat_room (chat_room_id));
diesel::joinable!(chat_room -> actor (actor_id));
diesel::joinable!(chat_room -> company (company_id));
diesel::joinable!(message -> actor (actor_id));
diesel::joinable!(message -> chat_room (chat_room_id));
diesel::joinable!(message -> file (file_id));
diesel::joinable!(message -> ticket (ticket_id));
diesel::joinable!(message_view -> actor (actor_id));
diesel::joinable!(message_view -> message (message_id));
diesel::joinable!(session -> actor (actor_id));
diesel::joinable!(ticket -> actor (actor_id));
diesel::joinable!(ticket -> company (company_id));
diesel::joinable!(ticket_pipeline -> actor (actor_id));
diesel::joinable!(ticket_pipeline -> ticket (ticket_id));

diesel::allow_tables_to_appear_in_same_query!(
    actor,
    address,
    category,
    chat_pipeline,
    chat_room,
    company,
    file,
    import,
    message,
    message_view,
    session,
    settings,
    spatial_ref_sys,
    ticket,
    ticket_pipeline,
);
