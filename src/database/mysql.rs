use sqlx::{mysql::MySqlPool, Error, MySql, Pool};

pub async fn db_connect() -> Result<Pool<MySql>, Error> {
    // let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let database_url = "mysql://root:root@127.0.0.1:3306/bee_api";
    println!("CONNECTION STRING => {}", database_url);

    MySqlPool::connect(&database_url).await
}
