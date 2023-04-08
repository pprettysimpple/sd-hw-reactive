// here is server that is accepting requests from clients
// and forwards them to the service
// server is using hyper to accept requests
use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use log::info;
use serde_json::Value;
use tokio_postgres::Config;
use crate::db::iface::connector::Connector;
use crate::service_context::ServiceContext;

// parse request and proxies it to the service
// every request input is provided as a json
// json serialization is done with serde_json
// format of the request is:
// /api/<method_name>
// {
//     <method_input>
// }
async fn handle(
    context: ServiceContext,
    req: Request<Body>,
) -> Result<Response<Body>, Infallible> {
    let uri = req.uri().path().to_string();
    info!("Received request: {}", uri);
    info!("Request body: {:?}", req.body());
    let result: Result<Response<Body>, String> = match uri.as_str().trim_start_matches("/api") {
        "/create_user" => {
            let body = hyper::body::to_bytes(req.into_body()).await.unwrap();
            let user = serde_json::from_slice(&body).unwrap();
            context
                .create_user(user)
                .await
                .map(|_| Response::new(Body::from(serde_json::to_string("").unwrap())))
        }
        "/get_user" => {
            let body = hyper::body::to_bytes(req.into_body()).await.unwrap();
            // read id from json map
            let v: Value = serde_json::from_str(&String::from_utf8(body.to_vec()).unwrap()).unwrap();
            let id = v["id"].as_str().unwrap().to_string();
            context
                .get_user(id)
                .await
                .map(|user| Response::new(Body::from(serde_json::to_string(&user).unwrap())))
        }
        "/add_product" => {
            let body = hyper::body::to_bytes(req.into_body()).await.unwrap();
            let product = serde_json::from_slice(&body).unwrap();
            context
                .add_product(product)
                .await
                .map(|_| Response::new(Body::from(serde_json::to_string("").unwrap())))
        }
        "/get_product" => {
            let body = hyper::body::to_bytes(req.into_body()).await.unwrap();
            let v: Value = serde_json::from_str(&String::from_utf8(body.to_vec()).unwrap()).unwrap();
            let id = v["id"].as_str().unwrap().to_string();
            context
                .get_product(id)
                .await
                .map(|product| Response::new(Body::from(serde_json::to_string(&product).unwrap())))
        }
        "/get_available_products" => {
            let body = hyper::body::to_bytes(req.into_body()).await.unwrap();
            let v: Value = serde_json::from_str(&String::from_utf8(body.to_vec()).unwrap()).unwrap();
            let user_id = v["user_id"].as_str().unwrap().to_string();
            context
                .get_available_products(user_id)
                .await
                .map(|products| Response::new(Body::from(serde_json::to_string(&products).unwrap())))
        }
        "/get_products_by_currency" => {
            let body = hyper::body::to_bytes(req.into_body()).await.unwrap();
            let v: Value = serde_json::from_str(&String::from_utf8(body.to_vec()).unwrap()).unwrap();
            let currency = v["currency"].as_str().unwrap().to_string();
            context
                .get_products_by_currency(currency)
                .await
                .map(|products| Response::new(Body::from(serde_json::to_string(&products).unwrap())))
        }
        "/index.html" => {
            match std::fs::read_to_string("src/catlog/web/index.html") {
                Ok(body) => Ok(Response::new(Body::from(body))),
                Err(err) => Err(err.to_string()),
            }
        }
        _ => {
            Ok(Response::builder()
                .status(404)
                .body(Body::from("Not found")).unwrap())
        }
    };

    info!("Response: {:?}", result);

    match result {
        Ok(resp) => Ok(resp),
        Err(err) => Ok(
            Response::builder()
                .status(500)
                .body(Body::from(err))
                .expect("Failed to create response"))
    }
}

pub async fn run_catlog_server(pg_config: Config) {
    let context = ServiceContext::new(pg_config);

    let make_service = make_service_fn(move |_conn: &AddrStream| {
        let context = context.clone();

        let service = service_fn(move |req| {
            handle(context.clone(), req)
        });

        async move { Ok::<_, Infallible>(service) }
    });

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

/*
format of the request is:
/api/<method_name>
{
    <method_input>
}

methods are:
/create_user
    input:
        id: string
        currency: string
    output:
        nothing
/get_user
    input:
        id: string
    output:
        id: string
        currency: string

/add_product
    input:
        id: string
        currency: string
    output:
        nothing

/get_product
    input:
        id: string
    output:
        id: string
        currency: string

/get_available_products
    input:
        user_id: string
    output:
        [
            {
                id: string
                currency: string
            }
        ]

/get_products_by_currency
    input:
        currency: string
    output:
        [
            {
                id: string
                currency: string
            }
        ]
 */