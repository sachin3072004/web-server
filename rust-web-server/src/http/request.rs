use std::str::Utf8Error;
use std::convert::TryFrom;
use super::method::Method;
use std::error::Error;
use std::fmt::{Result as FmtResult, Display, Formatter,Debug};
use std::str;
pub struct Request{
    pub path: String,
    pub query: Option<String>,
    pub method: Method,
}
impl TryFrom<&[u8]> for Request {
    type Error = ParseError;
    fn try_from(buf:&[u8]) -> Result<Self,Self::Error>{
        str::from_utf8(buf)?;
            unimplemented!()
    }
}


pub enum ParseError{
    InvalidRequest,
    InvalidEncoding,
    InvalidMethod,
    InvalidProtocol,
}

impl ParseError{
    fn message(&self) ->&str{
            match self{
                Self::InvalidRequest=>"Invalid Request",
                Self::InvalidEncoding=>"Invalid Encoding",
                Self::InvalidMethod=>"Invalid Method",
                Self::InvalidProtocol=>"Invalid Protocol",
            }
    }

}

impl From<Utf8Error> for ParseError{
    fn from(_:Utf8Error)->Self{
        Self::InvalidEncoding
    }
}

impl Display for ParseError{
    fn fmt(&self, f:&mut Formatter)->FmtResult{
        write!(f,"{}",self.message())
    }
}


impl Debug for ParseError{
    fn fmt(&self, f:&mut Formatter)->FmtResult{
        write!(f,"{}",self.message())
    }
}

impl Error for ParseError{}
