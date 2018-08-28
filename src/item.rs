// Copyright 2018 Syn Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::*;
use derive::{Data, DeriveInput};
use proc_macro2::TokenStream;
use punctuated::Punctuated;
use token::{Brace, Paren};

#[cfg(feature = "extra-traits")]
use std::hash::{Hash, Hasher};
#[cfg(feature = "extra-traits")]
use tt::TokenStreamHelper;

ast_enum_of_structs! {
    /// Things that can appear directly inside of a module or scope.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    ///
    /// # Syntax tree enum
    ///
    /// This type is a [syntax tree enum].
    ///
    /// [syntax tree enum]: enum.Expr.html#syntax-tree-enums
    pub enum Item {
        /// An `extern crate` item: `extern crate serde`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub ExternCrate(ItemExternCrate {
            pub attrs: Vec<Attribute>,
            pub vis: Visibility,
            pub extern_token: Token![extern],
            pub crate_token: Token![crate],
            pub ident: Ident,
            pub rename: Option<(Token![as], Ident)>,
            pub semi_token: Token![;],
        }),

        /// A use declaration: `use std::collections::HashMap`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Use(ItemUse {
            pub attrs: Vec<Attribute>,
            pub vis: Visibility,
            pub use_token: Token![use],
            pub leading_colon: Option<Token![::]>,
            pub tree: UseTree,
            pub semi_token: Token![;],
        }),

        /// A static item: `static BIKE: Shed = Shed(42)`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Static(ItemStatic {
            pub attrs: Vec<Attribute>,
            pub vis: Visibility,
            pub static_token: Token![static],
            pub mutability: Option<Token![mut]>,
            pub ident: Ident,
            pub colon_token: Token![:],
            pub ty: Box<Type>,
            pub eq_token: Token![=],
            pub expr: Box<Expr>,
            pub semi_token: Token![;],
        }),

        /// A constant item: `const MAX: u16 = 65535`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Const(ItemConst {
            pub attrs: Vec<Attribute>,
            pub vis: Visibility,
            pub const_token: Token![const],
            pub ident: Ident,
            pub colon_token: Token![:],
            pub ty: Box<Type>,
            pub eq_token: Token![=],
            pub expr: Box<Expr>,
            pub semi_token: Token![;],
        }),

        /// A free-standing function: `fn process(n: usize) -> Result<()> { ...
        /// }`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Fn(ItemFn {
            pub attrs: Vec<Attribute>,
            pub vis: Visibility,
            pub constness: Option<Token![const]>,
            pub unsafety: Option<Token![unsafe]>,
            pub asyncness: Option<Token![async]>,
            pub abi: Option<Abi>,
            pub ident: Ident,
            pub decl: Box<FnDecl>,
            pub block: Box<Block>,
        }),

        /// A module or module declaration: `mod m` or `mod m { ... }`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Mod(ItemMod {
            pub attrs: Vec<Attribute>,
            pub vis: Visibility,
            pub mod_token: Token![mod],
            pub ident: Ident,
            pub content: Option<(token::Brace, Vec<Item>)>,
            pub semi: Option<Token![;]>,
        }),

        /// A block of foreign items: `extern "C" { ... }`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub ForeignMod(ItemForeignMod {
            pub attrs: Vec<Attribute>,
            pub abi: Abi,
            pub brace_token: token::Brace,
            pub items: Vec<ForeignItem>,
        }),

        /// A type alias: `type Result<T> = std::result::Result<T, MyError>`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Type(ItemType {
            pub attrs: Vec<Attribute>,
            pub vis: Visibility,
            pub type_token: Token![type],
            pub ident: Ident,
            pub generics: Generics,
            pub eq_token: Token![=],
            pub ty: Box<Type>,
            pub semi_token: Token![;],
        }),

        /// An existential type: `existential type Iter: Iterator<Item = u8>`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Existential(ItemExistential {
            pub attrs: Vec<Attribute>,
            pub vis: Visibility,
            pub existential_token: Token![existential],
            pub type_token: Token![type],
            pub ident: Ident,
            pub generics: Generics,
            pub colon_token: Option<Token![:]>,
            pub bounds: Punctuated<TypeParamBound, Token![+]>,
            pub semi_token: Token![;],
        }),

        /// A struct definition: `struct Foo<A> { x: A }`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Struct(ItemStruct {
            pub attrs: Vec<Attribute>,
            pub vis: Visibility,
            pub struct_token: Token![struct],
            pub ident: Ident,
            pub generics: Generics,
            pub fields: Fields,
            pub semi_token: Option<Token![;]>,
        }),

        /// An enum definition: `enum Foo<A, B> { C<A>, D<B> }`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Enum(ItemEnum {
            pub attrs: Vec<Attribute>,
            pub vis: Visibility,
            pub enum_token: Token![enum],
            pub ident: Ident,
            pub generics: Generics,
            pub brace_token: token::Brace,
            pub variants: Punctuated<Variant, Token![,]>,
        }),

        /// A union definition: `union Foo<A, B> { x: A, y: B }`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Union(ItemUnion {
            pub attrs: Vec<Attribute>,
            pub vis: Visibility,
            pub union_token: Token![union],
            pub ident: Ident,
            pub generics: Generics,
            pub fields: FieldsNamed,
        }),

        /// A trait definition: `pub trait Iterator { ... }`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Trait(ItemTrait {
            pub attrs: Vec<Attribute>,
            pub vis: Visibility,
            pub unsafety: Option<Token![unsafe]>,
            pub auto_token: Option<Token![auto]>,
            pub trait_token: Token![trait],
            pub ident: Ident,
            pub generics: Generics,
            pub colon_token: Option<Token![:]>,
            pub supertraits: Punctuated<TypeParamBound, Token![+]>,
            pub brace_token: token::Brace,
            pub items: Vec<TraitItem>,
        }),

        /// An impl block providing trait or associated items: `impl<A> Trait
        /// for Data<A> { ... }`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Impl(ItemImpl {
            pub attrs: Vec<Attribute>,
            pub defaultness: Option<Token![default]>,
            pub unsafety: Option<Token![unsafe]>,
            pub impl_token: Token![impl],
            pub generics: Generics,
            /// Trait this impl implements.
            pub trait_: Option<(Option<Token![!]>, Path, Token![for])>,
            /// The Self type of the impl.
            pub self_ty: Box<Type>,
            pub brace_token: token::Brace,
            pub items: Vec<ImplItem>,
        }),

        /// A macro invocation, which includes `macro_rules!` definitions.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Macro(ItemMacro {
            pub attrs: Vec<Attribute>,
            /// The `example` in `macro_rules! example { ... }`.
            pub ident: Option<Ident>,
            pub mac: Macro,
            pub semi_token: Option<Token![;]>,
        }),

        /// A 2.0-style declarative macro introduced by the `macro` keyword.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Macro2(ItemMacro2 #manual_extra_traits {
            pub attrs: Vec<Attribute>,
            pub vis: Visibility,
            pub macro_token: Token![macro],
            pub ident: Ident,
            pub paren_token: Paren,
            pub args: TokenStream,
            pub brace_token: Brace,
            pub body: TokenStream,
        }),

        /// Tokens forming an item not interpreted by Syn.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Verbatim(ItemVerbatim #manual_extra_traits {
            pub tts: TokenStream,
        }),
    }
}

#[cfg(feature = "extra-traits")]
impl Eq for ItemMacro2 {}

#[cfg(feature = "extra-traits")]
impl PartialEq for ItemMacro2 {
    fn eq(&self, other: &Self) -> bool {
        self.attrs == other.attrs
            && self.vis == other.vis
            && self.macro_token == other.macro_token
            && self.ident == other.ident
            && self.paren_token == other.paren_token
            && TokenStreamHelper(&self.args) == TokenStreamHelper(&other.args)
            && self.brace_token == other.brace_token
            && TokenStreamHelper(&self.body) == TokenStreamHelper(&other.body)
    }
}

#[cfg(feature = "extra-traits")]
impl Hash for ItemMacro2 {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.attrs.hash(state);
        self.vis.hash(state);
        self.macro_token.hash(state);
        self.ident.hash(state);
        self.paren_token.hash(state);
        TokenStreamHelper(&self.args).hash(state);
        self.brace_token.hash(state);
        TokenStreamHelper(&self.body).hash(state);
    }
}

#[cfg(feature = "extra-traits")]
impl Eq for ItemVerbatim {}

#[cfg(feature = "extra-traits")]
impl PartialEq for ItemVerbatim {
    fn eq(&self, other: &Self) -> bool {
        TokenStreamHelper(&self.tts) == TokenStreamHelper(&other.tts)
    }
}

#[cfg(feature = "extra-traits")]
impl Hash for ItemVerbatim {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        TokenStreamHelper(&self.tts).hash(state);
    }
}

impl From<DeriveInput> for Item {
    fn from(input: DeriveInput) -> Item {
        match input.data {
            Data::Struct(data) => Item::Struct(ItemStruct {
                attrs: input.attrs,
                vis: input.vis,
                struct_token: data.struct_token,
                ident: input.ident,
                generics: input.generics,
                fields: data.fields,
                semi_token: data.semi_token,
            }),
            Data::Enum(data) => Item::Enum(ItemEnum {
                attrs: input.attrs,
                vis: input.vis,
                enum_token: data.enum_token,
                ident: input.ident,
                generics: input.generics,
                brace_token: data.brace_token,
                variants: data.variants,
            }),
            Data::Union(data) => Item::Union(ItemUnion {
                attrs: input.attrs,
                vis: input.vis,
                union_token: data.union_token,
                ident: input.ident,
                generics: input.generics,
                fields: data.fields,
            }),
        }
    }
}

ast_enum_of_structs! {
    /// A suffix of an import tree in a `use` item: `Type as Renamed` or `*`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    ///
    /// # Syntax tree enum
    ///
    /// This type is a [syntax tree enum].
    ///
    /// [syntax tree enum]: enum.Expr.html#syntax-tree-enums
    pub enum UseTree {
        /// A path prefix of imports in a `use` item: `std::...`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Path(UsePath {
            pub ident: Ident,
            pub colon2_token: Token![::],
            pub tree: Box<UseTree>,
        }),

        /// An identifier imported by a `use` item: `HashMap`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Name(UseName {
            pub ident: Ident,
        }),

        /// An renamed identifier imported by a `use` item: `HashMap as Map`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Rename(UseRename {
            pub ident: Ident,
            pub as_token: Token![as],
            pub rename: Ident,
        }),

        /// A glob import in a `use` item: `*`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Glob(UseGlob {
            pub star_token: Token![*],
        }),

        /// A braced group of imports in a `use` item: `{A, B, C}`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Group(UseGroup {
            pub brace_token: token::Brace,
            pub items: Punctuated<UseTree, Token![,]>,
        }),
    }
}

ast_enum_of_structs! {
    /// An item within an `extern` block.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    ///
    /// # Syntax tree enum
    ///
    /// This type is a [syntax tree enum].
    ///
    /// [syntax tree enum]: enum.Expr.html#syntax-tree-enums
    pub enum ForeignItem {
        /// A foreign function in an `extern` block.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Fn(ForeignItemFn {
            pub attrs: Vec<Attribute>,
            pub vis: Visibility,
            pub ident: Ident,
            pub decl: Box<FnDecl>,
            pub semi_token: Token![;],
        }),

        /// A foreign static item in an `extern` block: `static ext: u8`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Static(ForeignItemStatic {
            pub attrs: Vec<Attribute>,
            pub vis: Visibility,
            pub static_token: Token![static],
            pub mutability: Option<Token![mut]>,
            pub ident: Ident,
            pub colon_token: Token![:],
            pub ty: Box<Type>,
            pub semi_token: Token![;],
        }),

        /// A foreign type in an `extern` block: `type void`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Type(ForeignItemType {
            pub attrs: Vec<Attribute>,
            pub vis: Visibility,
            pub type_token: Token![type],
            pub ident: Ident,
            pub semi_token: Token![;],
        }),

        /// A macro invocation within an extern block.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Macro(ForeignItemMacro {
            pub attrs: Vec<Attribute>,
            pub mac: Macro,
            pub semi_token: Option<Token![;]>,
        }),

        /// Tokens in an `extern` block not interpreted by Syn.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Verbatim(ForeignItemVerbatim #manual_extra_traits {
            pub tts: TokenStream,
        }),
    }
}

#[cfg(feature = "extra-traits")]
impl Eq for ForeignItemVerbatim {}

#[cfg(feature = "extra-traits")]
impl PartialEq for ForeignItemVerbatim {
    fn eq(&self, other: &Self) -> bool {
        TokenStreamHelper(&self.tts) == TokenStreamHelper(&other.tts)
    }
}

#[cfg(feature = "extra-traits")]
impl Hash for ForeignItemVerbatim {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        TokenStreamHelper(&self.tts).hash(state);
    }
}

ast_enum_of_structs! {
    /// An item declaration within the definition of a trait.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    ///
    /// # Syntax tree enum
    ///
    /// This type is a [syntax tree enum].
    ///
    /// [syntax tree enum]: enum.Expr.html#syntax-tree-enums
    pub enum TraitItem {
        /// An associated constant within the definition of a trait.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Const(TraitItemConst {
            pub attrs: Vec<Attribute>,
            pub const_token: Token![const],
            pub ident: Ident,
            pub colon_token: Token![:],
            pub ty: Type,
            pub default: Option<(Token![=], Expr)>,
            pub semi_token: Token![;],
        }),

        /// A trait method within the definition of a trait.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Method(TraitItemMethod {
            pub attrs: Vec<Attribute>,
            pub sig: MethodSig,
            pub default: Option<Block>,
            pub semi_token: Option<Token![;]>,
        }),

        /// An associated type within the definition of a trait.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Type(TraitItemType {
            pub attrs: Vec<Attribute>,
            pub type_token: Token![type],
            pub ident: Ident,
            pub generics: Generics,
            pub colon_token: Option<Token![:]>,
            pub bounds: Punctuated<TypeParamBound, Token![+]>,
            pub default: Option<(Token![=], Type)>,
            pub semi_token: Token![;],
        }),

        /// A macro invocation within the definition of a trait.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Macro(TraitItemMacro {
            pub attrs: Vec<Attribute>,
            pub mac: Macro,
            pub semi_token: Option<Token![;]>,
        }),

        /// Tokens within the definition of a trait not interpreted by Syn.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Verbatim(TraitItemVerbatim #manual_extra_traits {
            pub tts: TokenStream,
        }),
    }
}

#[cfg(feature = "extra-traits")]
impl Eq for TraitItemVerbatim {}

#[cfg(feature = "extra-traits")]
impl PartialEq for TraitItemVerbatim {
    fn eq(&self, other: &Self) -> bool {
        TokenStreamHelper(&self.tts) == TokenStreamHelper(&other.tts)
    }
}

#[cfg(feature = "extra-traits")]
impl Hash for TraitItemVerbatim {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        TokenStreamHelper(&self.tts).hash(state);
    }
}

ast_enum_of_structs! {
    /// An item within an impl block.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    ///
    /// # Syntax tree enum
    ///
    /// This type is a [syntax tree enum].
    ///
    /// [syntax tree enum]: enum.Expr.html#syntax-tree-enums
    pub enum ImplItem {
        /// An associated constant within an impl block.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Const(ImplItemConst {
            pub attrs: Vec<Attribute>,
            pub vis: Visibility,
            pub defaultness: Option<Token![default]>,
            pub const_token: Token![const],
            pub ident: Ident,
            pub colon_token: Token![:],
            pub ty: Type,
            pub eq_token: Token![=],
            pub expr: Expr,
            pub semi_token: Token![;],
        }),

        /// A method within an impl block.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Method(ImplItemMethod {
            pub attrs: Vec<Attribute>,
            pub vis: Visibility,
            pub defaultness: Option<Token![default]>,
            pub sig: MethodSig,
            pub block: Block,
        }),

        /// An associated type within an impl block.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Type(ImplItemType {
            pub attrs: Vec<Attribute>,
            pub vis: Visibility,
            pub defaultness: Option<Token![default]>,
            pub type_token: Token![type],
            pub ident: Ident,
            pub generics: Generics,
            pub eq_token: Token![=],
            pub ty: Type,
            pub semi_token: Token![;],
        }),

        /// An existential type within an impl block.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Existential(ImplItemExistential {
            pub attrs: Vec<Attribute>,
            pub existential_token: Token![existential],
            pub type_token: Token![type],
            pub ident: Ident,
            pub generics: Generics,
            pub colon_token: Option<Token![:]>,
            pub bounds: Punctuated<TypeParamBound, Token![+]>,
            pub semi_token: Token![;],
        }),

        /// A macro invocation within an impl block.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Macro(ImplItemMacro {
            pub attrs: Vec<Attribute>,
            pub mac: Macro,
            pub semi_token: Option<Token![;]>,
        }),

        /// Tokens within an impl block not interpreted by Syn.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Verbatim(ImplItemVerbatim #manual_extra_traits {
            pub tts: TokenStream,
        }),
    }
}

#[cfg(feature = "extra-traits")]
impl Eq for ImplItemVerbatim {}

#[cfg(feature = "extra-traits")]
impl PartialEq for ImplItemVerbatim {
    fn eq(&self, other: &Self) -> bool {
        TokenStreamHelper(&self.tts) == TokenStreamHelper(&other.tts)
    }
}

#[cfg(feature = "extra-traits")]
impl Hash for ImplItemVerbatim {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        TokenStreamHelper(&self.tts).hash(state);
    }
}

ast_struct! {
    /// A method's signature in a trait or implementation: `unsafe fn
    /// initialize(&self)`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct MethodSig {
        pub constness: Option<Token![const]>,
        pub unsafety: Option<Token![unsafe]>,
        pub asyncness: Option<Token![async]>,
        pub abi: Option<Abi>,
        pub ident: Ident,
        pub decl: FnDecl,
    }
}

ast_struct! {
    /// Header of a function declaration, without including the body.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct FnDecl {
        pub fn_token: Token![fn],
        pub generics: Generics,
        pub paren_token: token::Paren,
        pub inputs: Punctuated<FnArg, Token![,]>,
        pub variadic: Option<Token![...]>,
        pub output: ReturnType,
    }
}

ast_enum_of_structs! {
    /// An argument in a function signature: the `n: usize` in `fn f(n: usize)`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    ///
    /// # Syntax tree enum
    ///
    /// This type is a [syntax tree enum].
    ///
    /// [syntax tree enum]: enum.Expr.html#syntax-tree-enums
    pub enum FnArg {
        /// Self captured by reference in a function signature: `&self` or `&mut
        /// self`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub SelfRef(ArgSelfRef {
            pub and_token: Token![&],
            pub lifetime: Option<Lifetime>,
            pub mutability: Option<Token![mut]>,
            pub self_token: Token![self],
        }),

        /// Self captured by value in a function signature: `self` or `mut
        /// self`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub SelfValue(ArgSelf {
            pub mutability: Option<Token![mut]>,
            pub self_token: Token![self],
        }),

        /// An explicitly typed pattern captured by a function signature.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Captured(ArgCaptured {
            pub pat: Pat,
            pub colon_token: Token![:],
            pub ty: Type,
        }),

        /// A pattern whose type is inferred captured by a function signature.
        pub Inferred(Pat),
        /// A type not bound to any pattern in a function signature.
        pub Ignored(Type),
    }
}

#[cfg(feature = "parsing")]
pub mod parsing {
    use super::*;

    use parse::{Parse, ParseStream, Result};
    use synom::ext::IdentExt;

    impl Parse for Item {
        fn parse(input: ParseStream) -> Result<Self> {
            let ahead = input.fork();
            ahead.call(Attribute::parse_outer)?;
            let vis: Visibility = ahead.parse()?;

            let lookahead = ahead.lookahead1();
            if lookahead.peek(Token![extern]) {
                ahead.parse::<Token![extern]>()?;
                let lookahead = ahead.lookahead1();
                if lookahead.peek(Token![crate]) {
                    input.parse().map(Item::ExternCrate)
                } else if lookahead.peek(Token![fn]) {
                    input.parse().map(Item::Fn)
                } else if lookahead.peek(token::Brace) {
                    input.parse().map(Item::ForeignMod)
                } else if lookahead.peek(LitStr) {
                    ahead.parse::<LitStr>()?;
                    let lookahead = ahead.lookahead1();
                    if lookahead.peek(token::Brace) {
                        input.parse().map(Item::ForeignMod)
                    } else if lookahead.peek(Token![fn]) {
                        input.parse().map(Item::Fn)
                    } else {
                        Err(lookahead.error())
                    }
                } else {
                    Err(lookahead.error())
                }
            } else if lookahead.peek(Token![use]) {
                input.parse().map(Item::Use)
            } else if lookahead.peek(Token![static]) {
                input.parse().map(Item::Static)
            } else if lookahead.peek(Token![const]) {
                ahead.parse::<Token![const]>()?;
                let lookahead = ahead.lookahead1();
                if lookahead.peek(Ident) {
                    input.parse().map(Item::Const)
                } else if lookahead.peek(Token![unsafe])
                    || lookahead.peek(Token![async])
                    || lookahead.peek(Token![extern])
                    || lookahead.peek(Token![fn])
                {
                    input.parse().map(Item::Fn)
                } else {
                    Err(lookahead.error())
                }
            } else if lookahead.peek(Token![unsafe]) {
                ahead.parse::<Token![unsafe]>()?;
                let lookahead = ahead.lookahead1();
                if lookahead.peek(Token![trait])
                    || lookahead.peek(Token![auto]) && ahead.peek2(Token![trait])
                {
                    input.parse().map(Item::Trait)
                } else if lookahead.peek(Token![impl ]) {
                    input.parse().map(Item::Impl)
                } else if lookahead.peek(Token![async])
                    || lookahead.peek(Token![extern])
                    || lookahead.peek(Token![fn])
                {
                    input.parse().map(Item::Fn)
                } else {
                    Err(lookahead.error())
                }
            } else if lookahead.peek(Token![async]) || lookahead.peek(Token![fn]) {
                input.parse().map(Item::Fn)
            } else if lookahead.peek(Token![mod]) {
                input.parse().map(Item::Mod)
            } else if lookahead.peek(Token![type]) {
                input.parse().map(Item::Type)
            } else if lookahead.peek(Token![existential]) {
                input.parse().map(Item::Existential)
            } else if lookahead.peek(Token![struct]) {
                input.parse().map(Item::Struct)
            } else if lookahead.peek(Token![enum]) {
                input.parse().map(Item::Enum)
            } else if lookahead.peek(Token![union]) && ahead.peek2(Ident) {
                input.parse().map(Item::Union)
            } else if lookahead.peek(Token![trait])
                || lookahead.peek(Token![auto]) && ahead.peek2(Token![trait])
            {
                input.parse().map(Item::Trait)
            } else if lookahead.peek(Token![impl ])
                || lookahead.peek(Token![default]) && !ahead.peek2(Token![!])
            {
                input.parse().map(Item::Impl)
            } else if lookahead.peek(Token![macro]) {
                input.parse().map(Item::Macro2)
            } else if vis.is_inherited()
                && (lookahead.peek(Ident)
                    || lookahead.peek(Token![self])
                    || lookahead.peek(Token![super])
                    || lookahead.peek(Token![extern])
                    || lookahead.peek(Token![crate])
                    || lookahead.peek(Token![::]))
            {
                input.parse().map(Item::Macro)
            } else {
                Err(lookahead.error())
            }
        }
    }

    impl Parse for ItemMacro {
        fn parse(input: ParseStream) -> Result<Self> {
            let attrs = input.call(Attribute::parse_outer)?;
            let path = input.call(Path::parse_mod_style)?;
            let bang_token: Token![!] = input.parse()?;
            let ident: Option<Ident> = input.parse()?;
            let (delimiter, tts) = input.call(mac::parse_delimiter)?;
            let semi_token: Option<Token![;]> = if !delimiter.is_brace() {
                Some(input.parse()?)
            } else {
                None
            };
            Ok(ItemMacro {
                attrs: attrs,
                ident: ident,
                mac: Macro {
                    path: path,
                    bang_token: bang_token,
                    delimiter: delimiter,
                    tts: tts,
                },
                semi_token: semi_token,
            })
        }
    }

    // TODO: figure out the actual grammar; is body required to be braced?
    impl Parse for ItemMacro2 {
        fn parse(input: ParseStream) -> Result<Self> {
            let args;
            let body;
            Ok(ItemMacro2 {
                attrs: input.call(Attribute::parse_outer)?,
                vis: input.parse()?,
                macro_token: input.parse()?,
                ident: input.parse()?,
                paren_token: parenthesized!(args in input),
                args: args.parse()?,
                brace_token: braced!(body in input),
                body: body.parse()?,
            })
        }
    }

    impl Parse for ItemExternCrate {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(ItemExternCrate {
                attrs: input.call(Attribute::parse_outer)?,
                vis: input.parse()?,
                extern_token: input.parse()?,
                crate_token: input.parse()?,
                ident: input.parse()?,
                rename: {
                    if input.peek(Token![as]) {
                        let as_token: Token![as] = input.parse()?;
                        let rename: Ident = input.parse()?;
                        Some((as_token, rename))
                    } else {
                        None
                    }
                },
                semi_token: input.parse()?,
            })
        }
    }

    impl Parse for ItemUse {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(ItemUse {
                attrs: input.call(Attribute::parse_outer)?,
                vis: input.parse()?,
                use_token: input.parse()?,
                leading_colon: input.parse()?,
                tree: input.call(use_tree)?,
                semi_token: input.parse()?,
            })
        }
    }

    fn use_tree(input: ParseStream) -> Result<UseTree> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident)
            || lookahead.peek(Token![self])
            || lookahead.peek(Token![super])
            || lookahead.peek(Token![crate])
            || lookahead.peek(Token![extern])
        {
            let ident = input.call(Ident::parse_any2)?;
            if input.peek(Token![::]) {
                Ok(UseTree::Path(UsePath {
                    ident: ident,
                    colon2_token: input.parse()?,
                    tree: Box::new(input.call(use_tree)?),
                }))
            } else if input.peek(Token![as]) {
                Ok(UseTree::Rename(UseRename {
                    ident: ident,
                    as_token: input.parse()?,
                    rename: input.parse()?,
                }))
            } else {
                Ok(UseTree::Name(UseName { ident: ident }))
            }
        } else if lookahead.peek(Token![*]) {
            Ok(UseTree::Glob(UseGlob {
                star_token: input.parse()?,
            }))
        } else if lookahead.peek(token::Brace) {
            let content;
            Ok(UseTree::Group(UseGroup {
                brace_token: braced!(content in input),
                items: content.parse_terminated(use_tree)?,
            }))
        } else {
            Err(lookahead.error())
        }
    }

    impl Parse for ItemStatic {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(ItemStatic {
                attrs: input.call(Attribute::parse_outer)?,
                vis: input.parse()?,
                static_token: input.parse()?,
                mutability: input.parse()?,
                ident: input.parse()?,
                colon_token: input.parse()?,
                ty: input.parse()?,
                eq_token: input.parse()?,
                expr: input.parse()?,
                semi_token: input.parse()?,
            })
        }
    }

    impl Parse for ItemConst {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(ItemConst {
                attrs: input.call(Attribute::parse_outer)?,
                vis: input.parse()?,
                const_token: input.parse()?,
                ident: input.parse()?,
                colon_token: input.parse()?,
                ty: input.parse()?,
                eq_token: input.parse()?,
                expr: input.parse()?,
                semi_token: input.parse()?,
            })
        }
    }

    impl Parse for ItemFn {
        fn parse(input: ParseStream) -> Result<Self> {
            let outer_attrs = input.call(Attribute::parse_outer)?;
            let vis: Visibility = input.parse()?;
            let constness: Option<Token![const]> = input.parse()?;
            let unsafety: Option<Token![unsafe]> = input.parse()?;
            let asyncness: Option<Token![async]> = input.parse()?;
            let abi: Option<Abi> = input.parse()?;
            let fn_token: Token![fn] = input.parse()?;
            let ident: Ident = input.parse()?;
            let generics: Generics = input.parse()?;

            let content;
            let paren_token = parenthesized!(content in input);
            let inputs = content.parse_terminated(<FnArg as Parse>::parse)?;

            let output: ReturnType = input.parse()?;
            let where_clause: Option<WhereClause> = input.parse()?;

            let content;
            let brace_token = braced!(content in input);
            let inner_attrs = content.call(Attribute::parse_inner)?;
            let stmts = content.call(Block::parse_within)?;

            Ok(ItemFn {
                attrs: {
                    let mut attrs = outer_attrs;
                    attrs.extend(inner_attrs);
                    attrs
                },
                vis: vis,
                constness: constness,
                unsafety: unsafety,
                asyncness: asyncness,
                abi: abi,
                ident: ident,
                decl: Box::new(FnDecl {
                    fn_token: fn_token,
                    paren_token: paren_token,
                    inputs: inputs,
                    output: output,
                    variadic: None,
                    generics: Generics {
                        where_clause: where_clause,
                        ..generics
                    },
                }),
                block: Box::new(Block {
                    brace_token: brace_token,
                    stmts: stmts,
                }),
            })
        }
    }

    impl Parse for FnArg {
        fn parse(input: ParseStream) -> Result<Self> {
            if input.peek(Token![&]) {
                let ahead = input.fork();
                if ahead.call(arg_self_ref).is_ok() && !ahead.peek(Token![:]) {
                    return input.call(arg_self_ref).map(FnArg::SelfRef);
                }
            }

            if input.peek(Token![mut]) || input.peek(Token![self]) {
                let ahead = input.fork();
                if ahead.call(arg_self).is_ok() && !ahead.peek(Token![:]) {
                    return input.call(arg_self).map(FnArg::SelfValue);
                }
            }

            let ahead = input.fork();
            let err = match ahead.call(arg_captured) {
                Ok(_) => return input.call(arg_captured).map(FnArg::Captured),
                Err(err) => err,
            };

            let ahead = input.fork();
            if ahead.parse::<Type>().is_ok() {
                return input.parse().map(FnArg::Ignored);
            }

            Err(err)
        }
    }

    fn arg_self_ref(input: ParseStream) -> Result<ArgSelfRef> {
        Ok(ArgSelfRef {
            and_token: input.parse()?,
            lifetime: input.parse()?,
            mutability: input.parse()?,
            self_token: input.parse()?,
        })
    }

    fn arg_self(input: ParseStream) -> Result<ArgSelf> {
        Ok(ArgSelf {
            mutability: input.parse()?,
            self_token: input.parse()?,
        })
    }

    fn arg_captured(input: ParseStream) -> Result<ArgCaptured> {
        Ok(ArgCaptured {
            pat: input.parse()?,
            colon_token: input.parse()?,
            ty: input.parse()?,
        })
    }

    impl Parse for ItemMod {
        fn parse(input: ParseStream) -> Result<Self> {
            let outer_attrs = input.call(Attribute::parse_outer)?;
            let vis: Visibility = input.parse()?;
            let mod_token: Token![mod] = input.parse()?;
            let ident: Ident = input.parse()?;

            let lookahead = input.lookahead1();
            if lookahead.peek(Token![;]) {
                Ok(ItemMod {
                    attrs: outer_attrs,
                    vis: vis,
                    mod_token: mod_token,
                    ident: ident,
                    content: None,
                    semi: Some(input.parse()?),
                })
            } else if lookahead.peek(token::Brace) {
                let content;
                let brace_token = braced!(content in input);
                let inner_attrs = content.call(Attribute::parse_inner)?;

                let mut items = Vec::new();
                while !content.is_empty() {
                    items.push(content.parse()?);
                }

                Ok(ItemMod {
                    attrs: {
                        let mut attrs = outer_attrs;
                        attrs.extend(inner_attrs);
                        attrs
                    },
                    vis: vis,
                    mod_token: mod_token,
                    ident: ident,
                    content: Some((brace_token, items)),
                    semi: None,
                })
            } else {
                Err(lookahead.error())
            }
        }
    }

    impl Parse for ItemForeignMod {
        fn parse(input: ParseStream) -> Result<Self> {
            let outer_attrs = input.call(Attribute::parse_outer)?;
            let abi: Abi = input.parse()?;

            let content;
            let brace_token = braced!(content in input);
            let inner_attrs = content.call(Attribute::parse_inner)?;
            let mut items = Vec::new();
            while !content.is_empty() {
                items.push(content.parse()?);
            }

            Ok(ItemForeignMod {
                attrs: {
                    let mut attrs = outer_attrs;
                    attrs.extend(inner_attrs);
                    attrs
                },
                abi: abi,
                brace_token: brace_token,
                items: items,
            })
        }
    }

    impl Parse for ForeignItem {
        fn parse(input: ParseStream) -> Result<Self> {
            let ahead = input.fork();
            ahead.call(Attribute::parse_outer)?;
            let vis: Visibility = ahead.parse()?;

            let lookahead = ahead.lookahead1();
            if lookahead.peek(Token![fn]) {
                input.parse().map(ForeignItem::Fn)
            } else if lookahead.peek(Token![static]) {
                input.parse().map(ForeignItem::Static)
            } else if lookahead.peek(Token![type]) {
                input.parse().map(ForeignItem::Type)
            } else if vis.is_inherited()
                && (lookahead.peek(Ident)
                    || lookahead.peek(Token![self])
                    || lookahead.peek(Token![super])
                    || lookahead.peek(Token![extern])
                    || lookahead.peek(Token![crate])
                    || lookahead.peek(Token![::]))
            {
                input.parse().map(ForeignItem::Macro)
            } else {
                Err(lookahead.error())
            }
        }
    }

    impl Parse for ForeignItemFn {
        fn parse(input: ParseStream) -> Result<Self> {
            let attrs = input.call(Attribute::parse_outer)?;
            let vis: Visibility = input.parse()?;
            let fn_token: Token![fn] = input.parse()?;
            let ident: Ident = input.parse()?;
            let generics: Generics = input.parse()?;

            let content;
            let paren_token = parenthesized!(content in input);
            let inputs = content.parse_synom(Punctuated::parse_terminated)?;
            let variadic: Option<Token![...]> = if inputs.empty_or_trailing() {
                content.parse()?
            } else {
                None
            };

            let output: ReturnType = input.parse()?;
            let where_clause: Option<WhereClause> = input.parse()?;
            let semi_token: Token![;] = input.parse()?;

            Ok(ForeignItemFn {
                attrs: attrs,
                vis: vis,
                ident: ident,
                decl: Box::new(FnDecl {
                    fn_token: fn_token,
                    paren_token: paren_token,
                    inputs: inputs,
                    output: output,
                    variadic: variadic,
                    generics: Generics {
                        where_clause: where_clause,
                        ..generics
                    },
                }),
                semi_token: semi_token,
            })
        }
    }

    impl Parse for ForeignItemStatic {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(ForeignItemStatic {
                attrs: input.call(Attribute::parse_outer)?,
                vis: input.parse()?,
                static_token: input.parse()?,
                mutability: input.parse()?,
                ident: input.parse()?,
                colon_token: input.parse()?,
                ty: input.parse()?,
                semi_token: input.parse()?,
            })
        }
    }

    impl Parse for ForeignItemType {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(ForeignItemType {
                attrs: input.call(Attribute::parse_outer)?,
                vis: input.parse()?,
                type_token: input.parse()?,
                ident: input.parse()?,
                semi_token: input.parse()?,
            })
        }
    }

    impl Parse for ForeignItemMacro {
        fn parse(input: ParseStream) -> Result<Self> {
            let attrs = input.call(Attribute::parse_outer)?;
            let mac: Macro = input.parse()?;
            let semi_token: Option<Token![;]> = if mac.delimiter.is_brace() {
                None
            } else {
                Some(input.parse()?)
            };
            Ok(ForeignItemMacro {
                attrs: attrs,
                mac: mac,
                semi_token: semi_token,
            })
        }
    }

    impl Parse for ItemType {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(ItemType {
                attrs: input.call(Attribute::parse_outer)?,
                vis: input.parse()?,
                type_token: input.parse()?,
                ident: input.parse()?,
                generics: {
                    let mut generics: Generics = input.parse()?;
                    generics.where_clause = input.parse()?;
                    generics
                },
                eq_token: input.parse()?,
                ty: input.parse()?,
                semi_token: input.parse()?,
            })
        }
    }

    impl Parse for ItemExistential {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(ItemExistential {
                attrs: input.call(Attribute::parse_outer)?,
                vis: input.parse()?,
                existential_token: input.parse()?,
                type_token: input.parse()?,
                ident: input.parse()?,
                generics: {
                    let mut generics: Generics = input.parse()?;
                    generics.where_clause = input.parse()?;
                    generics
                },
                colon_token: Some(input.parse()?),
                bounds: input.parse_synom(Punctuated::parse_separated_nonempty)?,
                semi_token: input.parse()?,
            })
        }
    }

    impl Parse for ItemStruct {
        fn parse(input: ParseStream) -> Result<Self> {
            let attrs = input.call(Attribute::parse_outer)?;
            let vis = input.parse::<Visibility>()?;
            let struct_token = input.parse::<Token![struct]>()?;
            let ident = input.parse::<Ident>()?;
            let generics = input.parse::<Generics>()?;
            let (where_clause, fields, semi_token) = derive::parsing::data_struct(input)?;
            Ok(ItemStruct {
                attrs: attrs,
                vis: vis,
                struct_token: struct_token,
                ident: ident,
                generics: Generics {
                    where_clause: where_clause,
                    ..generics
                },
                fields: fields,
                semi_token: semi_token,
            })
        }
    }

    impl Parse for ItemEnum {
        fn parse(input: ParseStream) -> Result<Self> {
            let attrs = input.call(Attribute::parse_outer)?;
            let vis = input.parse::<Visibility>()?;
            let enum_token = input.parse::<Token![enum]>()?;
            let ident = input.parse::<Ident>()?;
            let generics = input.parse::<Generics>()?;
            let (where_clause, brace_token, variants) = derive::parsing::data_enum(input)?;
            Ok(ItemEnum {
                attrs: attrs,
                vis: vis,
                enum_token: enum_token,
                ident: ident,
                generics: Generics {
                    where_clause: where_clause,
                    ..generics
                },
                brace_token: brace_token,
                variants: variants,
            })
        }
    }

    impl Parse for ItemUnion {
        fn parse(input: ParseStream) -> Result<Self> {
            let attrs = input.call(Attribute::parse_outer)?;
            let vis = input.parse::<Visibility>()?;
            let union_token = input.parse::<Token![union]>()?;
            let ident = input.parse::<Ident>()?;
            let generics = input.parse::<Generics>()?;
            let (where_clause, fields) = derive::parsing::data_union(input)?;
            Ok(ItemUnion {
                attrs: attrs,
                vis: vis,
                union_token: union_token,
                ident: ident,
                generics: Generics {
                    where_clause: where_clause,
                    ..generics
                },
                fields: fields,
            })
        }
    }

    impl Parse for ItemTrait {
        fn parse(input: ParseStream) -> Result<Self> {
            let attrs = input.call(Attribute::parse_outer)?;
            let vis: Visibility = input.parse()?;
            let unsafety: Option<Token![unsafe]> = input.parse()?;
            let auto_token: Option<Token![auto]> = input.parse()?;
            let trait_token: Token![trait] = input.parse()?;
            let ident: Ident = input.parse()?;
            let mut generics: Generics = input.parse()?;
            let colon_token: Option<Token![:]> = input.parse()?;
            let supertraits = if colon_token.is_some() {
                input.parse_synom(Punctuated::parse_separated_nonempty)?
            } else {
                Punctuated::new()
            };
            generics.where_clause = input.parse()?;

            let content;
            let brace_token = braced!(content in input);
            let mut items = Vec::new();
            while !content.is_empty() {
                items.push(content.parse()?);
            }

            Ok(ItemTrait {
                attrs: attrs,
                vis: vis,
                unsafety: unsafety,
                auto_token: auto_token,
                trait_token: trait_token,
                ident: ident,
                generics: generics,
                colon_token: colon_token,
                supertraits: supertraits,
                brace_token: brace_token,
                items: items,
            })
        }
    }

    impl Parse for TraitItem {
        fn parse(input: ParseStream) -> Result<Self> {
            let ahead = input.fork();
            ahead.call(Attribute::parse_outer)?;

            let lookahead = ahead.lookahead1();
            if lookahead.peek(Token![const]) {
                ahead.parse::<Token![const]>()?;
                let lookahead = ahead.lookahead1();
                if lookahead.peek(Ident) {
                    input.parse().map(TraitItem::Const)
                } else if lookahead.peek(Token![unsafe])
                    || lookahead.peek(Token![extern])
                    || lookahead.peek(Token![fn])
                {
                    input.parse().map(TraitItem::Method)
                } else {
                    Err(lookahead.error())
                }
            } else if lookahead.peek(Token![unsafe])
                || lookahead.peek(Token![extern])
                || lookahead.peek(Token![fn])
            {
                input.parse().map(TraitItem::Method)
            } else if lookahead.peek(Token![type]) {
                input.parse().map(TraitItem::Type)
            } else if lookahead.peek(Ident)
                || lookahead.peek(Token![self])
                || lookahead.peek(Token![super])
                || lookahead.peek(Token![extern])
                || lookahead.peek(Token![crate])
                || lookahead.peek(Token![::])
            {
                input.parse().map(TraitItem::Macro)
            } else {
                Err(lookahead.error())
            }
        }
    }

    impl Parse for TraitItemConst {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(TraitItemConst {
                attrs: input.call(Attribute::parse_outer)?,
                const_token: input.parse()?,
                ident: input.parse()?,
                colon_token: input.parse()?,
                ty: input.parse()?,
                default: {
                    if input.peek(Token![=]) {
                        let eq_token: Token![=] = input.parse()?;
                        let default: Expr = input.parse()?;
                        Some((eq_token, default))
                    } else {
                        None
                    }
                },
                semi_token: input.parse()?,
            })
        }
    }

    impl Parse for TraitItemMethod {
        fn parse(input: ParseStream) -> Result<Self> {
            let outer_attrs = input.call(Attribute::parse_outer)?;
            let constness: Option<Token![const]> = input.parse()?;
            let unsafety: Option<Token![unsafe]> = input.parse()?;
            let abi: Option<Abi> = input.parse()?;
            let fn_token: Token![fn] = input.parse()?;
            let ident: Ident = input.parse()?;
            let generics: Generics = input.parse()?;

            let content;
            let paren_token = parenthesized!(content in input);
            let inputs = content.parse_terminated(<FnArg as Parse>::parse)?;

            let output: ReturnType = input.parse()?;
            let where_clause: Option<WhereClause> = input.parse()?;

            let lookahead = input.lookahead1();
            let (brace_token, inner_attrs, stmts, semi_token) = if lookahead.peek(token::Brace) {
                let content;
                let brace_token = braced!(content in input);
                let inner_attrs = content.call(Attribute::parse_inner)?;
                let stmts = content.call(Block::parse_within)?;
                (Some(brace_token), inner_attrs, stmts, None)
            } else if lookahead.peek(Token![;]) {
                let semi_token: Token![;] = input.parse()?;
                (None, Vec::new(), Vec::new(), Some(semi_token))
            } else {
                return Err(lookahead.error());
            };

            Ok(TraitItemMethod {
                attrs: {
                    let mut attrs = outer_attrs;
                    attrs.extend(inner_attrs);
                    attrs
                },
                sig: MethodSig {
                    constness: constness,
                    unsafety: unsafety,
                    asyncness: None,
                    abi: abi,
                    ident: ident,
                    decl: FnDecl {
                        fn_token: fn_token,
                        paren_token: paren_token,
                        inputs: inputs,
                        output: output,
                        variadic: None,
                        generics: Generics {
                            where_clause: where_clause,
                            ..generics
                        },
                    },
                },
                default: brace_token.map(|brace_token| Block {
                    brace_token: brace_token,
                    stmts: stmts,
                }),
                semi_token: semi_token,
            })
        }
    }

    impl Parse for TraitItemType {
        fn parse(input: ParseStream) -> Result<Self> {
            let attrs = input.call(Attribute::parse_outer)?;
            let type_token: Token![type] = input.parse()?;
            let ident: Ident = input.parse()?;
            let mut generics: Generics = input.parse()?;
            let colon_token: Option<Token![:]> = input.parse()?;
            let bounds = if colon_token.is_some() {
                input.parse_synom(Punctuated::parse_separated_nonempty)?
            } else {
                Punctuated::new()
            };
            generics.where_clause = input.parse()?;
            let default = if input.peek(Token![=]) {
                let eq_token: Token![=] = input.parse()?;
                let default: Type = input.parse()?;
                Some((eq_token, default))
            } else {
                None
            };
            let semi_token: Token![;] = input.parse()?;

            Ok(TraitItemType {
                attrs: attrs,
                type_token: type_token,
                ident: ident,
                generics: generics,
                colon_token: colon_token,
                bounds: bounds,
                default: default,
                semi_token: semi_token,
            })
        }
    }

    impl Parse for TraitItemMacro {
        fn parse(input: ParseStream) -> Result<Self> {
            let attrs = input.call(Attribute::parse_outer)?;
            let mac: Macro = input.parse()?;
            let semi_token: Option<Token![;]> = if mac.delimiter.is_brace() {
                None
            } else {
                Some(input.parse()?)
            };
            Ok(TraitItemMacro {
                attrs: attrs,
                mac: mac,
                semi_token: semi_token,
            })
        }
    }

    impl Parse for ItemImpl {
        fn parse(input: ParseStream) -> Result<Self> {
            let outer_attrs = input.call(Attribute::parse_outer)?;
            let defaultness: Option<Token![default]> = input.parse()?;
            let unsafety: Option<Token![unsafe]> = input.parse()?;
            let impl_token: Token![impl ] = input.parse()?;
            let generics: Generics = input.parse()?;
            let trait_ = {
                let ahead = input.fork();
                if ahead.parse::<Option<Token![!]>>().is_ok()
                    && ahead.parse::<Path>().is_ok()
                    && ahead.parse::<Token![for]>().is_ok()
                {
                    let polarity: Option<Token![!]> = input.parse()?;
                    let path: Path = input.parse()?;
                    let for_token: Token![for] = input.parse()?;
                    Some((polarity, path, for_token))
                } else {
                    None
                }
            };
            let self_ty: Type = input.parse()?;
            let where_clause: Option<WhereClause> = input.parse()?;

            let content;
            let brace_token = braced!(content in input);
            let inner_attrs = content.call(Attribute::parse_inner)?;

            let mut items = Vec::new();
            while !content.is_empty() {
                items.push(content.parse()?);
            }

            Ok(ItemImpl {
                attrs: {
                    let mut attrs = outer_attrs;
                    attrs.extend(inner_attrs);
                    attrs
                },
                defaultness: defaultness,
                unsafety: unsafety,
                impl_token: impl_token,
                generics: Generics {
                    where_clause: where_clause,
                    ..generics
                },
                trait_: trait_,
                self_ty: Box::new(self_ty),
                brace_token: brace_token,
                items: items,
            })
        }
    }

    impl Parse for ImplItem {
        fn parse(input: ParseStream) -> Result<Self> {
            let ahead = input.fork();
            ahead.call(Attribute::parse_outer)?;
            let vis: Visibility = ahead.parse()?;

            let mut lookahead = ahead.lookahead1();
            let defaultness = if lookahead.peek(Token![default]) && !ahead.peek2(Token![!]) {
                let defaultness: Token![default] = ahead.parse()?;
                lookahead = ahead.lookahead1();
                Some(defaultness)
            } else {
                None
            };

            if lookahead.peek(Token![const]) {
                ahead.parse::<Token![const]>()?;
                let lookahead = ahead.lookahead1();
                if lookahead.peek(Ident) {
                    input.parse().map(ImplItem::Const)
                } else if lookahead.peek(Token![unsafe])
                    || lookahead.peek(Token![async])
                    || lookahead.peek(Token![extern])
                    || lookahead.peek(Token![fn])
                {
                    input.parse().map(ImplItem::Method)
                } else {
                    Err(lookahead.error())
                }
            } else if lookahead.peek(Token![unsafe])
                || lookahead.peek(Token![async])
                || lookahead.peek(Token![extern])
                || lookahead.peek(Token![fn])
            {
                input.parse().map(ImplItem::Method)
            } else if lookahead.peek(Token![type]) {
                input.parse().map(ImplItem::Type)
            } else if vis.is_inherited()
                && defaultness.is_none()
                && lookahead.peek(Token![existential])
            {
                input.parse().map(ImplItem::Existential)
            } else if vis.is_inherited()
                && defaultness.is_none()
                && (lookahead.peek(Ident)
                    || lookahead.peek(Token![self])
                    || lookahead.peek(Token![super])
                    || lookahead.peek(Token![extern])
                    || lookahead.peek(Token![crate])
                    || lookahead.peek(Token![::]))
            {
                input.parse().map(ImplItem::Macro)
            } else {
                Err(lookahead.error())
            }
        }
    }

    impl Parse for ImplItemConst {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(ImplItemConst {
                attrs: input.call(Attribute::parse_outer)?,
                vis: input.parse()?,
                defaultness: input.parse()?,
                const_token: input.parse()?,
                ident: input.parse()?,
                colon_token: input.parse()?,
                ty: input.parse()?,
                eq_token: input.parse()?,
                expr: input.parse()?,
                semi_token: input.parse()?,
            })
        }
    }

    impl Parse for ImplItemMethod {
        fn parse(input: ParseStream) -> Result<Self> {
            let outer_attrs = input.call(Attribute::parse_outer)?;
            let vis: Visibility = input.parse()?;
            let defaultness: Option<Token![default]> = input.parse()?;
            let constness: Option<Token![const]> = input.parse()?;
            let unsafety: Option<Token![unsafe]> = input.parse()?;
            let asyncness: Option<Token![async]> = input.parse()?;
            let abi: Option<Abi> = input.parse()?;
            let fn_token: Token![fn] = input.parse()?;
            let ident: Ident = input.parse()?;
            let generics: Generics = input.parse()?;

            let content;
            let paren_token = parenthesized!(content in input);
            let inputs = content.parse_terminated(<FnArg as Parse>::parse)?;

            let output: ReturnType = input.parse()?;
            let where_clause: Option<WhereClause> = input.parse()?;

            let content;
            let brace_token = braced!(content in input);
            let inner_attrs = content.call(Attribute::parse_inner)?;
            let stmts = content.call(Block::parse_within)?;

            Ok(ImplItemMethod {
                attrs: {
                    let mut attrs = outer_attrs;
                    attrs.extend(inner_attrs);
                    attrs
                },
                vis: vis,
                defaultness: defaultness,
                sig: MethodSig {
                    constness: constness,
                    unsafety: unsafety,
                    asyncness: asyncness,
                    abi: abi,
                    ident: ident,
                    decl: FnDecl {
                        fn_token: fn_token,
                        paren_token: paren_token,
                        inputs: inputs,
                        output: output,
                        variadic: None,
                        generics: Generics {
                            where_clause: where_clause,
                            ..generics
                        },
                    },
                },
                block: Block {
                    brace_token: brace_token,
                    stmts: stmts,
                },
            })
        }
    }

    impl Parse for ImplItemType {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(ImplItemType {
                attrs: input.call(Attribute::parse_outer)?,
                vis: input.parse()?,
                defaultness: input.parse()?,
                type_token: input.parse()?,
                ident: input.parse()?,
                generics: {
                    let mut generics: Generics = input.parse()?;
                    generics.where_clause = input.parse()?;
                    generics
                },
                eq_token: input.parse()?,
                ty: input.parse()?,
                semi_token: input.parse()?,
            })
        }
    }

    impl Parse for ImplItemExistential {
        fn parse(input: ParseStream) -> Result<Self> {
            let ety: ItemExistential = input.parse()?;
            Ok(ImplItemExistential {
                attrs: ety.attrs,
                existential_token: ety.existential_token,
                type_token: ety.type_token,
                ident: ety.ident,
                generics: ety.generics,
                colon_token: ety.colon_token,
                bounds: ety.bounds,
                semi_token: ety.semi_token,
            })
        }
    }

    impl Parse for ImplItemMacro {
        fn parse(input: ParseStream) -> Result<Self> {
            let attrs = input.call(Attribute::parse_outer)?;
            let mac: Macro = input.parse()?;
            let semi_token: Option<Token![;]> = if mac.delimiter.is_brace() {
                None
            } else {
                Some(input.parse()?)
            };
            Ok(ImplItemMacro {
                attrs: attrs,
                mac: mac,
                semi_token: semi_token,
            })
        }
    }

    impl Visibility {
        fn is_inherited(&self) -> bool {
            match *self {
                Visibility::Inherited => true,
                _ => false,
            }
        }
    }

    impl MacroDelimiter {
        fn is_brace(&self) -> bool {
            match *self {
                MacroDelimiter::Brace(_) => true,
                MacroDelimiter::Paren(_) | MacroDelimiter::Bracket(_) => false,
            }
        }
    }
}

#[cfg(feature = "printing")]
mod printing {
    use super::*;
    use attr::FilterAttrs;
    use proc_macro2::TokenStream;
    use quote::{ToTokens, TokenStreamExt};

    impl ToTokens for ItemExternCrate {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.extern_token.to_tokens(tokens);
            self.crate_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            if let Some((ref as_token, ref rename)) = self.rename {
                as_token.to_tokens(tokens);
                rename.to_tokens(tokens);
            }
            self.semi_token.to_tokens(tokens);
        }
    }

    impl ToTokens for ItemUse {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.use_token.to_tokens(tokens);
            self.leading_colon.to_tokens(tokens);
            self.tree.to_tokens(tokens);
            self.semi_token.to_tokens(tokens);
        }
    }

    impl ToTokens for ItemStatic {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.static_token.to_tokens(tokens);
            self.mutability.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            self.colon_token.to_tokens(tokens);
            self.ty.to_tokens(tokens);
            self.eq_token.to_tokens(tokens);
            self.expr.to_tokens(tokens);
            self.semi_token.to_tokens(tokens);
        }
    }

    impl ToTokens for ItemConst {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.const_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            self.colon_token.to_tokens(tokens);
            self.ty.to_tokens(tokens);
            self.eq_token.to_tokens(tokens);
            self.expr.to_tokens(tokens);
            self.semi_token.to_tokens(tokens);
        }
    }

    impl ToTokens for ItemFn {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.constness.to_tokens(tokens);
            self.unsafety.to_tokens(tokens);
            self.asyncness.to_tokens(tokens);
            self.abi.to_tokens(tokens);
            NamedDecl(&self.decl, &self.ident).to_tokens(tokens);
            self.block.brace_token.surround(tokens, |tokens| {
                tokens.append_all(self.attrs.inner());
                tokens.append_all(&self.block.stmts);
            });
        }
    }

    impl ToTokens for ItemMod {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.mod_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            if let Some((ref brace, ref items)) = self.content {
                brace.surround(tokens, |tokens| {
                    tokens.append_all(self.attrs.inner());
                    tokens.append_all(items);
                });
            } else {
                TokensOrDefault(&self.semi).to_tokens(tokens);
            }
        }
    }

    impl ToTokens for ItemForeignMod {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.abi.to_tokens(tokens);
            self.brace_token.surround(tokens, |tokens| {
                tokens.append_all(self.attrs.inner());
                tokens.append_all(&self.items);
            });
        }
    }

    impl ToTokens for ItemType {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.type_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            self.generics.to_tokens(tokens);
            self.generics.where_clause.to_tokens(tokens);
            self.eq_token.to_tokens(tokens);
            self.ty.to_tokens(tokens);
            self.semi_token.to_tokens(tokens);
        }
    }

    impl ToTokens for ItemExistential {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.existential_token.to_tokens(tokens);
            self.type_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            self.generics.to_tokens(tokens);
            self.generics.where_clause.to_tokens(tokens);
            if !self.bounds.is_empty() {
                TokensOrDefault(&self.colon_token).to_tokens(tokens);
                self.bounds.to_tokens(tokens);
            }
            self.semi_token.to_tokens(tokens);
        }
    }

    impl ToTokens for ItemEnum {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.enum_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            self.generics.to_tokens(tokens);
            self.generics.where_clause.to_tokens(tokens);
            self.brace_token.surround(tokens, |tokens| {
                self.variants.to_tokens(tokens);
            });
        }
    }

    impl ToTokens for ItemStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.struct_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            self.generics.to_tokens(tokens);
            match self.fields {
                Fields::Named(ref fields) => {
                    self.generics.where_clause.to_tokens(tokens);
                    fields.to_tokens(tokens);
                }
                Fields::Unnamed(ref fields) => {
                    fields.to_tokens(tokens);
                    self.generics.where_clause.to_tokens(tokens);
                    TokensOrDefault(&self.semi_token).to_tokens(tokens);
                }
                Fields::Unit => {
                    self.generics.where_clause.to_tokens(tokens);
                    TokensOrDefault(&self.semi_token).to_tokens(tokens);
                }
            }
        }
    }

    impl ToTokens for ItemUnion {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.union_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            self.generics.to_tokens(tokens);
            self.generics.where_clause.to_tokens(tokens);
            self.fields.to_tokens(tokens);
        }
    }

    impl ToTokens for ItemTrait {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.unsafety.to_tokens(tokens);
            self.auto_token.to_tokens(tokens);
            self.trait_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            self.generics.to_tokens(tokens);
            if !self.supertraits.is_empty() {
                TokensOrDefault(&self.colon_token).to_tokens(tokens);
                self.supertraits.to_tokens(tokens);
            }
            self.generics.where_clause.to_tokens(tokens);
            self.brace_token.surround(tokens, |tokens| {
                tokens.append_all(&self.items);
            });
        }
    }

    impl ToTokens for ItemImpl {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.defaultness.to_tokens(tokens);
            self.unsafety.to_tokens(tokens);
            self.impl_token.to_tokens(tokens);
            self.generics.to_tokens(tokens);
            if let Some((ref polarity, ref path, ref for_token)) = self.trait_ {
                polarity.to_tokens(tokens);
                path.to_tokens(tokens);
                for_token.to_tokens(tokens);
            }
            self.self_ty.to_tokens(tokens);
            self.generics.where_clause.to_tokens(tokens);
            self.brace_token.surround(tokens, |tokens| {
                tokens.append_all(self.attrs.inner());
                tokens.append_all(&self.items);
            });
        }
    }

    impl ToTokens for ItemMacro {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.mac.path.to_tokens(tokens);
            self.mac.bang_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            match self.mac.delimiter {
                MacroDelimiter::Paren(ref paren) => {
                    paren.surround(tokens, |tokens| self.mac.tts.to_tokens(tokens));
                }
                MacroDelimiter::Brace(ref brace) => {
                    brace.surround(tokens, |tokens| self.mac.tts.to_tokens(tokens));
                }
                MacroDelimiter::Bracket(ref bracket) => {
                    bracket.surround(tokens, |tokens| self.mac.tts.to_tokens(tokens));
                }
            }
            self.semi_token.to_tokens(tokens);
        }
    }

    impl ToTokens for ItemMacro2 {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.macro_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            self.paren_token.surround(tokens, |tokens| {
                self.args.to_tokens(tokens);
            });
            self.brace_token.surround(tokens, |tokens| {
                self.body.to_tokens(tokens);
            });
        }
    }

    impl ToTokens for ItemVerbatim {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.tts.to_tokens(tokens);
        }
    }

    impl ToTokens for UsePath {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.ident.to_tokens(tokens);
            self.colon2_token.to_tokens(tokens);
            self.tree.to_tokens(tokens);
        }
    }

    impl ToTokens for UseName {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.ident.to_tokens(tokens);
        }
    }

    impl ToTokens for UseRename {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.ident.to_tokens(tokens);
            self.as_token.to_tokens(tokens);
            self.rename.to_tokens(tokens);
        }
    }

    impl ToTokens for UseGlob {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.star_token.to_tokens(tokens);
        }
    }

    impl ToTokens for UseGroup {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.brace_token.surround(tokens, |tokens| {
                self.items.to_tokens(tokens);
            });
        }
    }

    impl ToTokens for TraitItemConst {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.const_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            self.colon_token.to_tokens(tokens);
            self.ty.to_tokens(tokens);
            if let Some((ref eq_token, ref default)) = self.default {
                eq_token.to_tokens(tokens);
                default.to_tokens(tokens);
            }
            self.semi_token.to_tokens(tokens);
        }
    }

    impl ToTokens for TraitItemMethod {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.sig.to_tokens(tokens);
            match self.default {
                Some(ref block) => {
                    block.brace_token.surround(tokens, |tokens| {
                        tokens.append_all(self.attrs.inner());
                        tokens.append_all(&block.stmts);
                    });
                }
                None => {
                    TokensOrDefault(&self.semi_token).to_tokens(tokens);
                }
            }
        }
    }

    impl ToTokens for TraitItemType {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.type_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            self.generics.to_tokens(tokens);
            if !self.bounds.is_empty() {
                TokensOrDefault(&self.colon_token).to_tokens(tokens);
                self.bounds.to_tokens(tokens);
            }
            self.generics.where_clause.to_tokens(tokens);
            if let Some((ref eq_token, ref default)) = self.default {
                eq_token.to_tokens(tokens);
                default.to_tokens(tokens);
            }
            self.semi_token.to_tokens(tokens);
        }
    }

    impl ToTokens for TraitItemMacro {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.mac.to_tokens(tokens);
            self.semi_token.to_tokens(tokens);
        }
    }

    impl ToTokens for TraitItemVerbatim {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.tts.to_tokens(tokens);
        }
    }

    impl ToTokens for ImplItemConst {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.defaultness.to_tokens(tokens);
            self.const_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            self.colon_token.to_tokens(tokens);
            self.ty.to_tokens(tokens);
            self.eq_token.to_tokens(tokens);
            self.expr.to_tokens(tokens);
            self.semi_token.to_tokens(tokens);
        }
    }

    impl ToTokens for ImplItemMethod {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.defaultness.to_tokens(tokens);
            self.sig.to_tokens(tokens);
            self.block.brace_token.surround(tokens, |tokens| {
                tokens.append_all(self.attrs.inner());
                tokens.append_all(&self.block.stmts);
            });
        }
    }

    impl ToTokens for ImplItemType {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.defaultness.to_tokens(tokens);
            self.type_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            self.generics.to_tokens(tokens);
            self.generics.where_clause.to_tokens(tokens);
            self.eq_token.to_tokens(tokens);
            self.ty.to_tokens(tokens);
            self.semi_token.to_tokens(tokens);
        }
    }

    impl ToTokens for ImplItemExistential {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.existential_token.to_tokens(tokens);
            self.type_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            self.generics.to_tokens(tokens);
            self.generics.where_clause.to_tokens(tokens);
            if !self.bounds.is_empty() {
                TokensOrDefault(&self.colon_token).to_tokens(tokens);
                self.bounds.to_tokens(tokens);
            }
            self.semi_token.to_tokens(tokens);
        }
    }

    impl ToTokens for ImplItemMacro {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.mac.to_tokens(tokens);
            self.semi_token.to_tokens(tokens);
        }
    }

    impl ToTokens for ImplItemVerbatim {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.tts.to_tokens(tokens);
        }
    }

    impl ToTokens for ForeignItemFn {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            NamedDecl(&self.decl, &self.ident).to_tokens(tokens);
            self.semi_token.to_tokens(tokens);
        }
    }

    impl ToTokens for ForeignItemStatic {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.static_token.to_tokens(tokens);
            self.mutability.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            self.colon_token.to_tokens(tokens);
            self.ty.to_tokens(tokens);
            self.semi_token.to_tokens(tokens);
        }
    }

    impl ToTokens for ForeignItemType {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.type_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            self.semi_token.to_tokens(tokens);
        }
    }

    impl ToTokens for ForeignItemMacro {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.mac.to_tokens(tokens);
            self.semi_token.to_tokens(tokens);
        }
    }

    impl ToTokens for ForeignItemVerbatim {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.tts.to_tokens(tokens);
        }
    }

    impl ToTokens for MethodSig {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.constness.to_tokens(tokens);
            self.unsafety.to_tokens(tokens);
            self.asyncness.to_tokens(tokens);
            self.abi.to_tokens(tokens);
            NamedDecl(&self.decl, &self.ident).to_tokens(tokens);
        }
    }

    struct NamedDecl<'a>(&'a FnDecl, &'a Ident);

    impl<'a> ToTokens for NamedDecl<'a> {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.0.fn_token.to_tokens(tokens);
            self.1.to_tokens(tokens);
            self.0.generics.to_tokens(tokens);
            self.0.paren_token.surround(tokens, |tokens| {
                self.0.inputs.to_tokens(tokens);
                if self.0.variadic.is_some() && !self.0.inputs.empty_or_trailing() {
                    <Token![,]>::default().to_tokens(tokens);
                }
                self.0.variadic.to_tokens(tokens);
            });
            self.0.output.to_tokens(tokens);
            self.0.generics.where_clause.to_tokens(tokens);
        }
    }

    impl ToTokens for ArgSelfRef {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.and_token.to_tokens(tokens);
            self.lifetime.to_tokens(tokens);
            self.mutability.to_tokens(tokens);
            self.self_token.to_tokens(tokens);
        }
    }

    impl ToTokens for ArgSelf {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.mutability.to_tokens(tokens);
            self.self_token.to_tokens(tokens);
        }
    }

    impl ToTokens for ArgCaptured {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.pat.to_tokens(tokens);
            self.colon_token.to_tokens(tokens);
            self.ty.to_tokens(tokens);
        }
    }
}
