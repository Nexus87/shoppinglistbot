// use std::future::Future;
// use actix::prelude::*;
// use crate::types::primitives::Integer;
// use crate::shopping_list_api::TodoistApi;
// use crate::shopping_list_api::ShoppingListApi;
// use async_trait::async_trait;
// 
// type ShoppingListFuture = Box<dyn Future<Output=Result<(), reqwest::Error>>>;
// 
// pub struct ShoppingListItem {
//     item: String
// }
// 
// impl Message for ShoppingListItem {
//     type Result = Result<(), hyper::Error>;
// }
// 
// pub struct ShoppingListActor {
//     api: TodoistApi,
//     project_id: Integer
// }
// 
// impl Actor for ShoppingListActor {
//     type Context = Context<Self>;
// }
// 
// #[async_trait]
// impl Handler<ShoppingListItem> for ShoppingListActor {
//     type Result = ShoppingListFuture ;
// 
//     fn handle(&mut self, msg: ShoppingListItem, _: &mut Context<Self>) -> Self::Result {
//         self.api.add_task(&msg.item, self.project_id)
//     }
// }