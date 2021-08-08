#![deny(warnings)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
use hexarch::AppError;
use async_trait::async_trait;
use hexarch::RF;
//use std::process::Output;
use std::future::Future;
use std::pin::Pin;


#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Create the repo
    let user_repo = UserRepo { _connection: "proto:mydb/foo".into()};

    api_handler_get_user(user_repo).await?;

    Ok(())
}


// ------------------------------------------------------------------------------------------
// API
async fn api_handler_get_user<T: UserLoader + Send + 'static>(loader: T) -> Result<(), AppError> {
    let user_id = 42;  // Eg read input from somewhere
    // let get_user = GetUser::new(user_id);
    // //let get_ = GetUser::new(user_id);
    //
    // let user = get_user.run(loader).await?;

    // This is the function that we can give a loader to actually run and get the user
    let get_user_func = getuser(user_id);
    // --------------------------------------------------------
    // Note on the side:
    // such functions we would actually like to be able to compose into bigger use cases
    // like this for example:
    // let usecase = getuser.and_then(get_address);
    // let user_and_address_func = usecase(user_id)
    // --------------------------------------------------------

    // here we give the function a loader to get the user
    let user = get_user_func(loader).await?;

    println!("User: {}",user.name);

    Ok(())
}

// use case
// async fn getuser<I:UserLoader>(user_loader: I,id:i32) -> Result<User, AppError> {
//     user_loader.load_user(id).await
// }

fn getuser<I:UserLoader + 'static>(id:i32) -> impl FnOnce(I) -> Pin<Box<dyn Future<Output=Result<User, AppError>>>> {
 move |user_loader: I| {
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

/// GetUser represents a partially applied use case, IOW, a
/// use case that is already bound do a specific ID to retrieve
/// and can be executed later on on an adapter
pub struct GetUser {
    user_id: i32
}
impl GetUser {
    pub fn new(user_id: i32) -> GetUser {
        GetUser { user_id }
    }
}


#[async_trait]
impl<I:UserLoader + Send + 'static> RF<I> for GetUser {
    type Output = Result<User, AppError>;

    async fn run(self, user_loader: I) -> Result<User, AppError> {
        let id = self.user_id;
        user_loader.load_user(id).await
    }
}

// pub struct And<> {
//
// }
// impl<I:UserLoader> RF<I> for And {
//     type Output = Result<User, AppError>;
//     async fn run(self, user_loader: I) -> Result<User, AppError> {
//         let id = self.user_id;
//         user_loader.load_user(id).await
//     }
// }




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
