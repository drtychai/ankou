pub trait TypeInfo {
    fn type_name() -> String;
    fn type_of(&self) -> String;
}

#[allow(unused_macros)]
macro_rules! impl_type_info {
    ($($name:ident$(<$($T:ident),+>)*),*) => {
        $(impl_type_info_single!($name$(<$($T),*>)*);)*
    };
}

#[allow(unused_macros)]
macro_rules! mut_if {
    ($name:ident = $value:expr, $($any:expr)+) => {
        let mut $name = $value;
    };
    ($name:ident = $value:expr,) => {
        let $name = $value;
    };
}

#[allow(unused_macros)]
macro_rules! impl_type_info_single {
    ($name:ident$(<$($T:ident),+>)*) => {
        impl$(<$($T: TypeInfo),*>)* TypeInfo for $name$(<$($T),*>)* {
            fn type_name() -> String {
                mut_if!(res = String::from(stringify!($name)), $($($T)*)*);
                $(
                    res.push('<');
                    $(
                        res.push_str(&$T::type_name());
                        res.push(',');
                    )*
                    res.pop();
                    res.push('>');
                )*
                res
            }
            fn type_of(&self) -> String {
                $name$(::<$($T),*>)*::type_name()
            }
        }
    }
}

impl<'a, T: TypeInfo + ?Sized> TypeInfo for &'a T {
    fn type_name() -> String {
        let mut res = String::from("&");
        res.push_str(&T::type_name());
        res
    }
    fn type_of(&self) -> String {
        <&T>::type_name()
    }
}

impl<'a, T: TypeInfo + ?Sized> TypeInfo for &'a mut T {
    fn type_name() -> String {
        let mut res = String::from("&mut ");
        res.push_str(&T::type_name());
        res
    }
    fn type_of(&self) -> String {
        <&mut T>::type_name()
    }
}

#[allow(unused_macros)]
macro_rules! type_of {
    ($x:expr) => {
        (&$x).type_of()
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::git_log::Repository;

    impl_type_info!(i32, i64, f32, f64, str, String, Vec<T>, Result<T,S>, Repository);

    #[test]
    fn i32() {
        assert_eq!(type_of!(1 as i32), type_of!("1".parse::<i32>().unwrap()));
    }

    #[test]
    fn i64() {
        assert_eq!(type_of!(1 as i64), type_of!("1".parse::<i64>().unwrap()));
    }

    #[test]
    fn f32() {
        assert_eq!(type_of!(1 as f32), type_of!("1".parse::<f32>().unwrap()));
    }

    #[test]
    fn f64() {
        assert_eq!(type_of!(1 as f64), type_of!("1".parse::<f64>().unwrap()));
    }

    #[test]
    fn str() {
        assert_eq!(type_of!("1"), type_of!(String::from("1").as_str()));
    }

    #[test]
    fn string() {
        assert_eq!(type_of!("1".to_string()), type_of!(String::from("1")));
    }

    #[test]
    fn vec_generic() {
        assert_eq!(type_of!(vec![1]), type_of!(Vec::new() as Vec<i32>));
    }

    //#[test]
    //fn result_generic() {
    //    assert_eq!(type_of!(), type_of!("1".parse::<Result<T,S>>().unwrap()));
    //}

    //#[test]
    //fn git_repo() {
    //    assert_eq!(type_of!(1 as i32), type_of!("1".parse::<Repository>().unwrap()));
    //}
}
