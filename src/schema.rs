table! {
        email_queue (_id) {
            _id -> BigInt,
            to_address -> Varchar,
            subject -> Varchar,
            mail_body -> Text,
            from -> Nullable<Varchar>,
            status -> Nullable<Tinyint>,
            _timestamp -> Nullable<Datetime>,
            send_time -> Datetime,
            sent_time -> Nullable<Datetime>,
            send_count -> Integer,
            last_error -> Nullable<Text>,
        }
    }