use axum::{routing::get, Router};
use dotenv::dotenv;
#[allow(unused_imports)]
use sqlx::{
    database, postgres::PgPoolOptions, Connection, FromRow, PgConnection, PgPool, Pool, Postgres,
    Row,
};
use std::env;
use std::error::Error;

#[derive(FromRow, Debug)]
#[allow(unused)]
struct Student {
    id: i32,
    first_name: String,
    last_name: String,
    gender: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let app: Router = Router::new().route("/", get(root));
    let host_port = "127.0.0.1:8080";
    let listener = tokio::net::TcpListener::bind(&host_port).await.unwrap();
    let _ = axum::serve(listener, app).await.unwrap();
    let database_url = env::var("DATABASE_URL").expect("getting the db url from env");
    let pool = sqlx::postgres::PgPool::connect(&database_url).await?;
    let query_create_table =
        "create table if not exists student (id serial primary key, first_name varchar(20), last_name varchar(20), gender varchar(20))";
    sqlx::query(query_create_table).execute(&pool).await?;

    println!("listener running at port {host_port}");

    return Ok(());
}

async fn root() -> &'static str {
    return "hello world";
}

#[allow(unused)]
async fn display(pool: Pool<Postgres>) -> Result<(), Box<dyn Error>> {
    let query = "select * from student";
    let student_datas: Vec<Student> = sqlx::query_as::<_, Student>(query).fetch_all(&pool).await?;
    for student in student_datas {
        println!("name: {}", student.id);
        println!("name: {}", student.first_name);
        println!("name: {}", student.last_name);
        println!("name: {}", student.gender);
    }
    return Ok(());
}

#[allow(unused)]
async fn insert(pool: &Pool<Postgres>, student: Student) -> Result<(), Box<dyn Error>> {
    let insert_query = "insert into student (id, first_name, last_name, gender) values ($1, $2, $3, $4)";
    sqlx::query(insert_query)
        .bind(student.id)
        .bind(student.first_name)
        .bind(student.last_name)
        .bind(student.gender)
        .execute(pool)
        .await?;
    return Ok(());
}

#[allow(unused)]
async fn delete(pool: &Pool<Postgres>, id: i32) -> Result<(), Box<dyn Error>> {
    let delete_query = "delete from student where id={$1}";
    sqlx::query(delete_query).bind(id).execute(pool).await?;
    println!("row deleted where id = {id}");
    return Ok(());
}
