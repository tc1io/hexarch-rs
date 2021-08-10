#![deny(warnings)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
use hexarch::AppError;
use async_trait::async_trait;
// use hexarch::RF;
//use std::process::Output;
use std::future::Future;
use std::pin::Pin;


#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Create the repo
    let user_repo = UserRepo { _connection: "proto:mydb/foo".into()};
    // Create use-case layer and pass the repo as input
    let get_user_fun = get_user(user_repo);
    // pass use-case layer to adapter
    api_handler_get_user(get_user_fun).await?;

    Ok(())
}


// ------------------------------------------------------------------------------------------
// API
async fn api_handler_get_user(
    get_user_func: impl FnOnce(i32)
        -> Pin<Box<dyn Future<Output=Result<User, AppError>>>>) -> Result<(), AppError> {
    let user_id = 42;  // Eg read input from somewhere

    // here we give the function an Id to get the user
    // logic and the repo is already pre-loaded
    let user = get_user_func(user_id).await?;

    println!("User: {}",user.name);

    Ok(())
}


fn get_user<I:UserLoader + 'static>(user_loader: I)
    -> impl FnOnce(i32) -> Pin<Box<dyn Future<Output=Result<User, AppError>>>> {
    move |id: i32| {
        user_loader.load_user(id)
    }
}


// ------------------------------------------------------------------------------------------
// Entities
pub struct User {
    name:String
}

// ------------------------------------------------------------------------------------------
// Use Cases
#[async_trait]
pub trait UserLoader {
    async fn load_user(self, id: i32) -> Result<User, AppError>;
}


// ------------------------------------------------------------------------------------------
// Adapters
pub struct UserRepo {
    _connection: String
}

#[async_trait]
impl UserLoader for UserRepo {
    async fn load_user(self, id: i32) -> Result<User, AppError> {
        let user = User{name: format!("User {}",id)};
        Ok(user)
    }
}
