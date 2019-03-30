use iproute::*;
use std::env;
use std::net::Ipv4Addr;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("please input gateway and dev")
    }

    let gateway = args[1]
        .parse::<Ipv4Addr>()
        .expect("invalid gateway ip address");
    let dev: String = args[2].clone();

    let old_routes = get_default_routes().expect("get default routes failed");
    println!("list default routes now:");
    for route in &old_routes {
        println!("{:?}", route);
    }

    let new_route = DefaultRoute::new(gateway, dev);
    println!("add new default route {:?}", new_route);

    // del old default routes
    for route in old_routes {
        del_default_route(route).expect("del default route failed");
    }

    // add new default route
    add_default_route(new_route).expect("add new default route failed");

    let new_routes = get_default_routes().expect("get default routes after add new route failed");
    println!("now, the new default routes:");
    for route in new_routes {
        println!("{:?}", route);
    }
}
