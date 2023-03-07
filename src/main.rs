use sqlite;
use sqlite::State;

fn main() {
    let _connection = sqlite::open("test.db").unwrap();
    // _connection.execute("CREATE TABLE account_1 (name TEXT, date TEXT, value FLOAT);").unwrap();
    _connection.execute("INSERT INTO account_1 VALUES ('Courses Carrefour', '11/01/2001', -54.50)").unwrap();
    let query = "SELECT * FROM account_1";
    let mut statement = _connection.prepare(query).unwrap();
    //statement.bind((1, 0)).unwrap();

    while let Ok(State::Row) = statement.next() {
        println!("name = {}", statement.read::<String, _>("name").unwrap());
        println!("date = {}", statement.read::<String, _>("date").unwrap());
        println!("value = {}", statement.read::<i64, _>("name").unwrap());
    }
}
