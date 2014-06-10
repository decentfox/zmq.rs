#![crate_id = "zmq.rs#0.1-pre"]
#![crate_type = "dylib"]
#![license = "MIT"]

pub use ctx::Context;
pub use consts::{SocketType, REQ};
pub use consts::{SocketOption, TYPE};
pub use consts::{HAUSNUMERO, ErrorCode, EINVAL, EPROTONOSUPPORT, ECONNREFUSED};
pub use socket_base::SocketBase;
pub use result::{ZmqResult, ZmqError};

mod ctx;
mod consts;
mod socket_base;
mod req;
mod result;
mod tcp_listener;
mod endpoint;


#[cfg(test)]
mod test {
    #[test]
    fn test_socket_type() {
        assert_eq!(super::REQ as int, 3);
    }

    #[test]
    fn test_socket_create() {
        let c = super::Context::new();
        let s = c.socket(super::REQ);
        assert_eq!(s.getsockopt(super::TYPE), super::REQ as int);
        match s.get_type() {
            super::REQ => (),
            //_ => assert!(false),
        }
    }

    #[test]
    fn test_socket_bind() {
        let c = super::Context::new();
        let s = c.socket(super::REQ);
        assert_eq!(s.bind("").unwrap_err().code, super::EINVAL);
        assert_eq!(s.bind("://127").unwrap_err().code, super::EINVAL);
        assert_eq!(s.bind("tcp://").unwrap_err().code, super::EINVAL);
        assert_eq!(s.bind("tcpp://127.0.0.1:12345").unwrap_err().code, super::EPROTONOSUPPORT);
        assert_eq!(s.bind("tcp://10.0.1.255:12345").unwrap_err().code, super::ECONNREFUSED);
        assert_eq!(s.bind("tcp://10.0.1.1:12z45").unwrap_err().code, super::EINVAL);
    }
}
