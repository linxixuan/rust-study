extern crate iron;
extern crate router;
#[macro_use] extern crate mime;

use iron::prelude::*;
use iron::status;
use std::str::FromStr;
use router::Router;
use urlencoded::UrlEncodedBody;

fn main() {
  let mut router = Router::new();

  router.get("/", get_form, "root");
  router.post("/gcd", post_gcd, "gcd");

  print!("Serving on http://localhost:3000...");
  Iron::new(router).http("localhost:3000").unwrap();
}

fn post_gcd(request: &mut Request) -> IronResult<Response> {
  let mut response = Response::new();

  let form_data = match request.get_ref::<UrlEncodedBody>() {
    Err(e) => {
      response.set_mut(status::BadRequest);
      response.set_mut(format!("error parsing form data: {:?}\n", e));
      return Ok(response);
    }
    Ok(map) => map
  };

  let unparsed_numbers = match form_data.get("n") {
    None => {
      response.set_mut(status::BadRequest);
      response.set_mut(format!("form data has no 'n' parameter\n"));
      return Ok(response);
    }
    Some(nums) => nums
  };

  let mut numbers = Vec::new();
  for unparsed in unparsed_numbers {
    match u64::from_str(&unparsed) {
      Err(_) => {
        response.set_mut(status::BadRequest);
        response.set_mut(
          format!("Value for 'n' parameter not a number: {:?}\n", unparsed)
        );
        return Ok(response)
      }
      Ok(n) => {
        numbers.push(n);
      }
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
      d = gcd(d, *m);
    }

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(
      format!("greatest common divisor of the numbers {:?} is <b>{}</b>", numbers, d)
    );
  }
  return Ok(response);
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
  assert!(n != 0 && m != 0);
    while m != 0 {
      if m < n {
        let t = m;
        m = n;
        n = t;
      }

      m = m % n;
    }
  return n;
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
  let mut response = Response::new();

  response.set_mut(status::Ok);
  response.set_mut(mime!(Text/Html; Charset=Utf8));
  response.set_mut(r#"
    <title>GCD</title>
    <form action="/gcd" method="post">
    <input name="n" />
    <input name="n" />
    <button type="submit">Compute</button>
    </form>
  "#);

  // Ok(response); // ??????????????? return ?????????
  return Ok(response);
}