use crate::util::point::Point;

use super::{
    fault_zone_block::FaultZoneBlock, fault_zone_point_square::FaultZonePointSquare,
    fault_zone_strip::FaultZoneStrip,
};

macro_rules! fault_zone_enum {
    ($($variant:ident($inner:tt)),*) => {
        pub enum FaultZone<'this> {
            $(
                $variant($inner<'this>),
            )*
        }

        impl<'this> FaultZone<'this> {
            pub fn find_target(&self, p: &Point) -> bool {
                match self {
                    $(
                        FaultZone::$variant(inner) => inner.find_target(p),
                    )*
                }
            }

            pub fn get_theta(&self) -> f64 {
                match self {
                    $(
                        FaultZone::$variant(inner) => inner.get_theta(),
                    )*
                }
            }
        }
    };
}

fault_zone_enum!(
    Block(FaultZoneBlock),
    Strip(FaultZoneStrip),
    PointSquare(FaultZonePointSquare)
);

// macro_rules! enum_with_impl {
//     (
//         pub enum $enum_name:ident 
//             $($variant:ident($inner:tt)),* 
//         impl 
//             $($enum_method:ident($($enum_method_arg:ident: $enum_method_arg_type:ty),*) -> $enum_method_return:ty);*
//     ) => {
//         pub enum $enum_name<'this> {
//             $(
//                 $variant($inner<'this>),
//             )*
//         }

//         impl<'this> $enum_name<'this> {
//             $(
//                 pub fn $enum_method(&self, $($enum_method_arg: $enum_method_arg_type),*) -> $enum_method_return {
//                     match self {
//                         $(
//                             $enum_name::$variant(inner) => inner.$enum_method($($enum_method_arg),*),
//                         )*
//                     }
//                 }
//             )*
//         }
//     };
// }

// enum_with_impl!(
//     pub enum FaultZone 
//         Block(FaultZoneBlock),
//         Strip(FaultZoneStrip),
//         PointSquare(FaultZonePointSquare)
//     impl 
//         find_target(p: &Point) -> bool;
//         get_theta() -> f64
// );

