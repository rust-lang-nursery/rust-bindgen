// bindgen-flags: -- -std=c++14

template <typename T>
class IndirectlyUsesTemplateParameter {
public:
    using Aliased = T;

    Aliased aliased;
};
