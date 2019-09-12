use actix_web::{error, middleware, web, App, Error, HttpResponse, HttpServer};
use bytes::BytesMut;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use futures::future::{err, Either};
use futures::{Future, Stream};

mod schema;
use schema::users;
use crate::common::define;

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub phone: i32,
    pub name: String,
    pub code: i32,
    pub code_last_time: i64,
}


pub struct NewUser<'a> {
    pub phone: &'a i32,
    pub name: &'a str,
}

/// Diesel query
fn query(
    ph: i32,
    pool: web::Data<Pool>,
) -> Result<User, diesel::result::Error> {
    use self::schema::users::dsl::*;

    let conn = &pool.get().unwrap();
    let mut items = users.filter(phone.eq(&ph)).load::<User>(conn)?;
    Ok(items.pop().unwrap())
}

const MAX_SIZE: usize = 262_144; // max payload size is 256k

pub fn verify_code(
    item: web::Json<User>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || query(item.into_inner().phone, pool)).then(|res| match res {
        Ok(user) => {
            if item.into_inner().code == user.code {
                Ok(HttpResponse::Ok().json(user))
            } else {
                Err(error::ErrorBadRequest("code is wrong!"))
            }
        },
        Err(_) =>  {
            Err(error::ErrorBadRequest("phone is wrong!"))
        }
    })
}

//This handler manually load request payload and parse json object
/*pub fn index_add(
    pl: web::Payload,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    pl
        // `Future::from_err` acts like `?` in that it coerces the error type from
        // the future into the final error type
        .from_err()
        // `fold` will asynchronously read each chunk of the request body and
        // call supplied closure, then it resolves to result of closure
        .fold(BytesMut::new(), move |mut body, chunk| {
            // limit max size of in-memory payload
            if (body.len() + chunk.len()) > MAX_SIZE {
                Err(error::ErrorBadRequest("overflow"))
            } else {
                body.extend_from_slice(&chunk);
                Ok(body)
            }
        })
        // `Future::and_then` can be used to merge an asynchronous workflow with a
        // synchronous workflow
        //
        // Douman NOTE:
        // The return value in this closure helps, to clarify result for compiler
        // as otheriwse it cannot understand it
        .and_then(move |body| {
            // body is loaded, now we can deserialize serde-json
            let r_obj = serde_json::from_slice::<User>(&body);

            // Send to the db for create
            match r_obj {
                Ok(obj) => {
                    Either::A(web::block(move || query(obj.phone, obj.name, pool)).then(|res| {
                        match res {
                            Ok(user) => Ok(HttpResponse::Ok().json(user)),
                            Err(_) => Ok(HttpResponse::InternalServerError().into()),
                        }
                    }))
                }
                Err(_) => Either::B(err(error::ErrorBadRequest("Json Decode Failed"))),
            }
        })
}*/
