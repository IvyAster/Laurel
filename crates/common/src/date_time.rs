use chrono::{DateTime, Local, Utc};



#[macro_export]
macro_rules! datetime_format {
    ($dt: expr) => {
        $dt.format("%Y-%m-%d %H:%M:%S").to_string()
    };
    ($dt: expr, $format: expr) => {
        $dt.format($format).to_string()
    };
}

#[macro_export]
macro_rules! time_format {
    ($dt: expr) => {
        $dt.format("%H:%M:%S").to_string()
    };
    ($dt: expr, $format: expr) => {
        $dt.format($format).to_string()
    };
}

#[macro_export]
macro_rules! date_format {
    ($dt: expr) => {
        $dt.format("%Y-%m-%d").to_string()
    };
    ($dt: expr, $format: expr) => {
        $dt.format($format).to_string()
    };
}

#[macro_export]
macro_rules! local_now {
    ($dt: ident) => {
        $dt::now().naive_local()
    };
}

//#[allow(non_snake_case)]