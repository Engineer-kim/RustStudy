mod IpAddresskind;

fn main() {
    let four = IpAddresskind::V4;
    let six = IpAddresskind::V6;

    let home = IpAddress {
        kind: four,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddress {
        kind: six,
        address: String::from("::1"),
    };
}

struct IpAddress {
    kind: IpAddrKind,
    address: String,
}
