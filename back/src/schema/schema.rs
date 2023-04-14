// @generated automatically by Diesel CLI.

diesel::table! {
    task (taskid) {
        taskid -> Uuid,
        userid -> Nullable<Uuid>,
        templateid -> Nullable<Uuid>,
        content -> Nullable<Varchar>,
        checked -> Bool,
        creationdate -> Timestamp,
        duetime -> Nullable<Timestamp>,
    }
}

diesel::table! {
    template (templateid) {
        templateid -> Uuid,
        userid -> Nullable<Uuid>,
        templatename -> Varchar,
        creationdate -> Timestamp,
        weekdays -> Array<Nullable<Bool>>,
        updatedate -> Nullable<Timestamp>,
    }
}

diesel::table! {
    user (userid) {
        userid -> Uuid,
        username -> Varchar,
        pwd -> Varchar,
        isnotionoauth -> Bool,
        lastlogin -> Nullable<Timestamp>,
        datecreated -> Timestamp,
        token -> Nullable<Varchar>,
    }
}

diesel::joinable!(task -> template (templateid));
diesel::joinable!(task -> user (userid));
diesel::joinable!(template -> user (userid));

diesel::allow_tables_to_appear_in_same_query!(
    task,
    template,
    user,
);
