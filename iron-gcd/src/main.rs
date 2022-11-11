extern crate iron;
#[macro_use] extern crate mime;

use iron::prelude::*;
use iron::status;

fn main() {
  print!("hello");
  Iron::new(get_form).http("localhost:3000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
  let mut response = Response::new();

  response.set_mut(status::Ok);
  response.set_mut(mime!(Text/Html; Charset=Utf8));
  response.set_mut(r#"
    <title>GCD</title>
    <form action="/gcd" method="post">
    <input name=""n />
    <input name=""n />
    <button type="submit">Compute</button>
    </form>
  "#);

  // Ok(response); // 为什么不写 return 会报错
  return Ok(response);
}