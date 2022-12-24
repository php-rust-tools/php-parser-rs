use crate::lexer::token::TokenKind;
use crate::parser::ast::modifiers::PropertyModifierGroup;
use crate::parser::ast::properties::Property;
use crate::parser::ast::properties::PropertyEntry;
use crate::parser::ast::properties::VariableProperty;
use crate::parser::error;
use crate::parser::error::ParseResult;
use crate::parser::expressions;
use crate::parser::internal::data_type;
use crate::parser::internal::utils;
use crate::parser::internal::variables;
use crate::parser::state::State;

pub fn parse(
    state: &mut State,
    class_name: &str,
    modifiers: PropertyModifierGroup,
) -> ParseResult<Property> {
    let ty = data_type::optional_data_type(state)?;

    let mut entries = vec![];
    let mut type_checked = false;
    loop {
        let variable = variables::simple_variable(state)?;

        if !type_checked {
            type_checked = true;
            if modifiers.has_readonly() && modifiers.has_static() {
                return Err(error::static_property_cannot_be_readonly(
                    state.named(class_name),
                    variable.to_string(),
                    variable.span,
                    modifiers.get_static().unwrap().span(),
                    modifiers.get_readonly().unwrap().span(),
                ));
            }

            match &ty {
                Some(ty) => {
                    if ty.includes_callable() || ty.is_bottom() {
                        return Err(error::forbidden_type_used_in_property(
                            state.named(class_name),
                            variable.to_string(),
                            variable.span,
                            ty.clone(),
                        ));
                    }
                }
                None => {
                    if let Some(modifier) = modifiers.get_readonly() {
                        return Err(error::missing_type_for_readonly_property(
                            state.named(class_name),
                            variable.to_string(),
                            variable.span,
                            modifier.span(),
                        ));
                    }
                }
            }
        }

        let current = state.stream.current();
        if current.kind == TokenKind::Equals {
            if let Some(modifier) = modifiers.get_readonly() {
                return Err(error::readonly_property_has_default_value(
                    state.named(class_name),
                    variable.to_string(),
                    variable.span,
                    modifier.span(),
                    current.span,
                ));
            }

            state.stream.next();
            let value = expressions::create(state)?;

            entries.push(PropertyEntry::Initialized {
                variable,
                equals: current.span,
                value,
            });
        } else {
            entries.push(PropertyEntry::Uninitialized { variable });
        }

        if state.stream.current().kind == TokenKind::Comma {
            state.stream.next();
        } else {
            break;
        }
    }

    let end = utils::skip_semicolon(state)?;

    Ok(Property {
        r#type: ty,
        modifiers,
        attributes: state.get_attributes(),
        entries,
        end,
    })
}

pub fn parse_var(state: &mut State, class_name: &str) -> ParseResult<VariableProperty> {
    utils::skip(state, TokenKind::Var)?;

    let ty = data_type::optional_data_type(state)?;

    let mut entries = vec![];
    let mut type_checked = false;
    loop {
        let variable = variables::simple_variable(state)?;

        if !type_checked {
            type_checked = true;

            if let Some(ty) = &ty {
                if ty.includes_callable() || ty.is_bottom() {
                    return Err(error::forbidden_type_used_in_property(
                        state.named(class_name),
                        variable.to_string(),
                        variable.span,
                        ty.clone(),
                    ));
                }
            }
        }

        let current = state.stream.current();
        if current.kind == TokenKind::Equals {
            let span = current.span;
            state.stream.next();
            let value = expressions::create(state)?;

            entries.push(PropertyEntry::Initialized {
                variable,
                equals: span,
                value,
            });
        } else {
            entries.push(PropertyEntry::Uninitialized { variable });
        }

        if state.stream.current().kind == TokenKind::Comma {
            state.stream.next();
        } else {
            break;
        }
    }

    let end = utils::skip_semicolon(state)?;

    Ok(VariableProperty {
        r#type: ty,
        attributes: state.get_attributes(),
        entries,
        end,
    })
}
