#![deny(warnings)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
use hexarch::AppError;
use async_trait::async_trait;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Create the repo
    let user_repo = UserRepo { _connection: "proto:mydb/foo".into()};

    api_handler_get_user(user_repo).await?;

    Ok(())
}

// ------------------------------------------------------------------------------------------
// API
async fn api_handler_get_user<T: UserLoader + 'static>(loader: T) -> Result<(), AppError> {
    let user_id = 42;  // Eg read input from somewhere
    let get_user = GetUser::new(user_id);
    let user = get_user.run1(loader).await?;
    println!("User: {}",user.name);
    Ok(())
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

    pub async fn run1<T:UserLoader>(self, user_loader: T) -> Result<User, AppError> {
        let id = self.user_id;
        user_loader.load_user(id).await
    }
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
