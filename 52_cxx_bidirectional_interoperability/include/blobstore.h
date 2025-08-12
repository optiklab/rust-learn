#pragma once
#include <memory>
#include "rust/cxx.h"

struct MultiBuf;
struct BlobMetadata;

class BlobstoreClient {
public:
  BlobstoreClient();
  uint64_t put(MultiBuf &buf) const;
  void tag(uint64_t blobid, rust::Str tag) const;
  BlobMetadata metadata(uint64_t blobid) const;

private:
  // Below, we are using a common C++ idiom called the 
  // "Pimpl" (Pointer to Implementation) pattern. It hides the implementation 
  // details of BlobstoreClient from users of the header, improving 
  // encapsulation and reducing compile-time dependencies.

  class Implementation; // A forward declaration of a nested class

  // A smart pointer to an instance of that Implementation class
  // However, unique_ptr actually preferred by Google style guide.
  std::shared_ptr<Implementation> pimpl; 
};


/////////////////// This is a C++ code called from Rust ////////////////////////
std::unique_ptr<BlobstoreClient> new_blobstore_client();