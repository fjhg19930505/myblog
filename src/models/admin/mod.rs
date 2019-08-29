use actix_web::{HttpRequest, HttpResponse, web}

/// Async request handler
fn add(
    name: web::Path<String>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // run diesel blocking code
    web::block(move || query(name.into_inner(), pool)).then(|res| match res {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

#[derive(Debug, Serialize, Deserialize)]
struct MyUser {
    phone: i32,
    name: String,
    code: i32,
    last_code_time : String,
}

const MAX_SIZE: usize = 262_144; // max payload size is 256k

/// This handler manually load request payload and parse json object
fn index_add(
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
            let r_obj = serde_json::from_slice::<MyUser>(&body);

            // Send to the db for create
            match r_obj {
                Ok(obj) => {
                    Either::A(web::block(move || query(obj.name, pool)).then(|res| {
                        match res {
                            Ok(user) => Ok(HttpResponse::Ok().json(user)),
                            Err(_) => Ok(HttpResponse::InternalServerError().into()),
                        }
                    }))
                }
                Err(_) => Either::B(err(error::ErrorBadRequest("Json Decode Failed"))),
            }
        })
}