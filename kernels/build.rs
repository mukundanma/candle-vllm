use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/pagedattention.cu");
    println!("cargo:rerun-if-changed=src/reshape_and_cache_kernel.cu");
    println!("cargo:rerun-if-changed=src/copy_blocks_kernel.cu");

    if let Ok(cuda_path) = env::var("CUDA_HOME") {
        println!("cargo:rustc-link-search=native={}/lib64", cuda_path);
    } else {
        println!("cargo:rustc-link-search=native=/usr/local/cuda/lib64");
    }
    println!("cargo:rustc-link-lib=dylib=cuda");
    println!("cargo:rustc-link-lib=dylib=cudart");
    println!("cargo:rustc-link-lib=dylib=cublas");
    println!("cargo:rustc-link-lib=dylib=curand");

    let compute_cap = env::var("CUDA_COMPUTE_CAP").unwrap_or("80".to_string());

    cc::Build::new()
        .cuda(true)
        .flag("-gencode")
        .flag(&format!("arch=compute_{},code=sm_{}", compute_cap, compute_cap))
        .file("src/pagedattention.cu")
        .file("src/reshape_and_cache_kernel.cu")
        .file("src/copy_blocks_kernel.cu")
        .compile("libpagedattention.a");
}
