use crate::hashing;

#[derive(Clone)]
pub struct Connection{
    pub id : i32,
    pub ip : String,
    pub port : i8,
}
pub struct Con{
    id : i32,
    ip : String,
    port : String,
}

pub fn Con_to_Connection(con : &Con)->Connection{
    Connection{
        id : con.id.to_owned(),
        ip : con.ip.to_string(),
        port : con.port.to_owned().parse::<i8>().unwrap(),
    }
}

pub fn get_connection()-> rusqlite::Connection{
    let conn = rusqlite::Connection::open("hashes.db").unwrap();
    conn
}

pub async fn clean_server(){
    let conn = get_connection();
    conn.execute("DELETE FROM cons", []).unwrap();
}

pub async fn add_connection(ip : &str, port : i8){
    println!("Adding connection");
    let conn = get_connection();
    println!("Adding connection : {}:{}" , ip , port);
    let mut stmt = conn.prepare("SELECT * FROM cons WHERE ip = ? AND port = ?").unwrap();
    let rows = stmt.query_map([ip , port.to_string().as_str()] , |_row|{Ok(1)}).unwrap();
    
    if rows.count() == 0{
        println!("Adding connection3");
        conn.execute("INSERT INTO cons (ind , ip , port) VALUES (? , ?)", [get_next_index().to_owned().to_string().as_str(), ip , port.to_string().as_str()]).unwrap();
        println!("Added connection");
    }
    else{
        println!("Connection already exists");
    }
}




pub async fn get_connections() -> Vec<Connection>{
    println!("Getting connections");
    let conn = get_connection();
    let mut stmt = conn.prepare("SELECT * FROM cons").unwrap();
    let rows = stmt.query_map([], |row|{
        Ok(Con{
            id : row.get(0).unwrap(),
            ip : row.get(1).unwrap(),
            port : row.get(2).unwrap(),
        })
    }).unwrap();
    let mut cons = Vec::new();
    for row in rows{
        cons.push(Con_to_Connection(&row.unwrap()));
    }

    cons
}






pub async fn get_nth_connection(n : i32) -> Connection{
    let conn = get_connection();
    let mut stmt = conn.prepare("SELECT * FROM cons WHERE ind = ?").unwrap();
    let rows = stmt.query_map([n], |row|{
        Ok(Con{
            id : row.get(0).unwrap(),
            ip : row.get(1).unwrap(),
            port : row.get(2).unwrap(),
        })
    }).unwrap();
    let mut cons = Vec::new();
    for row in rows{
        cons.push(row.unwrap());
    }
    return Con_to_Connection(cons.get(n as usize).unwrap().clone());
}





pub async fn remove_connection(id : i32){
    let conn = get_connection();
    conn.execute("DELETE FROM cons WHERE id = ?", [id]).unwrap();
}



pub async fn get_connections_len() -> i32{
    println!("Getting connections len");
    let conn = get_connection();
    let mut stmt = conn.prepare("SELECT * FROM cons").unwrap();
    let rows = stmt.query_map([], |row|{
        Ok(Connection{
            id : row.get(0).unwrap(),
            ip : row.get(1).unwrap(),
            port : row.get(2).unwrap(),
        })
    }).unwrap();
    let mut cons = Vec::new();
    for row in rows{
        cons.push(row.unwrap());
    }
    println!("Connection's len : {}", cons.len());
    cons.len() as i32
}



pub fn get_next_index() -> i32{
    let conn = get_connection();
    let mut stmt = conn.prepare("SELECT * FROM cons order by ind").unwrap();
    let rows = stmt.query_map([], |row|{
        Ok(Con{
            id : row.get(0).unwrap(),
            ip : row.get(1).unwrap(),
            port : row.get(2).unwrap(),
        })
    }).unwrap();
    let mut cons = Vec::new();
    for row in rows{
        cons.push(row.unwrap());
    }
    for i in 0..cons.len(){
        if cons.get(i).unwrap().id!=(i+1) as i32{
            return (i+1) as i32;
        }
    }
    (cons.len()+1) as i32
}