extern crate click_n_load2;

fn print_package(package: click_n_load2::Package) {
    println!("{:?}", package)
}

fn main() {
    click_n_load2::listen(print_package);
}
