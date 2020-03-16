#[cfg(test)]
mod tests {
    use quote::quote;
    #[test]
    fn quote() {
        let function = quote! { fn function() {} };
    }
}
