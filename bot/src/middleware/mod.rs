use std::panic::AssertUnwindSafe;
use sled::Db;

pub struct  SledMiddleware {
    db: AssertUnwindSafe<Db>   
}