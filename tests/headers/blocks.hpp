// bindgen-flags: --block-extern-crate -- -fblocks
// bindgen-osx-only

typedef unsigned long long size_t;

void atexit_b(void (^)(void));

typedef void *dispatch_data_t;

typedef bool (^dispatch_data_applier_t)(dispatch_data_t region,
                                        size_t offset,
                                        const void *buffer,
                                        size_t size);

bool dispatch_data_apply(dispatch_data_t data,
                         dispatch_data_applier_t applier);