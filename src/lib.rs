/// If variable is equal with the given
/// parameter returns true, otherwise false.
pub trait Is<T> {
    fn is(&self, is: T) -> bool;
}

/// If variable value is exactly the given
/// parameter, returns true, or false.
pub trait IfEq<T> {
    fn if_eq(&self, is: T) -> bool;
}

pub trait Then<R, F> {
    fn then(&self, f: F) -> R
    where
        F: Fn() -> R;
}

pub trait ThenPrint {
    fn then_print(&self, msg: &str) -> String;
}

// Generics implementation
impl<T> Is<T> for T
where
    T: std::cmp::PartialEq,
{
    fn is(&self, is: T) -> bool {
        *self == is
    }
}

// Generics implementation
impl<T> IfEq<T> for T
where
    T: std::cmp::PartialEq,
{
    fn if_eq(&self, is: T) -> bool {
        *self == is
    }
}

// Generics implementation
impl ThenPrint for bool {
    fn then_print(&self, msg: &str) -> String {
        if *self {
            return msg.to_owned();
        }
        msg.to_owned()
    }
}

impl<R, F> Then<R, F> for bool {
    fn then(&self, f: F) -> R
    where
        F: Fn() -> R,
    {
        f()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(9.if_eq(9).then_print("It works"), "It works".to_owned());
        assert_eq!(9.if_eq(9).then(|| "It works"), "It works".to_owned());
        assert_eq!(9.if_eq(9).then(|| (0..100).collect::<Vec<_>>().len()), 100);
        assert_eq!("alma".if_eq("alma").then(|| "ok"), "ok");
        assert_ne!("alma".if_eq("alma").then(|| "ok"), "_ok");
    }
}
