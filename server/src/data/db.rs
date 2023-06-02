use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Ws, Client};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;


// database configuraion
const NAMESPACE: &str = "blazy_short";
const DATABASE: &str = "main";
const TABLE_NAME: &str = "url";
const USERNAME: &str = "root";
const PASSWORD: &str = "root";

#[derive(Debug, Serialize, Deserialize)]
struct Record{
    url: String
}

pub struct DB{
    connection: Surreal<Client>
}

impl DB {

    async fn new() -> Self {

        let connection = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();
        
        connection.signin(Root {
            username: USERNAME,
            password: PASSWORD,
        }).await.unwrap();

        connection.use_ns(NAMESPACE).use_db(DATABASE).await.unwrap();
        Self{ connection }
    }

    async fn set(&self, id: String, url: String) -> Result<(), surrealdb::Error>{
        
        // type annotation needed https://github.com/surrealdb/surrealdb/issues/1626
        let _ : Record = self.connection
            .create((TABLE_NAME, id))
            .content(Record { url }).await?;

        Ok(())
    }

    async fn get(&self, id: String)-> Result<Option<String>, surrealdb::Error>{

        // type annotation needed https://github.com/surrealdb/surrealdb/issues/1626
        let record: Option<Record> = self.connection.select((TABLE_NAME, id)).await?;

        match record {
            Some(v) => Ok(Some(v.url)),
            None => Ok(None)
        }
    }

    async fn exist(&self, id: String) -> Result<bool, surrealdb::Error>{
        match self.get(id).await? {
            Some(_) => Ok(true),           
            None => Ok(false),           
        }
    }
}


#[cfg(test)]
mod tests {

    use super::DB;
    #[tokio::test]
    async fn test_database() {

        let db = DB::new().await;

        db.set("id5".to_string(), "httpsy://google.com/masalan".to_string()).await.unwrap();

        let link = db.get("id5".to_string()).await.unwrap().unwrap();

        assert_eq!(link,"httpsy://google.com/masalan")
    }
}