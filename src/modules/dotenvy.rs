pub fn get_token() -> String{
    return dotenvy::var("TOKEN").expect("cannot find token please check .env file");
}

pub fn get_api_token() -> String{
    return dotenvy::var("API_TOKEN_BITCOIN").expect("cannot find api ninja token please check .env file");
}
