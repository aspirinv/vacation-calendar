use std::sync::Arc;
use mongodb::Database;


pub struct State{
    pub db: Arc<Database>
}


// impl State{
//     pub fn get_db_conn(&self) -> Client {
//         self.db.into()
//     }
// }
