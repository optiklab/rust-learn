// In CXX's integration with Cargo, all #include paths begin with a crate name by default
#include "cxx_bidirectional_interoperability/include/blobstore.h"

BlobstoreClient::BlobstoreClient() {}

std::unique_ptr<BlobstoreClient> new_blobstore_client() {
  return std::unique_ptr<BlobstoreClient>(new BlobstoreClient());
}