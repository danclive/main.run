#[macro_export]
macro_rules! hand {
    ($name:ident, $body:block) => {
        pub fn $name(mut context: &mut $crate::sincere::app::context::Context) {
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

#[macro_export]
macro_rules! model {
    ($struct_name:ident, $table_name:expr) => (
        impl StructDocument for $struct_name {
            const NAME: &'static str = $table_name;

            fn get_database() -> Database {
                DB.clone()
            }
        }
    )
}
