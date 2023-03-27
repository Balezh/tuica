use sqlite;
use sqlite::Connection;
use sqlite::State;
use text_io::read;

fn print_account(connect: &Connection, 
                 account: String) {
    let query = format!("SELECT * FROM {}", account);
    let mut statement = connect.prepare(query).unwrap();

    println!("| name | date | amount |");
    while let Ok(State::Row) = statement.next() {
        println!("| {name} | {date} | {value} |",
                 name = statement.read::<String, _>("name").unwrap(),
                 date = statement.read::<String, _>("date").unwrap(),
                 value = statement.read::<f64, _>("value").unwrap());
    }
}

/// Create new account in current db
fn create_account(connect: &Connection, 
                  new_account: String) 
{
    let query = format!("CREATE TABLE {} (name STRING, date STRING, otherAccount STRING, amount FLOAT)", new_account);
    connect.execute(query).unwrap();
}

/// Used to add a transaction from an account to another 
fn create_transac(connect: &Connection, 
                  account_1: String,
                  account_2: String,
                  name: String,
                  date: String,
                  amount: f64) 
{
    let query_1 = format!("INSERT INTO {} VALUES ('{}','{}','{}','{}')", account_1, name, date, account_2, amount);
    let query_2 = format!("INSERT INTO {} VALUES ('{}','{}','{}','{}')", account_2, name, date, account_1, -amount);
    connect.execute(query_1).unwrap();
    connect.execute(query_2).unwrap();
} 

/// Return actual value of account
fn get_total(connect: &Connection, account: String) -> f64 
{
    return 0.0;
}

fn db_connect() -> Connection {
    println!("Name of db to connect : ");
    let db: String = read!();
    // _db = String::from("{}.db");
    println!("Trying to connect to {}", db);
    let connection = sqlite::open(db).unwrap();
    println!("Connected sucessfully !");
    return connection;
}

fn main() {
    /* test -- Begin -- */
    let connection = db_connect();
    create_account(&connection, String::from("account_1"));
    create_account(&connection, String::from("account_2"));
    create_transac(&connection, String::from("account_1"),
        String::from("account_2"), 
        String::from("test_1"),
        String::from("11/01/2001"), 51.69);
    /* test -- End -- */
}
