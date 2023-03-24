use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident, Result, Token,
};

pub struct Args(pub Vec<Ident>);

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let vars = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(Args(vars.into_iter().collect()))
    }
}
