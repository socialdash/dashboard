table! {
    transactions (id) {
        id -> Integer,
        from_address -> VarChar,
        to_address -> VarChar,
        block -> BigInt,
        value -> Numeric,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    capitalizations (id) {
        id -> Integer,
        time -> Timestamp,
        value -> Double,
    }
}
