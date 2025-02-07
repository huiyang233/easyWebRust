use darling::{Error, FromDeriveInput, FromMeta};
use darling::FromField;
use proc_macro::TokenStream;
use darling::ast::NestedMeta;
use darling::usage::UsesTypeParams;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{format_ident, quote, ToTokens};
use regex::Regex;
use syn::punctuated::Punctuated;
use syn::{parse::Parse, parse::ParseStream, parse_macro_input, Data, DeriveInput, Field, Fields, FnArg, GenericArgument, ItemFn, Pat, PathArguments, ReturnType, Token, Type};

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(curd))]
struct CURDOpts {
    table_name: Option<String>
}

#[derive(FromField, Default)]
#[darling(default, attributes(curd))]
struct FieldOpts {
    pk: bool,
    logic_del: bool,
}

#[derive(Debug, FromMeta)]
struct MethodOpts {
    sql: String,
}





fn generate_custom_query_method(method_name: &syn::Ident, sql: &str) -> TokenStream2 {
    quote! {
        pub async fn #method_name(db: impl sqlx::Executor<'_, Database=sqlx::Postgres>) -> Result<Vec<Self>, sqlx::Error> {
            crate::utils::db::QueryBuilder::<Self>::new_sql(#sql)
                .fetch_all()
                .await
        }
    }
}

fn convert_type_to_ref(ty: &Type) -> TokenStream2 {
    match ty {
        Type::Path(type_path) => {
            let last_segment = type_path.path.segments.last().unwrap();
            if last_segment.ident == "String" {
                quote! { &str }
            } else {
                quote! { &#ty }
            }
        }
        _ => quote! { &#ty },
    }
}

fn find_pk_field(fields: &syn::punctuated::Punctuated<Field, syn::token::Comma>) -> Option<(syn::Ident, syn::Type)> {
    for field in fields {
        let opts = FieldOpts::from_field(field).unwrap();
        if opts.pk {
            if let Some(ident) = &field.ident {
                return Some((ident.clone(), field.ty.clone()));
            }
        }
    }
    None
}

fn find_logic_del_field(fields: &syn::punctuated::Punctuated<Field, syn::token::Comma>) -> Option<syn::Ident> {
    for field in fields {
        let opts = FieldOpts::from_field(field).unwrap();
        if opts.logic_del {
            if let Some(ident) = &field.ident {
                return Some(ident.clone());
            }
        }
    }
    None
}

fn get_all_fields(fields: &syn::punctuated::Punctuated<Field, syn::token::Comma>)
    -> Vec<(syn::Ident, Type)> {
    fields.iter()
        .filter_map(|field| {
            field.ident.clone().map(|ident| (ident, field.ty.clone()))
        })
        .collect()
}



fn generate_get_logic_del_field_method(logic_del_field: &Option<syn::Ident>) -> TokenStream2 {
    let sql = if let Some(logic_del_field) = logic_del_field {
        let string = format!("{}",logic_del_field);
        quote!(
            Some(#string)
        )
    } else {
        quote!{None}
    };

    quote! {
        pub fn get_logic_del_field() -> Option<&'static str> {
           return #sql;
        }
    }
}

fn generate_select_by_pk_method(table_name: &str, pk_field: &Option<(syn::Ident, Type)>, logic_del_field: &Option<syn::Ident>, ) -> TokenStream2 {
    if let Some((pk_ident, pk_type)) = pk_field {
        let method_name = format_ident!("select_by_{}", pk_ident);
        let param_type = convert_type_to_ref(pk_type);

        let sql = if let Some(logic_del_field) = logic_del_field {
            format!(
                "SELECT * FROM {} WHERE {} = ? AND {} = false LIMIT 1",
                table_name,
                pk_ident,
                logic_del_field
            )
        } else {
            format!(
                "SELECT * FROM {} WHERE {} = ? LIMIT 1",
                table_name,
                pk_ident
            )
        };

        quote! {
            pub async fn #method_name(#pk_ident: #param_type) -> Result<Option<Self>, sqlx::Error> {
                crate::utils::db::QueryBuilder::<Self>::new_sql(#sql)
                    .bind(#pk_ident)
                    .fetch_optional()
                    .await
            }
        }
    } else {
        quote! {}
    }
}

fn generate_insert_method(table_name: &str, fields: &syn::punctuated::Punctuated<Field, syn::token::Comma>, ) -> TokenStream2 {
    let all_fields = get_all_fields(fields);
    let field_names: Vec<_> = all_fields.iter()
        .map(|(ident, _)| ident.to_string())
        .collect();
    
    let fields_str = field_names.join(", ");
    let placeholders = vec!["?"; field_names.len()].join(", ");

    let sql= format!(
        "INSERT INTO {} ({}) VALUES ({})",
        table_name,
        fields_str,
        placeholders
    );
    let field_values: Vec<_> = all_fields.iter()
        .map(|(ident, _)| quote! { &self.#ident })
        .collect();

    quote! {
        pub async fn insert(&self,db: impl sqlx::Executor<'_, Database=sqlx::Postgres>) -> Result<u64, sqlx::Error> {
            let mut builder = crate::utils::db::QueryBuilder::<crate::utils::db::Executor>::new_sql(#sql)
            #(.bind(#field_values))*;
            builder.transaction(db).await
        }
    }
}

fn generate_get_table_name_method(table_name: &str) -> TokenStream2 {
    quote! {
        pub fn get_table_name() -> &'static str  {
            return #table_name
        }
    }
}

fn generate_batch_insert_method(table_name: &str, fields: &syn::punctuated::Punctuated<Field, syn::token::Comma>, ) -> TokenStream2 {
    let all_fields = get_all_fields(fields);
    let field_names: Vec<_> = all_fields.iter()
        .map(|(ident, _)| ident.to_string())
        .collect();
    
    let fields_str = field_names.join(", ");
    let placeholders = vec!["?"; field_names.len()].join(", ");
    
    let sql = format!(
        "INSERT INTO {} ({}) VALUES ",
        table_name,
        fields_str,
    );

    let field_values: Vec<_> = all_fields.iter()
        .map(|(ident, _)| quote! { &x.#ident })
        .collect();

    let value = format!("({}),", placeholders);
    quote! {
        pub async fn insert_batch(db: impl sqlx::Executor<'_, Database=sqlx::Postgres>,items: &Vec<Self>) -> Result<u64, sqlx::Error> {
            let mut builder = crate::utils::db::QueryBuilder::<crate::utils::db::Executor>::new_sql(#sql);
            
            for x in items {
                builder.push_sql(#value);
                builder = builder
                #(.bind(#field_values))*
            }
            
            builder.trim();
            builder.transaction(db).await
        }
    }
}


fn generate_update_method(table_name: &str, fields: &syn::punctuated::Punctuated<Field, syn::token::Comma>, pk_field: &Option<(syn::Ident, syn::Type)>, logic_del_field: &Option<syn::Ident>, ) -> TokenStream2 {
    if let Some((pk_ident, _)) = pk_field {
        let all_fields = get_all_fields(fields);
        let update_fields: Vec<_> = all_fields.iter()
            .filter(|(ident, _)| {
                // 排除主键和逻辑删除字段
                Some(ident) != Some(pk_ident) &&
                    Some(ident) != logic_del_field.as_ref()
            })
            .collect();


        let field_bindings = update_fields.iter().map(|(ident, ty)| {
            let is_option = if let Type::Path(type_path) = ty {
                type_path.path.segments.last()
                    .map(|seg| seg.ident == "Option")
                    .unwrap_or(false)
            } else {
                false
            };

            if is_option {
                quote! {
                    if let Some(val) = &self.#ident {
                        builder.push_sql(concat!(" ", stringify!(#ident), " = ? ,"));
                        builder.bind_value(val);
                    }
                }
            } else {
                quote! {
                    builder.push_sql(concat!(" ", stringify!(#ident), " = ? ,"));
                    builder.bind_value(&self.#ident);
                }
            }
        });

        quote! {
            pub async fn update(&self,db: impl sqlx::Executor<'_, Database=sqlx::Postgres>) -> Result<u64, sqlx::Error> {
                let mut builder = crate::utils::db::QueryBuilder::<crate::utils::db::Executor>::new_sql(
                    concat!("UPDATE ", #table_name, " SET")
                );

                #(#field_bindings)*

                builder.trim();
                builder.push_sql(concat!(" WHERE ", stringify!(#pk_ident), " = ?"));
                builder.bind(&self.#pk_ident).transaction(db).await
            }
        }
    } else {
        quote! {
        }
    }
}

fn generate_update_col_method(name:String,table_name: &str,
                              fields: &Punctuated<Field, syn::token::Comma>,
                              pk_field: &Option<(syn::Ident, syn::Type)>,
                              logic_del_field: &Option<syn::Ident>
) -> TokenStream2 {
    if let Some((pk_ident, _)) = pk_field {
        let all_fields = get_all_fields(fields);
        let update_fields: Vec<_> = all_fields.iter()
            .filter(|(ident, _)| {
                // 排除主键和逻辑删除字段
                Some(ident) != Some(pk_ident) &&
                    Some(ident) != logic_del_field.as_ref()
            })
            .collect();


        let field_bindings = update_fields.iter().map(|(ident, ty)| {
            let is_option = if let Type::Path(type_path) = ty {
                type_path.path.segments.last()
                    .map(|seg| seg.ident == "Option")
                    .unwrap_or(false)
            } else {
                false
            };

            if is_option {
                quote! {
                    if let Some(val) = &self.#ident {
                        builder.push_sql(concat!(" ", stringify!(#ident), " = ? ,"));
                        builder.bind_value(val);
                    }
                }
            } else {
                quote! {
                    builder.push_sql(concat!(" ", stringify!(#ident), " = ? ,"));
                    builder.bind_value(&self.#ident);
                }
            }
        });
        let enum_name = format_ident!("{}Field", name);

        let field_match_arms = all_fields.iter().map(|(ident, _)| {
            let variant_name = format_ident!("{}", to_pascal_case(ident.to_string().as_str()));
            quote! {
                #enum_name::#variant_name => {
                    builder.push_sql(concat!(" ", stringify!(#ident), " = ?"));
                    builder.bind_value(&self.#ident);
                }
            }
        });

        quote! {
            pub async fn update_by_col(&self,db: impl sqlx::Executor<'_, Database=sqlx::Postgres>,col:#enum_name) -> Result<u64, sqlx::Error> {
                let mut builder = crate::utils::db::QueryBuilder::<crate::utils::db::Executor>::new_sql(
                    concat!("UPDATE ", #table_name, " SET")
                );

                #(#field_bindings)*

                builder.trim();
                builder.push_sql(" WHERE ");
                match col {
                    #(#field_match_arms),*
                }

                builder.transaction(db).await
            }
        }
    } else {
        quote! {
        }
    }
}


fn generate_delete_method(table_name: &str, pk_field: &Option<(syn::Ident, syn::Type)>, logic_del_field: &Option<syn::Ident>, ) -> TokenStream2 {

    if let Some((pk_ident, _)) = pk_field {
        let sql = if let Some(logic_del_field) = logic_del_field {
            format!(
                "UPDATE {} SET {} = true WHERE {} = ?",
                table_name,
                logic_del_field,
                pk_ident
            )

        } else {
            format!(
                "DELETE FROM {} WHERE {} = ?",
                table_name,
                pk_ident
            )
        };
        quote! {
            pub async fn delete(&self,db: impl sqlx::Executor<'_, Database=sqlx::Postgres>) -> Result<u64, sqlx::Error> {
                crate::utils::db::QueryBuilder::<crate::utils::db::Executor>::new_sql(#sql)
                    .bind(&self.#pk_ident)
                    .transaction(db)
                    .await
            }
        }
    } else {
        quote! {
        }
    }
}


fn generate_delete_by_id_method(table_name: &str, pk_field: &Option<(syn::Ident, syn::Type)>, logic_del_field: &Option<syn::Ident>, ) -> TokenStream2 {

    if let Some((pk_ident, _)) = pk_field {
        let sql = if let Some(logic_del_field) = logic_del_field {
            format!(
                "UPDATE {} SET {} = true WHERE {} = ?",
                table_name,
                logic_del_field,
                pk_ident
            )

        } else {
            format!(
                "DELETE FROM {} WHERE {} = ?",
                table_name,
                pk_ident
            )
        };
        quote! {
            pub async fn delete_by_id(db: impl sqlx::Executor<'_, Database=sqlx::Postgres>,id:&i64) -> Result<u64, sqlx::Error> {
                crate::utils::db::QueryBuilder::<crate::utils::db::Executor>::new_sql(#sql)
                    .bind(id)
                    .transaction(db)
                    .await
            }
        }
    } else {
        quote! {
        }
    }
}


struct ParseArgs {
    pub sqls: Vec<syn::Ident>,
}

impl Parse for ParseArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let r = Punctuated::<syn::Ident, Token![,]>::parse_terminated(input)?;
        Ok(Self {
            sqls: r.into_iter().collect(),
        })
    }
}

fn to_snake_name(name: &str) -> String {
    let len = name.len();
    let bytes = name.as_bytes();
    let mut new_name = String::with_capacity(name.len());
    let mut index = 0;
    for x in bytes {
        let c = *x as char;
        if c.is_ascii_uppercase() {
            if index != 0 && (index + 1) != len {
                new_name.push('_');
            }
            new_name.push(c.to_ascii_lowercase() as char);
        } else {
            new_name.push(c);
        }
        index += 1;
    }
    return new_name;
}

fn get_return_type(return_type: ReturnType)->Option<Vec<Ident>>{
    let ty = if let ReturnType::Type(_, ref ty) = return_type {
        ty
    }else {
        return None;
    };
    let type_path = if let Type::Path(type_path) = ty.as_ref() {
        type_path
    }else {
        return None;
    };
    let mut return_type = vec![];
    type_path.path.segments.iter().for_each(|x|{
        println!("ident:{:?}", x.ident.to_string());
        return_type.push(x.ident.clone());
            if let PathArguments::AngleBracketed(v) = x.arguments.clone() {
                // 最外层的Result
                if let Some(GenericArgument::Type(Type::Path(inner_type_path))) = v.args.first() {
                    inner_type_path.path.segments.iter().for_each(|option| {
                        // 第二层的Option
                        return_type.push(option.ident.clone());
                        if let PathArguments::AngleBracketed(v) = option.arguments.clone() {
                            if let Some(GenericArgument::Type(Type::Path(inner_type_path))) = v.args.first() {
                                inner_type_path.path.segments.iter().for_each(|option| {
                                    return_type.push(option.ident.clone());
                                    if let PathArguments::AngleBracketed(v) = option.arguments.clone() {
                                        if let Some(GenericArgument::Type(Type::Path(inner_type_path))) = v.args.first() {
                                            inner_type_path.path.segments.iter().for_each(|option| {
                                                return_type.push(option.ident.clone());
                                            })
                                        }
                                    }
                                })
                            }
                        }
                    });
                }
            }
    });
    Some(return_type)
}

fn extract_placeholders(input: &str) -> Vec<String> {
    let re = Regex::new(r"\#\{([^}]+)\}").unwrap();
    re.captures_iter(input)
        .filter_map(|caps| caps.get(1).map(|m| m.as_str().to_string()))
        .collect()
}

#[proc_macro_attribute]
pub fn select(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => { return TokenStream::from(Error::from(e).write_errors()); }
    };

    let input = syn::parse_macro_input!(input as ItemFn);
    let args = match MethodOpts::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => { return TokenStream::from(e.write_errors()); }
    };

    let visibility = input.vis;
    let signature = input.sig;
    let asyncness = signature.asyncness;
    let ident = signature.ident;
    let inputs = signature.inputs;
    let output = signature.output;

    let mut is_option = false;
    let mut is_vec = false;
    let return_type_vec = get_return_type(output.clone());
    let return_type_vec =  match return_type_vec {
        None => { panic!("Not Return Type") }
        Some(return_type_vec) => {return_type_vec}
    };

    if return_type_vec.contains(&Ident::new("Vec", Span::call_site())) {
        is_vec = true;
    }



    if return_type_vec.contains(&Ident::new("Option", Span::call_site())) {
        is_option = true;
    }
    let fetch_type = if is_option {
        quote! {
            .fetch_optional_no_marks().await
        }
    }else if is_vec{
        quote! {
            .fetch_all_no_marks().await
        }
    }else{
        quote! {
            .fetch_one_no_marks().await
        }
    };
    let return_type = return_type_vec.get(return_type_vec.len() - 1);
    let return_type =  match return_type {
        None => { panic!("Not Return Type") }
        Some(return_type) => {return_type}
    };
    let sql = args.sql;
    let placeholders = extract_placeholders(&sql);
    let mut args_vec = vec![];
    for fn_arg in inputs.clone() {
        if let FnArg::Typed(typed)=fn_arg{
            if let Pat::Ident(x)=typed.pat.as_ref() {
                args_vec.push(x.ident.clone())
            }
        }
    }
    let bind_field:Vec<_> = placeholders.iter().map(|x|{
        let split = x.split('.');
        let mut field = vec![];
        for x in split {
            field.push(Ident::new(x, Span::call_site()))
        }
        quote! { #(#field).* }
    }).collect();

    // 替换占位符为 ?
    let sql_with_placeholders = placeholders.iter()
        .enumerate()
        .fold(sql.to_string(), |acc, (index, placeholder)| {
            acc.replace(&format!("#{{{}}}", placeholder), &format!("${}", index + 1))
        });


    let expanded = quote! {
        #visibility #asyncness fn #ident(#inputs) #output{
            crate::utils::db::QueryBuilder::<#return_type>::new_sql(#sql_with_placeholders)
            #(.bind(#bind_field))*
            #fetch_type
        }
    };


    let stream = TokenStream::from(expanded);
    println!("stream:::{:#?}", stream.to_string());
    stream

}

#[proc_macro_derive(CURD, attributes(curd))]
pub fn derive_curd(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let opts = match CURDOpts::from_derive_input(&input) {
        Ok(opts) => opts,
        Err(e) => return e.write_errors().into(),
    };

    let table_name = opts.table_name.unwrap_or_else(|| {
        to_snake_name(&input.ident.to_string())
    });

    let fields = match &input.data {
        Data::Struct(data) => {
            match &data.fields {
                Fields::Named(fields) => &fields.named,
                _ => panic!("CRUD derive only supports named fields"),
            }
        },
        _ => panic!("CRUD derive only supports structs"),
    };


    let pk_field = find_pk_field(fields);
    let logic_del_field = find_logic_del_field(fields);

    let select_by_pk_method = generate_select_by_pk_method(&table_name, &pk_field, &logic_del_field);
    let save_method = generate_insert_method(&table_name, fields);
    let delete_method = generate_delete_method(&table_name, &pk_field, &logic_del_field);
    let delete_by_id_method = generate_delete_by_id_method(&table_name, &pk_field, &logic_del_field);
    let update_method = generate_update_method(&table_name, fields, &pk_field, &logic_del_field);
    let batch_insert_method = generate_batch_insert_method(&table_name, fields);
    let update_col_method = generate_update_col_method(name.to_string(),&table_name, fields, &pk_field, &logic_del_field);
    let get_table_name_method = generate_get_table_name_method(&table_name);

    let get_logic_del_field_method = generate_get_logic_del_field_method(&logic_del_field);

    // 生成字段枚举
    let field_enum = generate_field_enum(name.to_string(),fields);

    let expanded = quote! {
        impl #name {
            #select_by_pk_method
            #update_method
            #update_col_method
            #save_method
            #batch_insert_method
            #delete_method
            #delete_by_id_method
            #get_table_name_method
            #get_logic_del_field_method
        }

        #field_enum
    };

    TokenStream::from(expanded)
}


fn to_pascal_case(input: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in input.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c.to_ascii_lowercase());
        }
    }

    result
}

// 生成字段枚举
fn generate_field_enum(name:String,fields: &Punctuated<Field, syn::token::Comma>) -> TokenStream2 {
    let enum_name = format_ident!("{}Field", name);
    let variants: Vec<_> = fields.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        let variant_name = format_ident!("{}", to_pascal_case(ident.to_string().as_str()));
        quote! { #variant_name }
    }).collect();

    quote! {
        #[derive(Debug)]
        pub enum #enum_name {
            #(#variants),*
        }
    }
}
