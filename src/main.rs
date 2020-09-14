extern crate redis;

fn main() {
    do_something()
}

fn do_something() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    /* do something here */
    Ok(())
}

struct Employee {
    id: i16,
    first_name: String,
    last_name: String,
    occupation: String,
    salary: i16
}

fn build_employee(id: i16) -> Employee {

    let first = generate_name();
    let last = generate_name();

    Employee {
        id: id,
        first_name: first,
        last_name: last,
        occupation: generate_occupation(),
        salary: generate_salary()
    }
}

fn get_email(fist_name: String, last_name: String) -> String {

    let mut email: String = String::from(first.as_str());
    email.push_str(".");
    email.push_str(last.as_str());
    email.push_str("@company.com");
    return email;
}

fn generate_name() -> String {
    return String::from("Mike");
}

fn generate_occupation() -> String {
    return String::from("Job");
}

fn generate_salary() -> i16 {
    return 100_000;
}
