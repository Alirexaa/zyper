use crate::method::Verb::{*};

pub struct Method(Verb);

enum Verb {
    GET,
    POST,
    PUT,
    PATCH,
    OPTIONS,
    DELETE,
    HEAD,
    CONNECT,
    TRACE
}

impl Method{

    ///GET
    pub const GET : Method = Method(GET);

    ///POST
    pub const POST : Method = Method(POST);

    ///PUT
    pub const PUT : Method = Method(PUT);

    ///PATCH
    pub const PATCH : Method = Method(PATCH);

    ///OPTIONS
    pub const OPTIONS : Method = Method(OPTIONS);

    ///DELETE
    pub const DELETE : Method = Method(DELETE);

    ///HEAD
    pub const HEAD : Method = Method(HEAD);

    ///CONNECT
    pub const CONNECT : Method = Method(CONNECT);

    ///TRACE
    pub const TRACE : Method = Method(TRACE);
}