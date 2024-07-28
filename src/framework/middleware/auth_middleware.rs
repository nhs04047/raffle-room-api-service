use actix_web::error::ErrorUnauthorized;
use actix_web::{dev::ServiceRequest, Error};
use actix_web::dev::{Service, ServiceResponse, Transform};
use futures::future::{ok, Ready};
use futures::task::{Context, Poll};
use std::pin::Pin;
use std::rc::Rc;

// Define the middleware structure
pub struct AuthKeyMiddleware {
  valid_key: String,
}

impl AuthKeyMiddleware {
  pub fn new(valid_key: String) -> Self {
    AuthKeyMiddleware { valid_key }
  }
}

// Implement Transform for AuthKeyMiddleware
impl<S, B> Transform<S, ServiceRequest> for AuthKeyMiddleware
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
  S::Future: 'static,
  B: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type InitError = ();
  type Transform = AuthKeyMiddlewareService<S>;
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ok(AuthKeyMiddlewareService {
      service: Rc::new(service),
      valid_key: self.valid_key.clone(),
    })
  }
}

// Define the service structure
pub struct AuthKeyMiddlewareService<S> {
  service: Rc<S>,
  valid_key: String,
}

// Implement Service for AuthKeyMiddlewareService
impl<S, B> Service<ServiceRequest> for AuthKeyMiddlewareService<S>
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
  S::Future: 'static,
  B: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Future = Pin<Box<dyn futures::Future<Output = Result<Self::Response, Self::Error>>>>;

  fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self.service.poll_ready(cx)
  }

  fn call(&self, req: ServiceRequest) -> Self::Future {
    let auth_key = req.headers().get("Authorization");
    if let Some(key) = auth_key {
      let data = key.to_str();

      match data {
        Ok(auth_val) => {
            let d = format!("{}",auth_val);
            if d != self.valid_key {
                return Box::pin(async move{
                    Err(ErrorUnauthorized("Invalid auth credentials"))
                });
            }
        },
        Err(e) => {
            return Box::pin(async move {
                Err(ErrorUnauthorized(e.to_string()))
            });
        },
      }
    }else {
      return Box::pin(async move{
          Err(ErrorUnauthorized("Authentication Value not found"))
      });
    }
    let fut = self.service.call(req);

    Box::pin(async move {
        let res = fut.await?;

        Ok(res)
    }) 
  }
}
