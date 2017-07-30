
error_chain!{

    foreign_links {
        DbTimeout(::r2d2::GetTimeout);
        Diesel(::diesel::result::Error);
        Json(::serde_json::Error);
    }

    errors {
        UnknownUser { }
        InvalidPassword { }
        InvalidRole { }
        PermissionDenied {}
    }
}