use lambda_runtime::{lambda, Context, error::HandlerError};
use log::{info, error};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Debug)
        .expect("simple_logger::init_with_level() panicked");
    lambda!(verification_handler);
    Ok(())
}

fn verification_handler(object: String, ctx: Context) -> Result<bool, HandlerError> {
    use zero_orb::{interface::{BackPack, MarkZero}, CommonReference, FrLocal, G1Local, G2Local, GtLocal};

    match serde_json::from_str::<BackPack<CommonReference<FrLocal, G1Local, G2Local>, FrLocal, G1Local, G2Local, GtLocal>>(&object) 
    {
        Ok(orb) => {
            match orb.verify() {
                true => {
                    info!("True [PLACEHOLDER PUBLIC_KEY]");
                    Ok(true)
                },
                false => {
                    info!("False [PLACEHOLDER PUBLIC_KEY]");
                    Ok(false)
                },
            }
        },
        Err(_) => {
            error!("orb_verification::verification_handler::from_str() panicked when deserializing the object");
            Err(ctx.new_error("verification_handler::from_str() panicked when deserializing the object"))
        },
    }
}
