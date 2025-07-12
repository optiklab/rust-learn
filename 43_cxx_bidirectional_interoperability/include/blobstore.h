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
  class Implementation;
  std::shared_ptr<Implementation> pimpl; // unique_ptr actually preferred by Google style guide
};

std::unique_ptr<BlobstoreClient> new_blobstore_client();