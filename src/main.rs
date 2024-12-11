use wasmedge_sdk::wasi::WasiModule;

fn main() {
    let mut _wasi = WasiModule::create(None, None, Some(vec!["/:/"])).unwrap();
    println!("Hello, world!");
}