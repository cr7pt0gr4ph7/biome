use crate::{
    inner_string_text, AnyJsBinding, AnyJsImportClause, AnyJsModuleSource,
    AnyJsNamedImportSpecifier, JsCallExpression, JsDefaultImportSpecifier, JsImport,
    JsImportAssertion, JsImportCallExpression, JsModuleSource, JsNamedImportSpecifier,
    JsNamespaceImportSpecifier, JsShorthandNamedImportSpecifier, JsSyntaxKind, JsSyntaxToken,
};
use biome_rowan::{
    declare_node_union, AstNode, SyntaxError, SyntaxNodeOptionExt, SyntaxResult, SyntaxToken,
    TokenText,
};

impl JsImport {
    /// Returns the source of an import.
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_syntax::T;
    ///
    /// let source = make::js_module_source(make::js_string_literal("react"));
    /// let binding = make::js_identifier_binding(make::ident("React"));
    /// let specifier = make::js_default_import_specifier(binding.into());
    /// let clause = make::js_import_default_clause(specifier, make::token(T![from]), source.into()).build();
    /// let import = make::js_import(make::token(T![import]), clause.into()).build();
    ///
    /// assert_eq!(import.source_text().unwrap().text(), "react");
    /// ```
    pub fn source_text(&self) -> SyntaxResult<TokenText> {
        self.import_clause()?.source()?.inner_string_text()
    }

    /// Returns the whole token text of the import source specifier.
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_syntax::T;
    ///
    /// let source = make::js_module_source(make::js_string_literal("react"));
    /// let binding = make::js_identifier_binding(make::ident("React"));
    /// let specifier = make::js_default_import_specifier(binding.into());
    /// let clause = make::js_import_default_clause(specifier, make::token(T![from]), source.into()).build();
    /// let import = make::js_import(make::token(T![import]), clause.into()).build();
    ///
    /// assert_eq!(import.source_token().unwrap().text(), "\"react\"");
    /// ```
    pub fn source_token(&self) -> SyntaxResult<SyntaxToken<crate::JsLanguage>>{
        self.import_clause()?.source()?.value_token()
    }
}

impl JsImportCallExpression {
    /// Returns the inner text of the module specifier:
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_syntax::{AnyJsCallArgument, AnyJsExpression, T};
    /// use biome_js_factory::make;
    ///
    /// let import_token = make::token(T![import]);
    /// let source_name = make::js_string_literal_expression(make::js_string_literal("foo"));
    /// let call_arguments = make::js_call_arguments(
    ///     make::token(T!['(']),
    ///     make::js_call_argument_list([AnyJsCallArgument::AnyJsExpression(AnyJsExpression::AnyJsLiteralExpression(source_name.into()))], []),
    ///     make::token(T![')']),
    /// );
    /// let import_call_expression = make::js_import_call_expression(import_token, call_arguments);
    /// assert_eq!(import_call_expression.module_source_text().unwrap().text(), "foo");
    /// ```
    pub fn module_source_text(&self) -> Option<TokenText> {
        let [Some(argument)] = self.arguments().ok()?.get_arguments_by_index([0])
        else {
            return None;
        };
        argument
            .as_any_js_expression()?
            .as_any_js_literal_expression()?
            .as_js_string_literal_expression()?
            .inner_string_text()
            .ok()
    }

    /// Returns the whole token text of the module specifier, with quotes included:
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_syntax::{AnyJsCallArgument, AnyJsExpression, T};
    /// use biome_js_factory::make;
    ///
    /// let import_token = make::token(T![import]);
    /// let source_name = make::js_string_literal_expression(make::js_string_literal("foo"));
    /// let call_arguments = make::js_call_arguments(
    ///     make::token(T!['(']),
    ///     make::js_call_argument_list([AnyJsCallArgument::AnyJsExpression(AnyJsExpression::AnyJsLiteralExpression(source_name.into()))], []),
    ///     make::token(T![')']),
    /// );
    /// let import_call_expression = make::js_import_call_expression(import_token, call_arguments);
    /// assert_eq!(import_call_expression.module_source_token().unwrap().text(), "\"foo\"");
    /// ```
    pub fn module_source_token(&self) -> Option<SyntaxToken<crate::JsLanguage>> {
        let [Some(argument)] = self.arguments().ok()?.get_arguments_by_index([0])
        else {
            return None;
        };
        argument
            .as_any_js_expression()?
            .as_any_js_literal_expression()?
            .as_js_string_literal_expression()?
            .value_token()
            .ok()
    }
}

impl JsCallExpression {
    /// This is a specialized function that checks if the current [call expression]
    /// represents a call to `require("...")`:
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_syntax::{AnyJsCallArgument, AnyJsExpression, T};
    /// use biome_js_factory::make;
    ///
    /// let require_identifier = make::js_identifier_expression(make::js_reference_identifier(make::ident("require")));
    /// let source_name = make::js_string_literal_expression(make::js_string_literal("foo"));
    /// let call_arguments = make::js_call_arguments(
    ///     make::token(T!['(']),
    ///     make::js_call_argument_list([AnyJsCallArgument::AnyJsExpression(AnyJsExpression::AnyJsLiteralExpression(source_name.into()))], []),
    ///     make::token(T![')']),
    /// );
    /// let require_call_expression = make::js_call_expression(require_identifier.into(), call_arguments).build();
    /// assert!(require_call_expression.is_require_call_expression());
    /// ```
    pub fn is_require_call_expression(&self) -> bool {
        self.callee()
            .ok()
            .and_then(|callee| callee.as_js_reference_identifier()?.value_token().ok())
            .map(|name| name.text_trimmed() == "require")
            .unwrap_or(false)
    }

    /// Returns the inner text of the module specifier
    /// if this call expression represents a call to `require("...")`:
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_syntax::{AnyJsCallArgument, AnyJsExpression, T};
    /// use biome_js_factory::make;
    ///
    /// let require_identifier = make::js_identifier_expression(make::js_reference_identifier(make::ident("require")));
    /// let source_name = make::js_string_literal_expression(make::js_string_literal("foo"));
    /// let call_arguments = make::js_call_arguments(
    ///     make::token(T!['(']),
    ///     make::js_call_argument_list([AnyJsCallArgument::AnyJsExpression(AnyJsExpression::AnyJsLiteralExpression(source_name.into()))], []),
    ///     make::token(T![')']),
    /// );
    /// let require_call_expression = make::js_call_expression(require_identifier.into(), call_arguments).build();
    /// assert_eq!(require_call_expression.imported_module_source_text().unwrap().text(), "foo");
    /// ```
    pub fn imported_module_source_text(&self) -> Option<TokenText> {
        if self.is_require_call_expression() {
            let [Some(argument)] = self.arguments().ok()?.get_arguments_by_index([0])
            else {
                return None;
            };
            argument
                .as_any_js_expression()?
                .as_any_js_literal_expression()?
                .as_js_string_literal_expression()?
                .inner_string_text()
                .ok()
        } else {
            None
        }
    }

    /// Returns the whole token text of the module specifier, with quotes included,
    /// if this call expression represents a call to `require("...")`:
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_syntax::{AnyJsCallArgument, AnyJsExpression, T};
    /// use biome_js_factory::make;
    ///
    /// let require_identifier = make::js_identifier_expression(make::js_reference_identifier(make::ident("require")));
    /// let source_name = make::js_string_literal_expression(make::js_string_literal("foo"));
    /// let call_arguments = make::js_call_arguments(
    ///     make::token(T!['(']),
    ///     make::js_call_argument_list([AnyJsCallArgument::AnyJsExpression(AnyJsExpression::AnyJsLiteralExpression(source_name.into()))], []),
    ///     make::token(T![')']),
    /// );
    /// let require_call_expression = make::js_call_expression(require_identifier.into(), call_arguments).build();
    /// assert_eq!(require_call_expression.imported_module_source_token().unwrap().text(), "\"foo\"");
    /// ```
    pub fn imported_module_source_token(&self) -> Option<SyntaxToken<crate::JsLanguage>> {
        if self.is_require_call_expression() {
            let [Some(argument)] = self.arguments().ok()?.get_arguments_by_index([0])
            else {
                return None;
            };
            argument
                .as_any_js_expression()?
                .as_any_js_literal_expression()?
                .as_js_string_literal_expression()?
                .value_token()
                .ok()
        } else {
            None
        }
    }
}

impl AnyJsImportClause {
    /// Type token of the import clause.
    ///
    /// ```ts
    /// import { type X }
    ///          ^^^^
    /// ```
    pub fn type_token(&self) -> Option<JsSyntaxToken> {
        match self {
            Self::JsImportBareClause(_) => None,
            Self::JsImportDefaultClause(clause) => clause.type_token(),
            Self::JsImportNamedClause(clause) => clause.type_token(),
            Self::JsImportNamespaceClause(clause) => clause.type_token(),
            Self::JsImportCombinedClause(_) => None,
        }
    }

    /// Source of this import clause.
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_syntax::T;
    ///
    /// let source = make::js_module_source(make::js_string_literal("react"));
    /// let binding = make::js_identifier_binding(make::ident("React"));
    /// let specifier = make::js_default_import_specifier(binding.into());
    /// let clause = make::js_import_default_clause(specifier, make::token(T![from]), source.into()).build();
    ///
    /// assert_eq!(clause.source().unwrap().as_js_module_source().unwrap().inner_string_text().unwrap().text(), "react");
    /// ```
    pub fn source(&self) -> SyntaxResult<JsModuleSource> {
        let source = match self {
            Self::JsImportBareClause(clause) => clause.source(),
            Self::JsImportDefaultClause(clause) => clause.source(),
            Self::JsImportNamedClause(clause) => clause.source(),
            Self::JsImportNamespaceClause(clause) => clause.source(),
            Self::JsImportCombinedClause(clause) => clause.source(),
        };

        source.and_then(|source| match source {
            AnyJsModuleSource::JsModuleSource(source) => Ok(source),
            AnyJsModuleSource::JsMetavariable(_) => Err(SyntaxError::UnexpectedMetavariable),
        })
    }

    /// Assertion of this import clause.
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_syntax::T;
    ///
    /// let source = make::js_module_source(make::js_string_literal("react"));
    /// let binding = make::js_identifier_binding(make::ident("React"));
    /// let specifier = make::js_default_import_specifier(binding.into());
    /// let clause = make::js_import_default_clause(specifier, make::token(T![from]), source.into()).build();
    ///
    /// assert_eq!(clause.source().unwrap().as_js_module_source().unwrap().inner_string_text().unwrap().text(), "react");
    /// ```
    pub fn assertion(&self) -> Option<JsImportAssertion> {
        match self {
            Self::JsImportBareClause(clause) => clause.assertion(),
            Self::JsImportDefaultClause(clause) => clause.assertion(),
            Self::JsImportNamedClause(clause) => clause.assertion(),
            Self::JsImportNamespaceClause(clause) => clause.assertion(),
            Self::JsImportCombinedClause(clause) => clause.assertion(),
        }
    }
}

impl AnyJsNamedImportSpecifier {
    /// Type token of the import specifier.
    ///
    /// ```ts
    /// import { type X }
    ///          ^^^^
    /// ```
    pub fn type_token(&self) -> Option<JsSyntaxToken> {
        match self {
            Self::JsBogusNamedImportSpecifier(_) => None,
            Self::JsNamedImportSpecifier(specifier) => specifier.type_token(),
            Self::JsShorthandNamedImportSpecifier(specifier) => specifier.type_token(),
        }
    }

    /// Returns the import clause that includes this specifier.
    pub fn import_clause(&self) -> Option<AnyJsImportClause> {
        AnyJsImportClause::cast(self.syntax().ancestors().nth(3)?)
    }

    /// Returns `true` if this specifier or its import clause has **only** a type modifier.
    pub fn imports_only_types(&self) -> bool {
        self.type_token().is_some() || self.import_clause().and_then(|x| x.type_token()).is_some()
    }

    /// Imported name of this import specifier
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_syntax::AnyJsNamedImportSpecifier;
    ///
    /// let binding = make::js_identifier_binding(make::ident("React"));
    /// let specifier = make::js_shorthand_named_import_specifier(binding.into()).build();
    /// let specifier = AnyJsNamedImportSpecifier::JsShorthandNamedImportSpecifier(specifier);
    ///
    /// assert_eq!(specifier.imported_name().unwrap().text_trimmed(), "React");
    /// ```
    pub fn imported_name(&self) -> Option<JsSyntaxToken> {
        match self {
            specifier @ (Self::JsNamedImportSpecifier(_)
            | Self::JsShorthandNamedImportSpecifier(_)) => specifier
                .local_name()?
                .as_js_identifier_binding()?
                .name_token()
                .ok(),
            Self::JsBogusNamedImportSpecifier(_) => None,
        }
    }

    /// Local name of this import specifier
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_syntax::AnyJsNamedImportSpecifier;
    ///
    /// let binding = make::js_identifier_binding(make::ident("React"));
    /// let specifier = make::js_shorthand_named_import_specifier(binding.into()).build();
    /// let specifier = AnyJsNamedImportSpecifier::JsShorthandNamedImportSpecifier(specifier);
    ///
    /// let name_token = specifier.local_name().unwrap().as_js_identifier_binding().unwrap().name_token();
    /// assert_eq!(name_token.unwrap().text_trimmed(), "React");
    /// ```
    pub fn local_name(&self) -> Option<AnyJsBinding> {
        match self {
            Self::JsBogusNamedImportSpecifier(_) => None,
            Self::JsNamedImportSpecifier(specifier) => specifier.local_name().ok(),
            Self::JsShorthandNamedImportSpecifier(specifier) => specifier.local_name().ok(),
        }
    }

    pub fn with_type_token(self, type_token: Option<JsSyntaxToken>) -> AnyJsNamedImportSpecifier {
        match self {
            Self::JsBogusNamedImportSpecifier(_) => self,
            Self::JsNamedImportSpecifier(specifier) => specifier.with_type_token(type_token).into(),
            Self::JsShorthandNamedImportSpecifier(specifier) => {
                specifier.with_type_token(type_token).into()
            }
        }
    }
}

impl JsModuleSource {
    /// Get the inner text of a string not including the quotes
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_rowan::TriviaPieceKind;
    ///
    /// let source_token = make::js_string_literal("react")
    ///     .with_leading_trivia(vec![(TriviaPieceKind::Whitespace, " ")]);
    /// let source = make::js_module_source(source_token);
    ///
    /// assert_eq!(source.inner_string_text().unwrap().text(), "react");
    /// ```
    pub fn inner_string_text(&self) -> SyntaxResult<TokenText> {
        Ok(inner_string_text(&self.value_token()?))
    }
}

declare_node_union! {
    /// This node union is meant to match the following syntax:
    /// ```js
    ///    import "lodash";
    /// //        ^^^^^^^^
    ///    require("lodash")
    /// // ^^^^^^^^^^^^^^^^^
    ///    import("lodash")
    /// // ^^^^^^^^^^^^^^^^
    /// ```
    pub AnyJsImportSourceLike = JsModuleSource | JsCallExpression |  JsImportCallExpression
}

impl AnyJsImportSourceLike {
    /// Returns the inner text of the specifier:
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_syntax::AnyJsImportSourceLike;
    ///
    /// let source_name = make::js_module_source(make::js_string_literal("foo"));
    /// let any_import_specifier = AnyJsImportSourceLike::JsModuleSource(source_name);
    /// assert_eq!(any_import_specifier.inner_string_text().unwrap().text(), "foo")
    /// ```
    pub fn module_source_text(&self) -> Option<TokenText> {
        match self {
            AnyJsImportSourceLike::JsModuleSource(source) => source.inner_string_text().ok(),
            AnyJsImportSourceLike::JsCallExpression(expression) => {
                expression.imported_module_source_text()
            }
            AnyJsImportSourceLike::JsImportCallExpression(import_call) => {
                import_call.module_source_text()
            }
        }
    }

    /// Returns the whole token text of the specifier, with quotes included:
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_syntax::AnyJsImportSourceLike;
    ///
    /// let source_name = make::js_module_source(make::js_string_literal("foo"));
    /// let any_import_specifier = AnyJsImportSourceLike::JsModuleSource(source_name);
    /// assert_eq!(any_import_specifier.module_name_token().unwrap().text(), "\"foo\"")
    /// ```
    pub fn module_name_token(&self) -> Option<JsSyntaxToken> {
        match self {
            AnyJsImportSourceLike::JsModuleSource(source) => source.value_token().ok(),
            AnyJsImportSourceLike::JsCallExpression(expression) => {
                expression.imported_module_source_token()
            }
            AnyJsImportSourceLike::JsImportCallExpression(import_call) => {
                import_call.module_source_token()
            }
        }
    }

    /// Check whether the js import specifier like is in a ts module declaration:
    ///
    /// ```ts
    /// declare module "abc" {}
    /// ```
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_syntax::{AnyJsImportSourceLike, JsSyntaxKind, JsSyntaxToken};
    ///
    /// let module_token = JsSyntaxToken::new_detached(JsSyntaxKind::MODULE_KW, "module", [], []);
    /// let module_source = make::js_module_source(make::js_string_literal("foo"));
    /// let module_declaration = make::ts_external_module_declaration(module_token, module_source.into()).build();
    /// let any_import_specifier = AnyJsImportSourceLike::JsModuleSource(module_declaration.source().unwrap().as_js_module_source().unwrap().clone());
    /// assert!(any_import_specifier.is_in_ts_module_declaration());
    ///
    /// let module_source = make::js_module_source(make::js_string_literal("bar"));
    /// let any_import_specifier = AnyJsImportSourceLike::JsModuleSource(module_source.into());
    /// assert!(!any_import_specifier.is_in_ts_module_declaration());
    /// ```
    pub fn is_in_ts_module_declaration(&self) -> bool {
        // It first has to be a JsModuleSource
        matches!(self, AnyJsImportSourceLike::JsModuleSource(_))
            && matches!(
                self.syntax().parent().kind(),
                Some(JsSyntaxKind::TS_EXTERNAL_MODULE_DECLARATION)
            )
    }
}

declare_node_union! {
    pub AnyJsImportSpecifier = JsNamedImportSpecifier
        | JsShorthandNamedImportSpecifier
        | JsNamespaceImportSpecifier
        | JsDefaultImportSpecifier
}

impl AnyJsImportSpecifier {
    /// Imported name of this import specifier.
    pub fn local_name(&self) -> SyntaxResult<AnyJsBinding> {
        match self {
            Self::JsNamedImportSpecifier(specifier) => specifier.local_name(),
            Self::JsShorthandNamedImportSpecifier(specifier) => specifier.local_name(),
            Self::JsNamespaceImportSpecifier(specifier) => specifier.local_name(),
            Self::JsDefaultImportSpecifier(specifier) => specifier.local_name(),
        }
    }
}
