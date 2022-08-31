extern crate redis;
use redis::{Commands, RedisResult, AsyncCommands};
pub struct Connection{
    pub id : i32,
    pub ip : String,
    pub port : i8,
}
pub fn get_redis_client() -> redis::Client{ 
    redis::Client::open("redis://127.0.0.1").unwrap()//default:EcJr4CvMsvsGTyfPcss6@127.0.0.1").unwrap()
}
pub async fn start_redis_server(){
    let client = get_redis_client();
    let mut con = client.get_tokio_connection().await.unwrap();
    let k : bool = con.del("connections").await.unwrap();
    
}

pub async fn add_connection(ip : &str, port : i8){
    let client = get_redis_client();
    let mut con = client.get_tokio_connection().await.unwrap();
    let id : bool = con.lpush("connections" , format!("{}:{}" , ip , port)).await.unwrap();
}

pub async fn get_connections() -> Vec<Connection>{
    let client = get_redis_client();
    let mut con = client.get_tokio_connection().await.unwrap();
    let mut connections : Vec<Connection> = Vec::new();
    let ids : Vec<String>  = con.lrange("connections", 0, -1).await.unwrap();
    for id in ids{
        let idd : Vec<&str> = id.split(":").collect();
        let connection = Connection{
            id : 0,
            ip : idd[0].to_string(),
            port : idd[1].parse::<i8>().unwrap(),
        };
        connections.push(connection);
    }

    connections
}
pub async fn get_nth_connection(n : i32) -> Connection{
    let client = get_redis_client();
    let mut con = client.get_tokio_connection().await.unwrap();
    let id : String = con.lindex("connections", n.try_into().unwrap()).await.unwrap();
    let idd : Vec<&str> = id.split(":").collect();
    let connection = Connection{
        id : 0,
        ip : idd[0].to_string(),
        port : idd[1].parse::<i8>().unwrap(),
    };
    connection
}

pub fn remove_connection(id : i32){
    let client = get_redis_client();
    let mut con = client.get_connection().unwrap();
    let a : bool = con.lrem("connections", 1, format!("{}", id)).unwrap();
}
pub fn get_connections_len() -> i32{
    let client = get_redis_client();
    let mut con = client.get_connection().unwrap();
    con.llen("connections").unwrap()
}
