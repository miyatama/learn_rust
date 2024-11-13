pub trait IsSvcA {
    fn a(&self) -> String;
}

pub trait SvcA {}

impl<T: SvcA> IsSvcA for T {
    fn a(&self) -> String {
        "svc-a".to_owned()
    }
}

// Provide A service.
pub trait HaveSvcA {
    type A: IsSvcA; // Not SvcA
    fn get_svc_a(&self) -> &Self::A;
}

pub trait IsSvcB {
    fn b(&self) -> String;
}

// SvcB depends on HaveSvcA instead of IsSvcA.
pub trait SvcB: HaveSvcA {}

impl<T: SvcB> IsSvcB for T {
    fn b(&self) -> String {
        let a = self.get_svc_a();
        format!("a: {}, b: {}", a.a(), "svc-b")
    }
}

// Provide B service.
pub trait HaveSvcB {
    type B: IsSvcB; // Not SvcB
    fn get_svc_b(&self) -> &Self::B;
}

pub fn use_b<S: HaveSvcB>(svc: S) -> String {
    let b = svc.get_svc_b();
    format!("[use] {}", b.b())
}

#[cfg(test)]
mod tests {
    use super::*;
}
