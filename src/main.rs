extern crate redis;
extern crate rand;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use redis::{Connection, RedisResult};

fn main() {
    let mut con = setup_redis_connection();
    println!("Connected to redis instance");

    let e1: Employee = build_employee(1);
    println!("About to publish {:?} to redis", e1);
    publish(con, e1);
}

fn setup_redis_connection() -> Connection {
    let client = redis::Client::open("redis://127.0.0.1");
    return client.get_connection()?;
}

fn publish(mut con: Connection, employee: Employee) {
    let _set : () = redis::cmd("SET").arg("E:"+employee.id).arg(employee.last_name).query(&mut con)?;
}

fn retrieve(mut con: Connection, key: i32) -> String {
    let _get : () = redis::cmd("GET").arg("E:"+employee.id).query(&mut con)?;
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
    return email;
}

fn generate_name(_random_num: i16) -> String {

    let rand_string: String = thread_rng()
        .sample_iter(Alphanumeric)
        .take(10)
        .collect();

    return rand_string;
}

fn generate_occupation(_random_num: i16) -> String {
    return String::from("Job");
}

fn generate_salary() -> i32 {
    return 100_000;
}
