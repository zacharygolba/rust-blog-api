macro_rules! resource {
    ( $item:ident, | &$this:ident | { $( attr $name:ident () -> $type:ty $getter:block )* } ) => (
        impl IntoResource for $item {
            fn into_resource(&$this) -> Resource {
                let mut res = HashMap::new();

                $(
                    res.insert(
                        stringify!($name).to_owned(),
                        $this.$name.clone(),
                    );
                )*

                Resource {
                    id: $this.id.to_string(),
                    kind: stringify!($item).to_owned().to_lowercase(),
                    attributes: res,
                }
            }
        }
    );
}
