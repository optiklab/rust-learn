
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {

        // In CXX's integration with Cargo, all #include paths begin with a crate name by default
        include!("cxx_bidirectional_interoperability/include/blobstore.h");

        type BlobstoreClient; // Opaque type

        fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
            // UniquePtr is a Rust binding of C++ std::unique_ptr
    }
}

fn main() {
    let client = ffi::new_blobstore_client();
}