pub trait UsesSvcA {
    fn a(&self) -> String;
}

pub trait SvcA {}

impl<T: SvcA> UsesSvcA for T {
    fn a(&self) -> String {
        "svc-a".to_owned()
    }
}

// Provide A service.
pub trait ProvidesSvcA {
    type A: UsesSvcA; // Not SvcA
    fn get_svc_a(&self) -> &Self::A;
}

pub trait UsesSvcB {
    fn b(&self) -> String;
}

// SvcB depends on ProvidesSvcA instead of UsesSvcA.
pub trait SvcB: ProvidesSvcA {}

impl<T: SvcB> UsesSvcB for T {
    fn b(&self) -> String {
        let a = self.get_svc_a();
        format!("a: {}, b: {}", a.a(), "svc-b")
    }
}

// Provide B service.
pub trait ProvidesSvcB {
    type B: UsesSvcB; // Not SvcB
    fn get_svc_b(&self) -> Self::B;
}

pub fn use_b<S: ProvidesSvcB>(svc: S) -> String {
    let b = svc.get_svc_b();
    format!("[use] {}", b.b())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_use_b() {
        struct MockUses {};
        impl UsesSvcB for MockUses {
            fn b(&self) -> String {
                "test".to_string()
            }
        }

        struct MockProvides {};
        impl ProvidesSvcB for MockProvides {
            type B = MockUses;
            fn get_svc_b(&self) -> Self::B {
                MockUses {}
            }
        }
        let mock = MockProvides {};
        let actual = use_b(mock);
        let expect = "[use] test".to_string();
        assert_eq!(actual, expect);
    }
}
