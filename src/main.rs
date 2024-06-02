use dotenv::dotenv;
#[allow(unused_imports)]
use sqlx::{
    database, postgres::PgPoolOptions, Connection, FromRow, PgConnection, PgPool, Pool, Postgres,
    Row,
};
use std::env;
use std::error::Error;

#[derive(FromRow, Debug)]
struct Student {
    id: i32,
    fname: String,
    lname: String,
    gender: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("getting the db url from env");
    let pool = sqlx::postgres::PgPool::connect(&database_url).await?;

    let query_create_table =
        "create table if not exists student (id serial primary key, fname varchar(20), lname varchar(20), gender varchar(20))";
    sqlx::query(query_create_table).execute(&pool).await?;

    return Ok(());
}

async fn display(pool: Pool<Postgres>) -> Result<(), Box<dyn Error>> {
    let query = "select * from student";
    let student_datas: Vec<Student> = sqlx::query_as::<_, Student>(query).fetch_all(&pool).await?;
    for student in student_datas {
        println!("name: {}", student.id);
        println!("name: {}", student.fname);
        println!("name: {}", student.lname);
        println!("name: {}", student.gender);
    }
    return Ok(());
}

async fn insert(pool: Pool<Postgres>, student: Student) -> Result<(), Box<dyn Error>> {
    let insert_query = "insert into student (id, fname, lname, gender) values ($1, $2, $3, $4)";
    sqlx::query(insert_query)
        .bind(&student.id)
        .bind(&student.fname)
        .bind(&student.lname)
        .bind(&student.gender)
        .execute(&pool)
        .await?;
    return Ok(());
}
