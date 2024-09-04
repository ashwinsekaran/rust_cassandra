use cassandra_cpp::*;

#[derive(Debug)]
struct Employee {
    id: i32,
    name: String,
}

#[tokio::main]
async fn main() -> Result<()> {

    let cassandra_host = "localhost"; // localhost for cargo build (OR) cassandra for docker build
    let user = "cassandra";
    let password = "cassandra";

    let data = Employee {
        id: 1,
        name: "john".to_string(),
    };

    let mut cluster = Cluster::default();
    cluster.set_credentials(user, password)?;
    cluster.set_contact_points(cassandra_host)?;
    let session = cluster.connect().await?;

    session.execute("CREATE KEYSPACE IF NOT EXISTS employee WITH replication = {'class':'SimpleStrategy', 'replication_factor' : 1};").await?;
    session.execute("CREATE TABLE IF NOT EXISTS employee.details (id int, name text, PRIMARY KEY (id));").await?; // Not ideal use case in cassandra to create a primary key like this

    save_employee(&session, data).await;

    let saved_details = get_employee(&session, 1).await?;

    println!("Employee details id - {:?}", Some(saved_details));

    Ok(())
}

async fn save_employee(session: &Session, data: Employee) {
    let mut statement = session.statement("INSERT INTO employee.details (id, name) VALUES (?, ?);");
    statement.bind(0, data.id).unwrap();
    statement.bind(1, data.name.as_str()).unwrap();
    statement.execute().await.unwrap();
}


async fn get_employee(session: &Session, id: i32) -> Result<Option<Employee>> {
    let query = "SELECT id, name FROM employee.details WHERE id = ?";
    let mut statement = session.statement(query);

    statement.bind_int32(0, id)?;
    let result = statement.execute().await?;

    match result.first_row() {
        None => Ok(None),
        Some(row) => {
            Ok(Some(Employee{
                id: row.get_by_name("id")?,
                name: row.get_by_name("name")?,
            }))
        }
    }
}