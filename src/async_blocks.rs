#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        struct MyError;

        async fn foo() -> Result<(), MyError> {
            Ok(())
        }

        async fn bar() -> Result<(), MyError> {
            Ok(())
        }

        let _fut = async {
            foo().await?;
            bar().await?;
            Ok::<(), MyError>(()) // <- note the explicit type annotation here
        };
    }
}
