#![deny(warnings)]
use hexarch::AppError;
use async_trait::async_trait;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let repo = Repo {conn: "xxx".into()};
    let uc = GetAFoo::new(repo);
    let y = uc.apply(42).await?;
    println!("{}",y);
    Ok(())
}

// The 'interface' that the repo has to implement. it is defined in the use case layer
// like we do in go
#[async_trait]
pub trait FooLoader {
    async fn load_foo(self, id: i32) -> Result<String, AppError>;
}

pub struct Repo {
    conn: String
}

#[async_trait]
impl FooLoader for Repo {
    async fn load_foo(self, _id: i32) -> Result<String, AppError> {
        let foo: String = self.conn.clone();
        Ok(foo)
    }
}


// This struct will hold the repository - by using a struct here, instead of
// using a makeUseCase() function, we have the benefit that the result is
// a struct type, not some strange compile time inferred Fn type - this makes
// it much easier to deal with lifetimes (I think) - in the end, Rust closures are
// structs anyway, so there really is not much of a difference
// pub struct GetAFoo<'a,T> {
//     loader: T,
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
// impl<'a,T: FooLoader + 'static> FnOnce<(i32,)> for GetAFoo<'a,T> {
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

pub struct GetAFoo<T> {
    loader: T,
}
// just a constructor for the use case ( similar to the MakeUseCase..  functions we use in Go)
impl<T> GetAFoo<T> {
    pub fn new(loader: T) -> GetAFoo<T> {
        GetAFoo { loader }
    }
}

// impl<T: FooLoader + 'static> GetAFoo<T> {
//     pub fn apply(&self, id: i32) -> Box<dyn std::future::Future<Output=Result<String, AppError>> + '_> {
//          Box::new(self.loader.load_foo(id))
//      }
// }
impl<T: FooLoader + 'static> GetAFoo<T> {
    pub fn apply(self, id: i32) -> impl std::future::Future<Output=Result<String, AppError>> + 'static {
        //Box::new(self.loader.load_foo(id))
        self.loader.load_foo(id)
    }
}

// impl<T: FooLoader + 'static> FnOnce<(i32,)> for GetAFoo<T> {
//     type Output = Box<dyn std::future::Future<Output=Result<String, error::AppError>>>;
//     extern "rust-call" fn call_once(self, args: (i32,)) -> Box<dyn std::future::Future<Output=Result<String, AppError>>> {
//
//         Box::new(self.loader.load_foo(args.0))
//     }
// }
// impl<T: 'static + FooLoader> FnMut<(i32,)> for GetAFoo<T> {
//     extern "rust-call" fn call_mut(&mut self, args: (i32,)) -> Box<dyn std::future::Future<Output=Result<entities::Foo, error::AppError>>> {
//         Box::new(self.loader.load_foo(args.0))
//     }
// }
// impl<T: 'static + FooLoader> Fn<(i32,)> for GetAFoo<T> {
//     extern "rust-call" fn call(&self, args: (i32,)) -> Box<dyn std::future::Future<Output=Result<entities::Foo, error::AppError>>> {
//         Box::new(self.loader.load_foo(args.0))
//     }
// }
