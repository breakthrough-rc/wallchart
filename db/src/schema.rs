// @generated automatically by Diesel CLI.

diesel::table! {
    activities (id) {
        id -> Text,
        name -> Text,
        icon -> Text,
    }
}

diesel::table! {
    assessments (id) {
        id -> Text,
        value -> Int4,
        worker_id -> Text,
    }
}

diesel::table! {
    locations (id) {
        id -> Text,
        name -> Text,
        worksite_id -> Text,
    }
}

diesel::table! {
    shift_assignments (id) {
        id -> Text,
        shift_id -> Text,
        worker_id -> Text,
    }
}

diesel::table! {
    shifts (id) {
        id -> Text,
        name -> Text,
        loction_id -> Text,
    }
}

diesel::table! {
    tags (id) {
        id -> Text,
        activity_id -> Text,
        worker_id -> Text,
    }
}

diesel::table! {
    workers (id) {
        id -> Text,
        first_name -> Text,
        last_name -> Text,
    }
}

diesel::table! {
    worksites (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::joinable!(assessments -> workers (worker_id));
diesel::joinable!(locations -> worksites (worksite_id));
diesel::joinable!(shift_assignments -> shifts (shift_id));
diesel::joinable!(shift_assignments -> workers (worker_id));
diesel::joinable!(shifts -> locations (loction_id));
diesel::joinable!(tags -> activities (activity_id));
diesel::joinable!(tags -> workers (worker_id));

diesel::allow_tables_to_appear_in_same_query!(
    activities,
    assessments,
    locations,
    shift_assignments,
    shifts,
    tags,
    workers,
    worksites,
);
