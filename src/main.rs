use sqlite;
use sqlite::Connection;
use sqlite::State;
use text_io::read;

fn print_account(connect: Connection, account: String) {
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

fn create_account(connect: Connection, new_account: String) {
    let query = format!("CREATE TABLE {} (name STRING, date STRING, 2nd_account STRING, amount FLOAT)", new_account);
    connect.prepare(query).unwrap();
}

// fn create_transac(connect: Connection, )

fn db_connect() -> Connection {
    println!("Name of db to connect : ");
    let _db: String = read!();
    // _db = String::from("{}.db");
    println!("Trying to connect to {}", _db);
    let connection = sqlite::open(_db).unwrap();
    println!("Connected sucessfully !");
    return connection;
}

fn main() {
    let connection = db_connect();
    let account = String::from("account_1");
    print_account(connection, account);
}
