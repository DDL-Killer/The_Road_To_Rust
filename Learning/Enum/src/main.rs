fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    rount(four);
    rount(six);
}

fn rount(ip_type:IpAddrKind){

}
enum IpAddrKind{
    V4,
    V6,
}