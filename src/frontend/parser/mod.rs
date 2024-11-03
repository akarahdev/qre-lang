pub mod ast;
pub mod structs;
pub mod core;
pub mod headers;
mod stmts;
mod exprs;
mod helpers;

#[macro_export]
macro_rules! match_token_type {
    (in $self:expr, let $name:ident: $ty:expr => $token_type:pat) => {
        let Some($name) = $self.tokens.next().clone() else {
            $self.errors.push((
                "expected OpenParen, found EOF".to_string(),
                $self.tokens.vector.last().unwrap().clone().span,
            ));
            return None;
        };
        let $token_type = $name.token_type else {
            $self.errors.push((
                format!("expected {:?}, found {:?}", $ty, $name.token_type),
                $self.tokens.vector.last().unwrap().clone().span,
            ));
            return None;
        };
    };
}