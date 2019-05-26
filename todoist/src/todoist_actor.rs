use actix::prelude::*;
use types::primitives::Integer;
use shopping_list_api::TodoistApi;
use shopping_list_api::ShoppingListApi;

type ShoppingListFuture = Box<Future<Item=(), Error=hyper::Error>>;

pub struct ShoppingListItem {
    item: String
}

impl Message for ShoppingListItem {
    type Result = Result<(), hyper::Error>;
}

pub struct ShoppingListActor {
    api: TodoistApi,
    project_id: Integer
}

impl Actor for ShoppingListActor {
    type Context = Context<Self>;
}

impl Handler<ShoppingListItem> for ShoppingListActor {
    type Result = ShoppingListFuture ;

    fn handle(&mut self, msg: ShoppingListItem, _: &mut Context<Self>) -> Self::Result {
        self.api.add_task(&msg.item, self.project_id)
    }
}