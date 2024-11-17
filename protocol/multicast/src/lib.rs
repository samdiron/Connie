pub mod cast;


#[macro_use]
extern crate lazy_static;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

lazy_static! {
    pub static ref IPV4: IpAddr = Ipv4Addr::new(224, 0, 0, 123).into();
    pub static ref IPV6: IpAddr = Ipv6Addr::new(0xFF02, 0, 0, 0, 0, 0, 0, 0x0123).into();
}


#[cfg(test)]
mod test {
    use crate::IPV4;
    fn ipv4_multicast_test() 
    {
        assert!(IPV4.is_multicast()); 
    }

    use crate::IPV6;
    fn ipv6_multicast_test() 
    {
        assert!(IPV6.is_multicast())
    }

}










// #[cfg(test)]
// mod tests {
//
//     #[test]
//     fn it_works() {
//      
//     }
// }
