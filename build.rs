fn main(){
    println!("cargo:rustc-link-arg=-shared");
    println!("cargo:rustc-link-arg=-undefined");
    //println!("cargo:rustc-link-arg=dynamic_lookup");
}
