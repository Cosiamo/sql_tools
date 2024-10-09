use std::thread::{self, JoinHandle};

use crate::{errors::Error, select::{utils::stmt_res, SelectBuilder, SelectProps, WhereClause}, utils::remove_invalid_chars, QueryBuilder, QueryTypes, SQLTypes};

#[derive(Debug)]
pub struct OracleConnect {
    pub connection_string: String,
    pub username: String,
    pub password: String,
}

impl OracleConnect {
    pub fn new(connection_string: &str, username: &str, password: &str) -> Self {
        Self {
            connection_string: connection_string.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

impl QueryBuilder for OracleConnect {
    fn select(self, table: &str, columns: Vec<String>) -> SelectProps {
        let fmt_cols = columns.iter().map(|cols| {
            remove_invalid_chars(cols)
        }).collect::<Vec<String>>();
        SelectProps {
            connect: SQLTypes::Oracle(self),
            columns: fmt_cols,
            table: table.to_string(),
            clause: None,
        }
    }
}

impl SelectBuilder for SelectProps {
    fn filter(self, column: &str, value: &str) -> WhereClause {
        let where_clause = format!("{} {}", column, value);
        // self.query = format!("{} WHERE {}", self.query, &where_clause);
        WhereClause { 
            query_type: QueryTypes::Select(self),
            clause: where_clause
        }
    }

    fn build(self) -> Result<Vec<Vec<Option<String>>>, Error> {
        let conn_info = match self.connect {
            SQLTypes::Oracle(oracle_connect) => oracle_connect,
        };
        // println!("{:?}", self.columns);
        let query: String;

        let count_sql: String;
        match self.clause {
            Some(filters) => {
                count_sql = format!("SELECT COUNT(*) FROM {} WHERE {}", &self.table, &filters);
                query = format!("SELECT row_number() over (order by rowid) as rn, {} FROM {} WHERE {}", &self.columns.join(", "), &self.table, filters);
            },
            None => {
                count_sql = format!("SELECT COUNT(*) FROM {}", &self.table);
                query = format!("SELECT row_number() over (order by rowid) as rn, {} FROM {}", &self.columns.join(", "), &self.table);
            },
        }

        let mut count: Option<usize> = None;
        let conn: oracle::Connection = oracle::Connection::connect(&conn_info.username, &conn_info.password, &conn_info.connection_string).unwrap(); 
        let count_query = conn.query(&count_sql, &[])?;
        for res in count_query {
            let row = res?;
            count = Some(row.get_as::<usize>()?);
        };

        let len: usize = if let Some(val) = count { val } else { return Err(Error::CountError) };
        let nthreads = num_cpus::get();
        let num = (len / nthreads + if len % nthreads == 0 { 0 } else { 1 }) as f32;

        let mut handles: Vec<JoinHandle<Result<Vec<Vec<Option<String>>>, Error>>> = Vec::new();

        let mut c: usize = 0;
        let mut prev: usize = 0;

        let col_len = self.columns.len() + 1;

        for n in 0..nthreads {
            let start: usize;
            if n == 0 { start = 1 }
            else { start = prev + 1 }
            let mut end = (c + 1) * num.ceil() as usize;
            if end > len { end = len }
            // println!("Start:{}  End:{}", start, end);
            let stmt = format!("SELECT * FROM ({}) WHERE rn >= {} and rn <= {}", query, start, end);
            // println!("{:?}", stmt);
            let username = conn_info.username.to_owned();
            let password = conn_info.password.to_owned();
            let connection_string = conn_info.connection_string.to_owned();
            
            handles.push(thread::spawn(move || {
                let conn: oracle::Connection = oracle::Connection::connect(username, password, connection_string).unwrap(); 
                let stmt = conn.statement(&stmt).build()?;
                stmt_res(stmt, col_len)
            }));
            prev = end;
            c += 1;
        }

        let mut group = Vec::new();
        for handle in handles {
            let mut handle = handle.join().unwrap()?;
            let res = handle.iter_mut().map(|row|{
                let _ = row.remove(0);
                row.to_owned()
            }).collect::<Vec<Vec<Option<String>>>>();
            group.push(res);
        }
        let res = group.concat();
        // res.iter().for_each(|c|{ println!("{:?}", c) });

        Ok(res)
    }
    
    fn build_single_thread(self) -> Result<Vec<Vec<Option<String>>>, Error> {
        let conn_info = match self.connect {
            SQLTypes::Oracle(oracle_connect) => oracle_connect,
        };
        let query = match self.clause {
            Some(filters) => format!("SELECT {} FROM {} WHERE {}", &self.columns.join(", "), &self.table, filters),
            None => format!("SELECT {} FROM {}", &self.columns.join(", "), &self.table),
        };
        let conn: oracle::Connection = oracle::Connection::connect(conn_info.username, conn_info.password, conn_info.connection_string).unwrap(); 
        let stmt = conn.statement(&query).build()?;
        stmt_res(stmt, self.columns.len())
    }
}