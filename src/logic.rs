use zero_orb::{interface::*, *};
use diesel::{Connection, RunQueryDsl, QueryDsl};


pub fn exposed_handler(object: String, ctx: lambda_runtime::Context) -> Result<bool, lambda_runtime::error::HandlerError> {

    match handler(object) {
        Ok(x) => Ok(x),
        Err(e) => Err(ctx.new_error(e)),
    }

}

fn handler(object: String) -> Result<bool, &'static str> {

    match serde_json::from_str::<BackPack<CommonReference<FrLocal, G1Local, G2Local>, FrLocal, G1Local, G2Local, GtLocal>>(
        &object
    ) {
        Ok(orb) => {
            match std::fs::read_to_string("src/tests/sample.crs") {
                Ok(crs) => {
                    match orb.get_crs_str() == crs {
                        true => {
                            let orb_ref_copy = orb.copy_str();
                            match orb.verify() {
                                true => {
                                    log::info!("True {}", &orb_ref_copy.2);
                                    match diesel::insert_into(crate::schema::true_orbs::table)
                                        .values(&crate::models::InsertOrb::into_from_tuple(orb_ref_copy))
                                        .execute(&establish_connection()) 
                                    {
                                        Ok(_) => Ok(true),

                                        Err(e) => {
                                            log::error!("logic::handler() verified the orb as true but failed to update the databse: {}", e);
                                            Err("logic::handler() verified the orb as true but failed to update the databse")
                                        },
                                    }
                                },
                                false => {
                                    log::info!("False {}", orb_ref_copy.2);
                                    Ok(false)
                                },
                            }
                        },
                        false =>  {
                            log::error!("logic::handler() rejected the provided CRS as non-equivalent to the databse");
                            Err("logic::handler() rejected the provided CRS as non-equivalent to the database")
                        },
                    }
                },
                Err(_) => {
                    log::error!("logic::handler::from_str() panicked when reading the crs from file");
                    Err("logic::handler panicked whilst reading the crs from file")
                },
            }
            
        },
        Err(e) => {
            log::error!("logic::handler::from_str() panicked when deserializing the object");
            Err("logic::handler::from_str() panicked when deserializing the object")
        },
    }
}

fn establish_connection() -> diesel::pg::PgConnection {

    dotenv::dotenv().ok();

    diesel::pg::PgConnection::establish(
        &std::env::var("DATABASE_URL")
            .expect("main::diesel::pg::PgConnection::establish::std::env::var() DATABASE_URL must be set")
    ).expect("main::diesel::pg::PgConnection::estalish() Error Connecting to Database Url")
}

#[test]
fn test_true_orb_insert() {
    use amelia_orb::interface::Amelia;
    use diesel::ExpressionMethods;
    use crate::{models, schema};

    let amelia_serialized = serde_json::to_string(
        &Amelia {
            crs: std::fs::read_to_string("src/tests/sample.crs")
                .expect("internal_tests: reading sample.crs to String"),
            sleep: 4,
            calories: 2,
            output: 8,
            key_pair: Amelia::gen_ed25519_key_pairing(),
        }
    ).expect("internal_tests: Serializing &x to String");

    let object = Amelia::go_andromeda(amelia_serialized);
    
    let (_, _, sig, _, _) = serde_json::from_str::<BackPack<CommonReference<FrLocal, G1Local, G2Local>, FrLocal, G1Local, G2Local, GtLocal>>(
        &object
    ).unwrap().copy_str();

    assert!(handler(object).expect("internal_test::Handler() panicked"));

    let expected = &schema::true_orbs::table
        .filter(schema::true_orbs::columns::signature.eq(&sig))
        .load::<models::TrueOrb>(&establish_connection())
        .unwrap(); 
    for e in expected {
        assert_eq!(
            e.signature, 
            sig
        );
    };
}