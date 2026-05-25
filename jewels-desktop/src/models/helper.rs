#[macro_export]
macro_rules! with_model {
    ($ptr:expr, |$pin:ident| $body:expr) => {
        if let Some(inner) = unsafe { $ptr.as_mut() } {
            let mut $pin = unsafe { Pin::new_unchecked(inner) };
            $body
        }
    };
}

#[macro_export]
macro_rules! with_model_bool {
    ($ptr:expr, |$pin:ident| $body:expr) => {
        if let Some(inner) = unsafe { $ptr.as_mut() } {
            let $pin = unsafe { Pin::new_unchecked(inner) };
            $body
        } else {
            false
        }
    };
}
