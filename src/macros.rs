
// Request::ListAccountsRequest => RequestBuilder::api("/v3/accounts",Method::GET,APP_KEY)

//
// #[macro_export]
// macro_rules! struct_map {
//     (
//         $(#[$meta:meta])*
//         struct $name:ident {
//             $(
//             $(#[$field_meta:meta])*
//             $field_vis:vis $field_name:ident: $field_type:ty,
//             ),*
//         }
//     ) => {
//         $(#[$meta])*
//         struct $name {
//             $(
//             $(#[$field_meta])*
//             $field_vis $field_name: $field_type,
//             )*
//         }
//
//         impl $name {
//             fn build_request_with_header(&self, mut builder: warp::http::request::Builder) -> warp::http::request::Builder {
//                 $(
//                 if let Some(value) = self.$field_name.clone() {
//                     builder = builder.header(get_meta_key!(stringify!($($field_meta),*), "rename"), value)
//                 }
//                 )*
//                 builder
//             }
//         }
//     };
// }
// macro_rules! get_meta_key {
//     ($($value:expr, $key:expr)*) => {
//         {
//             let line = $value;
//             let start_bytes = line.find("(").unwrap_or(0) + 1;
//             let end_bytes = line.find(")").unwrap_or(line.len());
//             let result = &line[start_bytes..end_bytes];
//             for part in result.split(',') {
//                 if part.starts_with($key)
//                 {
//                     for part in part.split('=') {
//                         if part.trim() != $key
//                         {
//                             part.trim().replace("\"", "")
//                         }
//                     }
//                 }
//             }
//         }
//     };
// }
