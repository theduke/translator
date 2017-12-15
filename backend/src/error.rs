
error_chain!{

    foreign_links {
        DbPool(::r2d2::Error);
        Db(::diesel::result::Error);
        Json(::serde_json::Error);
    }

    errors {
        UnknownUser { }
        InvalidPassword { }
        InvalidRole { }
        PermissionDenied {}
    }
}
