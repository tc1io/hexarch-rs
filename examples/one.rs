#![deny(warnings)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
use hexarch::AppError;
use async_trait::async_trait;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Create the repo
    let user_repo = UserRepo { _connection: "proto:mydb/foo".into()};

    // Create the use case
    let get_user = GetUser::new(user_repo);

    // Supply use case to handler.
    api_handler_get_user(get_user).await?;

    Ok(())
}

// ------------------------------------------------------------------------------------------
// API
async fn api_handler_get_user<T>(use_case: GetUser<T>) -> Result<(), AppError> {
    let user_id = 42;  // Eg read input from somewhere
    let user = use_case.apply(user_id).await?;
    //let user = use_case(user_id).await?;
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

pub struct GetUser<T: UserLoader + 'static> {
    user_loader: T,
}
impl<T> GetUser<T> {
    pub fn new(user_loader: T) -> GetUser<T> {
        GetUser { user_loader }
    }
}

impl<T> GetUser<T> {
    pub fn apply(self, id: i32) -> impl std::future::Future<Output=Result<User, AppError>> + 'static {
        //Box::new(self.user_loader.load_user(id))
        self.user_loader.load_user(id)
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



// ------------------------------------------------------------------------------------------
// This struct will hold the repository - by using a struct here, instead of
// using a makeUseCase() function, we have the benefit that the result is
// a struct type, not some strange compile time inferred Fn type - this makes
// it much easier to deal with lifetimes (I think) - in the end, Rust closures are
// structs anyway, so there really is not much of a difference
// pub struct GetAFoo<'a,T> {
//     user_loader: T,
//     phantom: std::marker::PhantomData<&'a T>,
// }
// // just a constructor for the use case ( similar to the MakeUseCase..  functions we use in Go)
// impl<'a,T> GetAFoo<'a,T> {
//     pub fn new(loader:T) -> GetAFoo<'a,T> {
//         GetAFoo{loader, phantom: std::marker::PhantomData}
//     }
//
// }
// Make the use case callable with i32 IDs.
// impl<'a,T: UserLoader + 'static> FnOnce<(i32,)> for GetAFoo<'a,T> {
//     type Output = Box<dyn std::future::Future<Output=Result<entities::Foo, error::AppError>>>;
//     extern "rust-call" fn call_once(self, args: (i32,)) -> Box<dyn std::future::Future<Output=Result<entities::Foo, error::AppError>>> {
//         Box::new(self.loader.load_foo(args.0))
//     }
// }

// The HTTP GET handler would take an explicit GetAFoo parameter:
//
// for example in handlers/foo.rs
// ------------------------------
// fn handleGetFoo(uc: GetAFoo) {
//   let id = request param "id"
//   foo = getAFoo(id)
//   respond with foo body
// }

// in main.rs:
// ------------------------------
// let repo = MyRepo::new()
// let getAFoo = GetAFoo::new(repo)
//
// now getAFoo is a var that *is* the use case with the repo inside
//
// since we implement FnOnce trait for GetAFoo we can call it with specific IDs like this:
//
// foo = getAFoo(42)
//
// Meaning: we can leverage the fact that we can turn any struct into a closure by implementing
// FnOnce, FnMut, or Fn traits on it.



// impl<T: UserLoader + 'static> FnOnce<(i32,)> for GetAFoo<T>
// {
//     type Output = Box<dyn std::future::Future<Output=Result<User, AppError>>>;
//     extern "rust-call" fn call_once(self, args: (i32,)) -> Box<dyn std::future::Future<Output=Result<User, AppError>>> {
//         Box::new(self.loader.load_foo(args.0))
//     }
// }
// impl<T: 'static + UserLoader> FnMut<(i32,)> for GetAFoo<T> {
//     extern "rust-call" fn call_mut(&mut self, args: (i32,)) -> Box<dyn std::future::Future<Output=Result<entities::Foo, error::AppError>>> {
//         Box::new(self.loader.load_foo(args.0))
//     }
// }
// impl<T: 'static + UserLoader> Fn<(i32,)> for GetAFoo<T> {
//     extern "rust-call" fn call(&self, args: (i32,)) -> Box<dyn std::future::Future<Output=Result<entities::Foo, error::AppError>>> {
//         Box::new(self.loader.load_foo(args.0))
//     }
// }
