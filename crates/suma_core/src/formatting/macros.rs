#[macro_export]
macro_rules! impl_exportable {
    ($type: ty, $object_name:expr,
    simple {$($simple_field:ident), * $(,)?},
    composite {$($composite_field:ident), * $(,)?}) => {
        impl Exportable for $type {
            fn export_with(&self, exporter : &mut dyn Exporter){
                exporter.begin_object($object_name);
                $(
                exporter.write_field(stringify!($simple_field), &format!("{}", &self.$simple_field));
                )*

                $(
                exporter.begin_array(stringify!($composite_field));
                for item in &self.$composite_field {
                    item.export_with(exporter);
                }
                exporter.end_array();
                )*
                exporter.end_object();
            }
        }
    };
}
