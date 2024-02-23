use crate::method::Verb::{*};

#[derive(Debug, PartialEq)]
pub struct Method(Verb);

#[derive(Debug, PartialEq)]
enum Verb {
    GET,
    POST,
    PUT,
    PATCH,
    OPTIONS,
    DELETE,
    HEAD,
    CONNECT,
    TRACE,
}

impl Method {
    ///GET
    pub const GET: Method = Method(GET);

    ///POST
    pub const POST: Method = Method(POST);

    ///PUT
    pub const PUT: Method = Method(PUT);

    ///PATCH
    pub const PATCH: Method = Method(PATCH);

    ///OPTIONS
    pub const OPTIONS: Method = Method(OPTIONS);

    ///DELETE
    pub const DELETE: Method = Method(DELETE);

    ///HEAD
    pub const HEAD: Method = Method(HEAD);

    ///CONNECT
    pub const CONNECT: Method = Method(CONNECT);

    ///TRACE
    pub const TRACE: Method = Method(TRACE);

    /// Return a &str representation of the HTTP method
    pub fn to_str(&self) -> &str {
        match self.0 {
            GET => "GET",
            POST => "POST",
            PUT => "PUT",
            PATCH => "PATCH",
            OPTIONS => "OPTIONS",
            DELETE => "DELETE",
            HEAD => "HEAD",
            CONNECT => "CONNECT",
            TRACE => "TRACE"
        }
    }

    /// Converts string slice to an HTTP method.
    pub fn from_str(src: &str) -> Method {
        match src {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "PATCH" => Method::PATCH,
            "OPTIONS" => Method::OPTIONS,
            "DELETE" => Method::DELETE,
            "HEAD" => Method::HEAD,
            "CONNECT" => Method::CONNECT,
            "TRACE" => Method::TRACE,
            _ => todo!()
        }
    }
}


impl PartialEq<str> for Method {
    fn eq(&self, other: &str) -> bool {
        self.to_str() == other
    }
}

impl PartialEq<Method> for str {
    fn eq(&self, other: &Method) -> bool {
        self == other.to_str()
    }
}


impl<'a> PartialEq<&'a str> for Method {
    fn eq(&self, other: &&'a str) -> bool {
        self.to_str() == *other
    }
}

impl<'a> PartialEq<Method> for &'a str {
    fn eq(&self, other: &Method) -> bool {
       *self == other.to_str()
    }
}

impl<'a> PartialEq<&'a Method> for Method{
    fn eq(&self, other: &&'a Method) -> bool {
        self == * other
    }
}

impl<'a> PartialEq<Method> for &'a Method{
    fn eq(&self, other: &Method) -> bool {
        *self == other
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_method_eq() {

        assert_eq!(Method::GET, Method::GET);

        assert_eq!(Method::GET, Method::from_str("GET"));
        assert_eq!(Method::GET.to_str(), "GET");

        assert_eq!("GET", Method::GET);
        assert_eq!(Method::GET,"GET");

        assert_eq!("GET",&Method::GET);
        assert_eq!(&Method::GET,"GET");

        assert_eq!(Method::GET,&Method::GET);
        assert_eq!(&Method::GET,Method::GET);

    }
}