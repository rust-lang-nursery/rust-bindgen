
template<typename T>
class Foo {
public:
  typedef T elem_type;
  typedef T* ptr_type;

  typedef struct Bar {
    int x, y;
  } Bar;
};
