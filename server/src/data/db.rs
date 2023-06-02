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

#[derive(Debug, Serialize, Deserialize)]
struct Counter{
    value: usize 
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

        // insert id if not exist
        let _ : Counter = connection
            .create(("counter", "value"))
            .content( Counter { value:1 })
            .await
            .unwrap_or(Counter { value: 1 });

        Self{ connection }
    }

    async fn set(&self, id: String, url: String) -> Result<(), surrealdb::Error>{
        
        // type annotation needed https://github.com/surrealdb/surrealdb/issues/1626
        let _ : Record = self.connection
            .create((TABLE_NAME, id))
            .content(Record { url }).await?;

        Ok(())
    }

    async fn get(&self, id: String) -> Result<Option<String>, surrealdb::Error>{

        // type annotation needed https://github.com/surrealdb/surrealdb/issues/1626
        let record: Option<Record> = self.connection.select((TABLE_NAME, id)).await?;

        match record {
            Some(v) => Ok(Some(v.url)),
            None => Ok(None)
        }
    }

    async fn set_counter(&self, counter: usize) {

        // type annotation needed https://github.com/surrealdb/surrealdb/issues/1626
        let _: Option<Counter> = self.connection
            .update(("counter", "value"))
            .content(Counter{value:counter})
            .await
            .unwrap();

    }

    async fn load_counter(&self) -> Option<usize> {

        // type annotation needed https://github.com/surrealdb/surrealdb/issues/1626
        let counter: Option<Counter> = self
            .connection
            .select(("counter", "value"))
            .await.unwrap();
        Some(counter.unwrap().value)
    }


}


#[cfg(test)]
mod tests {

    use super::DB;
    #[tokio::test]
    async fn test_database() {

        let db = DB::new().await;

        db.set("id5".to_string(), "https://google.com/masalan".to_string()).await.unwrap();

        let link = db.get("id5".to_string()).await.unwrap().unwrap();

        assert_eq!(link,"https://google.com/masalan");

        db.set_counter(10).await;
        let c= db.load_counter().await.unwrap();

        assert_eq!(c,10);
    }
}