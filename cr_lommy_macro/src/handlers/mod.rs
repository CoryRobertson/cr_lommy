use syn::Ident;

pub(crate) mod all_args_constructor;
pub(crate) mod enum_variant_list;
pub(crate) mod getters;
pub(crate) mod setters;
pub(crate) mod specific_getters;
pub(crate) mod specific_setters;

fn is_ident_present_in_attr<I>(attrs: &[syn::Attribute], ident: &I) -> bool
where
    I: ?Sized,
    Ident: PartialEq<I>,
{
    attrs
        .iter()
        .find(|attr| attr.path().is_ident(ident))
        .is_some()
}
