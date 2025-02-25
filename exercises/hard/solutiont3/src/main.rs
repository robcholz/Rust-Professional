// I AM NOT DONE

mod district;
mod json;

fn main() {
    let provinces = district::count_provinces();
    println!("provinces: {provinces}");
}
