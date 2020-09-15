extern crate redis;
extern crate rand;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn main() {
    do_something();

    let e1: Employee = build_employee(1);
    println!("{:?}", e1);
}

fn do_something() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1")?;
    let mut _con = client.get_connection()?;

    Ok(())
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
