extern crate redis;
extern crate rand;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use redis::{Connection, RedisResult, Client, Commands};

static ID: i32 = 5;

fn main() {

    let mut con = setup_redis_connection().unwrap();
    let e: Employee = build_employee(ID);
    println!("About to publish {:?} to redis", e);
    publish(&mut con, e);

    let val = retrieve(&mut con, ID);
    println!("Value of {:?} is {}", ID, val.unwrap());
}

fn setup_redis_connection() -> RedisResult<Connection> {
    let client = Client::open("redis://127.0.0.1/")?;
    let con = client.get_connection();
    println!("Connected to redis instance");
    con
}

fn publish(con: &mut Connection, employee: Employee) -> RedisResult<()> {
    let _ : () = con.set(format!("E:{}", employee.id.to_string().as_str()), employee.last_name)?;
    Ok(())
}

fn retrieve(con: &mut Connection, id: i32) -> RedisResult<String> {
    con.get(format!("E:{}", id.to_string().as_str()))
}

#[derive(Debug)]
struct Employee {
    id: i32,
    first_name: String,
    last_name: String,
    occupation: String,
    salary: i32
}

fn build_employee(id: i32) -> Employee {

    let first = generate_name(5);
    let last = generate_name(8);

    Employee {
        id,
        first_name: first,
        last_name: last,
        occupation: generate_occupation(1),
        salary: generate_salary()
    }
}

fn get_email(first_name: String, last_name: String) -> String {

    let mut email: String = String::from(first_name.as_str());
    email.push_str(".");
    email.push_str(last_name.as_str());
    email.push_str("@company.com");
    email
}

fn generate_name(_random_num: i16) -> String {
    thread_rng().sample_iter(Alphanumeric).take(10).collect()
}

fn generate_occupation(_random_num: i16) -> String {
    String::from("Job")
}

fn generate_salary() -> i32 {
    100_000
}
