pub fn f1() {}

pub fn f2() {}

#[cfg(test)]
mod tests {
    use feature_str_from_raw_parts_util::str_from_raw_parts;

    #[test]
    fn should_ok() {
        let x = unsafe { str_from_raw_parts("foobar".as_ptr(), 3) };
        let _ = x;
    }
}
