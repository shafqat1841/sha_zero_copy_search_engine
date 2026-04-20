use zero_copy_search_engine::run;

fn main() {
    if let Err(err) = run() {
        println!("file: main.rs ~ line 5 ~ ifletErr ~ err : {:?} ", err);
    };
}
