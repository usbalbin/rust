#![feature(const_try)]
#![feature(const_identity_convert)]

fn main() {
    const fn foo() -> Result<bool, ()> {
        Err(())?;
        Ok(true)
    }

    const FOO: Result<bool, ()> = foo();
    assert_eq!(Err(()), FOO);
}
