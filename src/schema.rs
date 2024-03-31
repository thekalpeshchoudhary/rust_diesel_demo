// @generated automatically by Diesel CLI.

diesel::table! {
    car (car_uid) {
        car_uid -> Uuid,
        #[max_length = 100]
        make -> Varchar,
        #[max_length = 100]
        model -> Varchar,
        price -> Numeric,
    }
}

diesel::table! {
    person (person_uid) {
        person_uid -> Uuid,
        #[max_length = 50]
        first_name -> Varchar,
        #[max_length = 50]
        last_name -> Varchar,
        #[max_length = 7]
        gender -> Varchar,
        #[max_length = 100]
        email -> Nullable<Varchar>,
        date_of_birth -> Date,
        #[max_length = 50]
        country_of_birth -> Varchar,
        car_uid -> Nullable<Uuid>,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::joinable!(person -> car (car_uid));

diesel::allow_tables_to_appear_in_same_query!(
    car,
    person,
    posts,
);
