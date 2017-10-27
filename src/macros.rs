#[macro_export]
macro_rules! hand {
    ($name:ident, $body:block) => {
        pub fn $name(mut context: &mut $crate::sincere::Context) {
            let result = {

                let h = $body;

                h(&mut context)
            };

            match result {
                Ok(result) => {
                    context.response.from_json(result).unwrap();
                },
                Err(err) => {
                    context.response.from_json(Response::<Empty>::error(err)).unwrap();
                }
            }
        }
    }
}
