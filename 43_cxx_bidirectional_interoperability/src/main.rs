
#[cxx::bridge]
mod ffi {
    // A shared struct BlobMetadata to pass metadata about blobs between our 
    // Rust application and C++ blobstore client.
    // Shared structs are data structures whose complete definition is visible 
    // to both languages, making it possible to pass them by value 
    // across the language boundary. Shared structs translate 
    // to a C++ aggregate-initialization compatible struct exactly matching
    // the layout of the Rust one.
    struct BlobMetadata {
        size: usize,
        tags: Vec<String>,
    }

    extern "Rust" {
        type MultiBuf;

        fn next_chunk(buf: &mut MultiBuf) -> &[u8];
    }

    unsafe extern "C++" {

        // In CXX's integration with Cargo, all #include paths begin with a crate name by default
        include!("cxx_bidirectional_interoperability/include/blobstore.h");

        type BlobstoreClient; // Opaque type

        fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
            // UniquePtr is a Rust binding of C++ std::unique_ptr

        // This is a non-static member of BlobstoreClient.
        // If there would be more than one type defined in this bridge,
        // you can disambiguate which one a method belongs to by writing:
        // self: &BlobstoreClient
        fn put(&self, parts: &mut MultiBuf) -> u64;

        fn tag(&self, blobid: u64, tag: &str);
        fn metadata(&self, blobid: u64) -> BlobMetadata;
    }
}

fn main() {
    let client = ffi::new_blobstore_client();

    // Upload a blob.
    let chunks = vec![b"fearless".to_vec(), b"concurrency".to_vec()];
    let mut buf = MultiBuf { chunks, pos: 0 };
    let blobid = client.put(&mut buf);
    println!("blobid = {blobid}");

    // Add a tag.
    client.tag(blobid, "rust");

    // Read back the tags.
    let metadata = client.metadata(blobid);
    println!("tags = {:?}", metadata.tags);
}

// An iterator over contiguous chunks of a discontiguous file object. Toy
// implementation uses a Vec<Vec<u8>> but in reality this might be iterating
// over some more complex Rust data structure like a rope, or maybe loading
// chunks lazily from somewhere.
pub struct MultiBuf {
    chunks: Vec<Vec<u8>>,
    pos: usize,
}

pub fn next_chunk(buf: &mut MultiBuf) -> &[u8] {
    let next = buf.chunks.get(buf.pos);
    buf.pos += 1;
    next.map_or(&[], Vec::as_slice)
}