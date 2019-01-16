pub fn handler(object: String, ctx: lambda_runtime::Context) -> Result<bool, lambda_runtime::error::HandlerError> {
    use zero_orb::{interface::{BackPack, MarkZero, ZeroRef}, CommonReference, FrLocal, G1Local, G2Local, GtLocal};

    match serde_json::from_str::<BackPack<CommonReference<FrLocal, G1Local, G2Local>, FrLocal, G1Local, G2Local, GtLocal>>(
        &object
    ) {
        Ok(orb) => {
            let orb_ref_copy = orb.copy_str();
            match orb.verify() {
                true => {
                    log::info!("True {}", &orb_ref_copy.2);
                    insert_true_orb(
                        &establish_connection(),
                        crate::models::InsertOrb::into_from_tuple(orb_ref_copy),                            
                    );
                    Ok(true)
                },
                false => {
                    log::info!("False {}", orb_ref_copy.2);
                    Ok(false)
                },
            }
        },
        Err(_) => {
            log::error!("orb_verification::verification_handler::from_str() panicked when deserializing the object");
            Err(ctx.new_error("verification_handler::from_str() panicked when deserializing the object"))
        },
    }
}

pub fn establish_connection() -> diesel::pg::PgConnection {
    use diesel::Connection;

    dotenv::dotenv().ok();
    diesel::pg::PgConnection::establish(
        &std::env::var("DATABASE_URL")
            .expect("main::diesel::pg::PgConnection::establish::std::env::var() DATABASE_URL must be set")
    ).expect("main::diesel::pg::PgConnection::estalish() Error Connecting to Database Url")
}

fn insert_true_orb(conn: &diesel::pg::PgConnection, value: crate::models::InsertOrb) -> crate::models::TrueOrb {
    use diesel::RunQueryDsl;

    diesel::insert_into(crate::schema::true_orbs::table)
        .values(&value)
        .get_result(conn)
        .expect("logic::insert_true_orb::diesel::insert panicked whilst inserting ")

}