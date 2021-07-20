// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq

namespace detail {
template<typename T>
struct PointerType {
  typedef T* Type;
};
}
template<typename T>
class UniquePtr {
public:
  typedef typename detail::PointerType<T> Pointer;
};
