//! Registry to `DiagnosticCode` to `DiagnosticKind` mappings.

use std::fmt;

use once_cell::sync::Lazy;
use ruff_macros::DiagnosticCodePrefix;
use rustc_hash::FxHashMap;
use rustpython_parser::ast::Location;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumIter, EnumString};

use crate::ast::types::Range;
use crate::autofix::Fix;
use crate::violation::Violation;
use crate::violations;

macro_rules! define_rule_mapping {
    ($($code:ident => $mod:ident::$name:ident,)+) => {
        #[derive(
            AsRefStr,
            DiagnosticCodePrefix,
            EnumIter,
            EnumString,
            Debug,
            Display,
            PartialEq,
            Eq,
            Clone,
            Serialize,
            Deserialize,
            Hash,
            PartialOrd,
            Ord,
        )]
        pub enum DiagnosticCode {
            $(
                $code,
            )+
        }

        #[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize)]
        pub enum DiagnosticKind {
            $(
                $name($mod::$name),
            )+
        }

        impl DiagnosticCode {
            /// A placeholder representation of the `DiagnosticKind` for the diagnostic.
            pub fn kind(&self) -> DiagnosticKind {
                match self {
                    $(
                        DiagnosticCode::$code => DiagnosticKind::$name(<$mod::$name as Violation>::placeholder()),
                    )+
                }
            }
        }

        impl DiagnosticKind {
            /// A four-letter shorthand code for the diagnostic.
            pub fn code(&self) -> &'static DiagnosticCode {
                match self {
                    $(
                        DiagnosticKind::$name(..) => &DiagnosticCode::$code,
                    )+
                }
            }

            /// The body text for the diagnostic.
            pub fn body(&self) -> String {
                match self {
                    $(
                        DiagnosticKind::$name(x) => Violation::message(x),
                    )+
                }
            }

            /// Whether the check kind is (potentially) fixable.
            pub fn fixable(&self) -> bool {
                match self {
                    $(
                        DiagnosticKind::$name(x) => x.autofix_title_formatter().is_some(),
                    )+
                }
            }

            /// The message used to describe the fix action for a given `DiagnosticKind`.
            pub fn commit(&self) -> Option<String> {
                match self {
                    $(
                        DiagnosticKind::$name(x) => x.autofix_title_formatter().map(|f| f(x)),
                    )+
                }
            }
        }

        $(
            impl From<$mod::$name> for DiagnosticKind {
                fn from(x: $mod::$name) -> Self {
                    DiagnosticKind::$name(x)
                }
            }
        )+

    };
}

define_rule_mapping!(
    // pycodestyle errors
    E401 => violations::MultipleImportsOnOneLine,
    E402 => violations::ModuleImportNotAtTopOfFile,
    E501 => violations::LineTooLong,
    E711 => violations::NoneComparison,
    E712 => violations::TrueFalseComparison,
    E713 => violations::NotInTest,
    E714 => violations::NotIsTest,
    E721 => violations::TypeComparison,
    E722 => violations::DoNotUseBareExcept,
    E731 => violations::DoNotAssignLambda,
    E741 => violations::AmbiguousVariableName,
    E742 => violations::AmbiguousClassName,
    E743 => violations::AmbiguousFunctionName,
    E902 => violations::IOError,
    E999 => violations::SyntaxError,
    // pycodestyle warnings
    W292 => violations::NoNewLineAtEndOfFile,
    W605 => violations::InvalidEscapeSequence,
    // pyflakes
    F401 => violations::UnusedImport,
    F402 => violations::ImportShadowedByLoopVar,
    F403 => violations::ImportStarUsed,
    F404 => violations::LateFutureImport,
    F405 => violations::ImportStarUsage,
    F406 => violations::ImportStarNotPermitted,
    F407 => violations::FutureFeatureNotDefined,
    F501 => violations::PercentFormatInvalidFormat,
    F502 => violations::PercentFormatExpectedMapping,
    F503 => violations::PercentFormatExpectedSequence,
    F504 => violations::PercentFormatExtraNamedArguments,
    F505 => violations::PercentFormatMissingArgument,
    F506 => violations::PercentFormatMixedPositionalAndNamed,
    F507 => violations::PercentFormatPositionalCountMismatch,
    F508 => violations::PercentFormatStarRequiresSequence,
    F509 => violations::PercentFormatUnsupportedFormatCharacter,
    F521 => violations::StringDotFormatInvalidFormat,
    F522 => violations::StringDotFormatExtraNamedArguments,
    F523 => violations::StringDotFormatExtraPositionalArguments,
    F524 => violations::StringDotFormatMissingArguments,
    F525 => violations::StringDotFormatMixingAutomatic,
    F541 => violations::FStringMissingPlaceholders,
    F601 => violations::MultiValueRepeatedKeyLiteral,
    F602 => violations::MultiValueRepeatedKeyVariable,
    F621 => violations::ExpressionsInStarAssignment,
    F622 => violations::TwoStarredExpressions,
    F631 => violations::AssertTuple,
    F632 => violations::IsLiteral,
    F633 => violations::InvalidPrintSyntax,
    F634 => violations::IfTuple,
    F701 => violations::BreakOutsideLoop,
    F702 => violations::ContinueOutsideLoop,
    F704 => violations::YieldOutsideFunction,
    F706 => violations::ReturnOutsideFunction,
    F707 => violations::DefaultExceptNotLast,
    F722 => violations::ForwardAnnotationSyntaxError,
    F811 => violations::RedefinedWhileUnused,
    F821 => violations::UndefinedName,
    F822 => violations::UndefinedExport,
    F823 => violations::UndefinedLocal,
    F841 => violations::UnusedVariable,
    F842 => violations::UnusedAnnotation,
    F901 => violations::RaiseNotImplemented,
    // pylint
    PLC0414 => violations::UselessImportAlias,
    PLC2201 => violations::MisplacedComparisonConstant,
    PLC3002 => violations::UnnecessaryDirectLambdaCall,
    PLE0117 => violations::NonlocalWithoutBinding,
    PLE0118 => violations::UsedPriorGlobalDeclaration,
    PLE1142 => violations::AwaitOutsideAsync,
    PLR0206 => violations::PropertyWithParameters,
    PLR0402 => violations::ConsiderUsingFromImport,
    PLR1701 => violations::ConsiderMergingIsinstance,
    PLR1722 => violations::UseSysExit,
    PLW0120 => violations::UselessElseOnLoop,
    PLW0602 => violations::GlobalVariableNotAssigned,
    // flake8-builtins
    A001 => violations::BuiltinVariableShadowing,
    A002 => violations::BuiltinArgumentShadowing,
    A003 => violations::BuiltinAttributeShadowing,
    // flake8-bugbear
    B002 => violations::UnaryPrefixIncrement,
    B003 => violations::AssignmentToOsEnviron,
    B004 => violations::UnreliableCallableCheck,
    B005 => violations::StripWithMultiCharacters,
    B006 => violations::MutableArgumentDefault,
    B007 => violations::UnusedLoopControlVariable,
    B008 => violations::FunctionCallArgumentDefault,
    B009 => violations::GetAttrWithConstant,
    B010 => violations::SetAttrWithConstant,
    B011 => violations::DoNotAssertFalse,
    B012 => violations::JumpStatementInFinally,
    B013 => violations::RedundantTupleInExceptionHandler,
    B014 => violations::DuplicateHandlerException,
    B015 => violations::UselessComparison,
    B016 => violations::CannotRaiseLiteral,
    B017 => violations::NoAssertRaisesException,
    B018 => violations::UselessExpression,
    B019 => violations::CachedInstanceMethod,
    B020 => violations::LoopVariableOverridesIterator,
    B021 => violations::FStringDocstring,
    B022 => violations::UselessContextlibSuppress,
    B023 => violations::FunctionUsesLoopVariable,
    B024 => violations::AbstractBaseClassWithoutAbstractMethod,
    B025 => violations::DuplicateTryBlockException,
    B026 => violations::StarArgUnpackingAfterKeywordArg,
    B027 => violations::EmptyMethodWithoutAbstractDecorator,
    B904 => violations::RaiseWithoutFromInsideExcept,
    B905 => violations::ZipWithoutExplicitStrict,
    // flake8-blind-except
    BLE001 => violations::BlindExcept,
    // flake8-comprehensions
    C400 => violations::UnnecessaryGeneratorList,
    C401 => violations::UnnecessaryGeneratorSet,
    C402 => violations::UnnecessaryGeneratorDict,
    C403 => violations::UnnecessaryListComprehensionSet,
    C404 => violations::UnnecessaryListComprehensionDict,
    C405 => violations::UnnecessaryLiteralSet,
    C406 => violations::UnnecessaryLiteralDict,
    C408 => violations::UnnecessaryCollectionCall,
    C409 => violations::UnnecessaryLiteralWithinTupleCall,
    C410 => violations::UnnecessaryLiteralWithinListCall,
    C411 => violations::UnnecessaryListCall,
    C413 => violations::UnnecessaryCallAroundSorted,
    C414 => violations::UnnecessaryDoubleCastOrProcess,
    C415 => violations::UnnecessarySubscriptReversal,
    C416 => violations::UnnecessaryComprehension,
    C417 => violations::UnnecessaryMap,
    // flake8-debugger
    T100 => violations::Debugger,
    // mccabe
    C901 => violations::FunctionIsTooComplex,
    // flake8-tidy-imports
    TID251 => violations::BannedApi,
    TID252 => violations::BannedRelativeImport,
    // flake8-return
    RET501 => violations::UnnecessaryReturnNone,
    RET502 => violations::ImplicitReturnValue,
    RET503 => violations::ImplicitReturn,
    RET504 => violations::UnnecessaryAssign,
    RET505 => violations::SuperfluousElseReturn,
    RET506 => violations::SuperfluousElseRaise,
    RET507 => violations::SuperfluousElseContinue,
    RET508 => violations::SuperfluousElseBreak,
    // flake8-implicit-str-concat
    ISC001 => violations::SingleLineImplicitStringConcatenation,
    ISC002 => violations::MultiLineImplicitStringConcatenation,
    ISC003 => violations::ExplicitStringConcatenation,
    // flake8-print
    T201 => violations::PrintFound,
    T203 => violations::PPrintFound,
    // flake8-quotes
    Q000 => violations::BadQuotesInlineString,
    Q001 => violations::BadQuotesMultilineString,
    Q002 => violations::BadQuotesDocstring,
    Q003 => violations::AvoidQuoteEscape,
    // flake8-annotations
    ANN001 => violations::MissingTypeFunctionArgument,
    ANN002 => violations::MissingTypeArgs,
    ANN003 => violations::MissingTypeKwargs,
    ANN101 => violations::MissingTypeSelf,
    ANN102 => violations::MissingTypeCls,
    ANN201 => violations::MissingReturnTypePublicFunction,
    ANN202 => violations::MissingReturnTypePrivateFunction,
    ANN204 => violations::MissingReturnTypeSpecialMethod,
    ANN205 => violations::MissingReturnTypeStaticMethod,
    ANN206 => violations::MissingReturnTypeClassMethod,
    ANN401 => violations::DynamicallyTypedExpression,
    // flake8-2020
    YTT101 => violations::SysVersionSlice3Referenced,
    YTT102 => violations::SysVersion2Referenced,
    YTT103 => violations::SysVersionCmpStr3,
    YTT201 => violations::SysVersionInfo0Eq3Referenced,
    YTT202 => violations::SixPY3Referenced,
    YTT203 => violations::SysVersionInfo1CmpInt,
    YTT204 => violations::SysVersionInfoMinorCmpInt,
    YTT301 => violations::SysVersion0Referenced,
    YTT302 => violations::SysVersionCmpStr10,
    YTT303 => violations::SysVersionSlice1Referenced,
    // flake8-simplify
    SIM101 => violations::DuplicateIsinstanceCall,
    SIM102 => violations::NestedIfStatements,
    SIM103 => violations::ReturnBoolConditionDirectly,
    SIM105 => violations::UseContextlibSuppress,
    SIM107 => violations::ReturnInTryExceptFinally,
    SIM108 => violations::UseTernaryOperator,
    SIM109 => violations::CompareWithTuple,
    SIM110 => violations::ConvertLoopToAny,
    SIM111 => violations::ConvertLoopToAll,
    SIM117 => violations::MultipleWithStatements,
    SIM118 => violations::KeyInDict,
    SIM201 => violations::NegateEqualOp,
    SIM202 => violations::NegateNotEqualOp,
    SIM208 => violations::DoubleNegation,
    SIM210 => violations::IfExprWithTrueFalse,
    SIM211 => violations::IfExprWithFalseTrue,
    SIM212 => violations::IfExprWithTwistedArms,
    SIM220 => violations::AAndNotA,
    SIM221 => violations::AOrNotA,
    SIM222 => violations::OrTrue,
    SIM223 => violations::AndFalse,
    SIM300 => violations::YodaConditions,
    // pyupgrade
    UP001 => violations::UselessMetaclassType,
    UP003 => violations::TypeOfPrimitive,
    UP004 => violations::UselessObjectInheritance,
    UP005 => violations::DeprecatedUnittestAlias,
    UP006 => violations::UsePEP585Annotation,
    UP007 => violations::UsePEP604Annotation,
    UP008 => violations::SuperCallWithParameters,
    UP009 => violations::PEP3120UnnecessaryCodingComment,
    UP010 => violations::UnnecessaryFutureImport,
    UP011 => violations::UnnecessaryLRUCacheParams,
    UP012 => violations::UnnecessaryEncodeUTF8,
    UP013 => violations::ConvertTypedDictFunctionalToClass,
    UP014 => violations::ConvertNamedTupleFunctionalToClass,
    UP015 => violations::RedundantOpenModes,
    UP016 => violations::RemoveSixCompat,
    UP017 => violations::DatetimeTimezoneUTC,
    UP018 => violations::NativeLiterals,
    UP019 => violations::TypingTextStrAlias,
    UP020 => violations::OpenAlias,
    UP021 => violations::ReplaceUniversalNewlines,
    UP022 => violations::ReplaceStdoutStderr,
    UP023 => violations::RewriteCElementTree,
    UP024 => violations::OSErrorAlias,
    UP025 => violations::RewriteUnicodeLiteral,
    UP026 => violations::RewriteMockImport,
    UP027 => violations::RewriteListComprehension,
    UP028 => violations::RewriteYieldFrom,
    UP029 => violations::UnnecessaryBuiltinImport,
    // pydocstyle
    D100 => violations::PublicModule,
    D101 => violations::PublicClass,
    D102 => violations::PublicMethod,
    D103 => violations::PublicFunction,
    D104 => violations::PublicPackage,
    D105 => violations::MagicMethod,
    D106 => violations::PublicNestedClass,
    D107 => violations::PublicInit,
    D200 => violations::FitsOnOneLine,
    D201 => violations::NoBlankLineBeforeFunction,
    D202 => violations::NoBlankLineAfterFunction,
    D203 => violations::OneBlankLineBeforeClass,
    D204 => violations::OneBlankLineAfterClass,
    D205 => violations::BlankLineAfterSummary,
    D206 => violations::IndentWithSpaces,
    D207 => violations::NoUnderIndentation,
    D208 => violations::NoOverIndentation,
    D209 => violations::NewLineAfterLastParagraph,
    D210 => violations::NoSurroundingWhitespace,
    D211 => violations::NoBlankLineBeforeClass,
    D212 => violations::MultiLineSummaryFirstLine,
    D213 => violations::MultiLineSummarySecondLine,
    D214 => violations::SectionNotOverIndented,
    D215 => violations::SectionUnderlineNotOverIndented,
    D300 => violations::UsesTripleQuotes,
    D301 => violations::UsesRPrefixForBackslashedContent,
    D400 => violations::EndsInPeriod,
    D402 => violations::NoSignature,
    D403 => violations::FirstLineCapitalized,
    D404 => violations::NoThisPrefix,
    D405 => violations::CapitalizeSectionName,
    D406 => violations::NewLineAfterSectionName,
    D407 => violations::DashedUnderlineAfterSection,
    D408 => violations::SectionUnderlineAfterName,
    D409 => violations::SectionUnderlineMatchesSectionLength,
    D410 => violations::BlankLineAfterSection,
    D411 => violations::BlankLineBeforeSection,
    D412 => violations::NoBlankLinesBetweenHeaderAndContent,
    D413 => violations::BlankLineAfterLastSection,
    D414 => violations::NonEmptySection,
    D415 => violations::EndsInPunctuation,
    D416 => violations::SectionNameEndsInColon,
    D417 => violations::DocumentAllArguments,
    D418 => violations::SkipDocstring,
    D419 => violations::NonEmpty,
    // pep8-naming
    N801 => violations::InvalidClassName,
    N802 => violations::InvalidFunctionName,
    N803 => violations::InvalidArgumentName,
    N804 => violations::InvalidFirstArgumentNameForClassMethod,
    N805 => violations::InvalidFirstArgumentNameForMethod,
    N806 => violations::NonLowercaseVariableInFunction,
    N807 => violations::DunderFunctionName,
    N811 => violations::ConstantImportedAsNonConstant,
    N812 => violations::LowercaseImportedAsNonLowercase,
    N813 => violations::CamelcaseImportedAsLowercase,
    N814 => violations::CamelcaseImportedAsConstant,
    N815 => violations::MixedCaseVariableInClassScope,
    N816 => violations::MixedCaseVariableInGlobalScope,
    N817 => violations::CamelcaseImportedAsAcronym,
    N818 => violations::ErrorSuffixOnExceptionName,
    // isort
    I001 => violations::UnsortedImports,
    // eradicate
    ERA001 => violations::CommentedOutCode,
    // flake8-bandit
    S101 => violations::AssertUsed,
    S102 => violations::ExecUsed,
    S103 => violations::BadFilePermissions,
    S104 => violations::HardcodedBindAllInterfaces,
    S105 => violations::HardcodedPasswordString,
    S106 => violations::HardcodedPasswordFuncArg,
    S107 => violations::HardcodedPasswordDefault,
    S108 => violations::HardcodedTempFile,
    S113 => violations::RequestWithoutTimeout,
    S324 => violations::HashlibInsecureHashFunction,
    S501 => violations::RequestWithNoCertValidation,
    S506 => violations::UnsafeYAMLLoad,
    // flake8-boolean-trap
    FBT001 => violations::BooleanPositionalArgInFunctionDefinition,
    FBT002 => violations::BooleanDefaultValueInFunctionDefinition,
    FBT003 => violations::BooleanPositionalValueInFunctionCall,
    // flake8-unused-arguments
    ARG001 => violations::UnusedFunctionArgument,
    ARG002 => violations::UnusedMethodArgument,
    ARG003 => violations::UnusedClassMethodArgument,
    ARG004 => violations::UnusedStaticMethodArgument,
    ARG005 => violations::UnusedLambdaArgument,
    // flake8-import-conventions
    ICN001 => violations::ImportAliasIsNotConventional,
    // flake8-datetimez
    DTZ001 => violations::CallDatetimeWithoutTzinfo,
    DTZ002 => violations::CallDatetimeToday,
    DTZ003 => violations::CallDatetimeUtcnow,
    DTZ004 => violations::CallDatetimeUtcfromtimestamp,
    DTZ005 => violations::CallDatetimeNowWithoutTzinfo,
    DTZ006 => violations::CallDatetimeFromtimestamp,
    DTZ007 => violations::CallDatetimeStrptimeWithoutZone,
    DTZ011 => violations::CallDateToday,
    DTZ012 => violations::CallDateFromtimestamp,
    // pygrep-hooks
    PGH001 => violations::NoEval,
    PGH002 => violations::DeprecatedLogWarn,
    PGH003 => violations::BlanketTypeIgnore,
    PGH004 => violations::BlanketNOQA,
    // pandas-vet
    PD002 => violations::UseOfInplaceArgument,
    PD003 => violations::UseOfDotIsNull,
    PD004 => violations::UseOfDotNotNull,
    PD007 => violations::UseOfDotIx,
    PD008 => violations::UseOfDotAt,
    PD009 => violations::UseOfDotIat,
    PD010 => violations::UseOfDotPivotOrUnstack,
    PD011 => violations::UseOfDotValues,
    PD012 => violations::UseOfDotReadTable,
    PD013 => violations::UseOfDotStack,
    PD015 => violations::UseOfPdMerge,
    PD901 => violations::DfIsABadVariableName,
    // flake8-errmsg
    EM101 => violations::RawStringInException,
    EM102 => violations::FStringInException,
    EM103 => violations::DotFormatInException,
    // flake8-pytest-style
    PT001 => violations::IncorrectFixtureParenthesesStyle,
    PT002 => violations::FixturePositionalArgs,
    PT003 => violations::ExtraneousScopeFunction,
    PT004 => violations::MissingFixtureNameUnderscore,
    PT005 => violations::IncorrectFixtureNameUnderscore,
    PT006 => violations::ParametrizeNamesWrongType,
    PT007 => violations::ParametrizeValuesWrongType,
    PT008 => violations::PatchWithLambda,
    PT009 => violations::UnittestAssertion,
    PT010 => violations::RaisesWithoutException,
    PT011 => violations::RaisesTooBroad,
    PT012 => violations::RaisesWithMultipleStatements,
    PT013 => violations::IncorrectPytestImport,
    PT015 => violations::AssertAlwaysFalse,
    PT016 => violations::FailWithoutMessage,
    PT017 => violations::AssertInExcept,
    PT018 => violations::CompositeAssertion,
    PT019 => violations::FixtureParamWithoutValue,
    PT020 => violations::DeprecatedYieldFixture,
    PT021 => violations::FixtureFinalizerCallback,
    PT022 => violations::UselessYieldFixture,
    PT023 => violations::IncorrectMarkParenthesesStyle,
    PT024 => violations::UnnecessaryAsyncioMarkOnFixture,
    PT025 => violations::ErroneousUseFixturesOnFixture,
    PT026 => violations::UseFixturesWithoutParameters,
    // flake8-pie
    PIE790 => violations::NoUnnecessaryPass,
    PIE794 => violations::DupeClassFieldDefinitions,
    PIE807 => violations::PreferListBuiltin,
    // Ruff
    RUF001 => violations::AmbiguousUnicodeCharacterString,
    RUF002 => violations::AmbiguousUnicodeCharacterDocstring,
    RUF003 => violations::AmbiguousUnicodeCharacterComment,
    RUF004 => violations::KeywordArgumentBeforeStarArgument,
    RUF100 => violations::UnusedNOQA,
);

/// The source from which a violation originates.
#[derive(EnumIter, Debug, PartialEq, Eq)]
pub enum DiagnosticOrigin {
    Pyflakes,
    Pycodestyle,
    McCabe,
    Isort,
    Pydocstyle,
    Pyupgrade,
    PEP8Naming,
    Flake82020,
    Flake8Annotations,
    Flake8Bandit,
    Flake8BlindExcept,
    Flake8BooleanTrap,
    Flake8Bugbear,
    Flake8Builtins,
    Flake8Comprehensions,
    Flake8Debugger,
    Flake8ErrMsg,
    Flake8ImplicitStrConcat,
    Flake8ImportConventions,
    Flake8Print,
    Flake8PytestStyle,
    Flake8Quotes,
    Flake8Return,
    Flake8Simplify,
    Flake8TidyImports,
    Flake8UnusedArguments,
    Flake8Datetimez,
    Eradicate,
    PandasVet,
    PygrepHooks,
    Pylint,
    Flake8Pie,
    Ruff,
}

pub enum Platform {
    PyPI,
    GitHub,
}

impl fmt::Display for Platform {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Platform::PyPI => fmt.write_str("PyPI"),
            Platform::GitHub => fmt.write_str("GitHub"),
        }
    }
}

impl DiagnosticOrigin {
    pub fn title(&self) -> &'static str {
        match self {
            DiagnosticOrigin::Eradicate => "eradicate",
            DiagnosticOrigin::Flake82020 => "flake8-2020",
            DiagnosticOrigin::Flake8Annotations => "flake8-annotations",
            DiagnosticOrigin::Flake8Bandit => "flake8-bandit",
            DiagnosticOrigin::Flake8BlindExcept => "flake8-blind-except",
            DiagnosticOrigin::Flake8BooleanTrap => "flake8-boolean-trap",
            DiagnosticOrigin::Flake8Bugbear => "flake8-bugbear",
            DiagnosticOrigin::Flake8Builtins => "flake8-builtins",
            DiagnosticOrigin::Flake8Comprehensions => "flake8-comprehensions",
            DiagnosticOrigin::Flake8Debugger => "flake8-debugger",
            DiagnosticOrigin::Flake8ErrMsg => "flake8-errmsg",
            DiagnosticOrigin::Flake8ImplicitStrConcat => "flake8-implicit-str-concat",
            DiagnosticOrigin::Flake8ImportConventions => "flake8-import-conventions",
            DiagnosticOrigin::Flake8Print => "flake8-print",
            DiagnosticOrigin::Flake8PytestStyle => "flake8-pytest-style",
            DiagnosticOrigin::Flake8Quotes => "flake8-quotes",
            DiagnosticOrigin::Flake8Return => "flake8-return",
            DiagnosticOrigin::Flake8TidyImports => "flake8-tidy-imports",
            DiagnosticOrigin::Flake8Simplify => "flake8-simplify",
            DiagnosticOrigin::Flake8UnusedArguments => "flake8-unused-arguments",
            DiagnosticOrigin::Flake8Datetimez => "flake8-datetimez",
            DiagnosticOrigin::Isort => "isort",
            DiagnosticOrigin::McCabe => "mccabe",
            DiagnosticOrigin::PandasVet => "pandas-vet",
            DiagnosticOrigin::PEP8Naming => "pep8-naming",
            DiagnosticOrigin::Pycodestyle => "pycodestyle",
            DiagnosticOrigin::Pydocstyle => "pydocstyle",
            DiagnosticOrigin::Pyflakes => "Pyflakes",
            DiagnosticOrigin::PygrepHooks => "pygrep-hooks",
            DiagnosticOrigin::Pylint => "Pylint",
            DiagnosticOrigin::Pyupgrade => "pyupgrade",
            DiagnosticOrigin::Flake8Pie => "flake8-pie",
            DiagnosticOrigin::Ruff => "Ruff-specific rules",
        }
    }

    pub fn codes(&self) -> Vec<DiagnosticCodePrefix> {
        match self {
            DiagnosticOrigin::Eradicate => vec![DiagnosticCodePrefix::ERA],
            DiagnosticOrigin::Flake82020 => vec![DiagnosticCodePrefix::YTT],
            DiagnosticOrigin::Flake8Annotations => vec![DiagnosticCodePrefix::ANN],
            DiagnosticOrigin::Flake8Bandit => vec![DiagnosticCodePrefix::S],
            DiagnosticOrigin::Flake8BlindExcept => vec![DiagnosticCodePrefix::BLE],
            DiagnosticOrigin::Flake8BooleanTrap => vec![DiagnosticCodePrefix::FBT],
            DiagnosticOrigin::Flake8Bugbear => vec![DiagnosticCodePrefix::B],
            DiagnosticOrigin::Flake8Builtins => vec![DiagnosticCodePrefix::A],
            DiagnosticOrigin::Flake8Comprehensions => vec![DiagnosticCodePrefix::C4],
            DiagnosticOrigin::Flake8Datetimez => vec![DiagnosticCodePrefix::DTZ],
            DiagnosticOrigin::Flake8Debugger => vec![DiagnosticCodePrefix::T10],
            DiagnosticOrigin::Flake8ErrMsg => vec![DiagnosticCodePrefix::EM],
            DiagnosticOrigin::Flake8ImplicitStrConcat => vec![DiagnosticCodePrefix::ISC],
            DiagnosticOrigin::Flake8ImportConventions => vec![DiagnosticCodePrefix::ICN],
            DiagnosticOrigin::Flake8Print => vec![DiagnosticCodePrefix::T20],
            DiagnosticOrigin::Flake8PytestStyle => vec![DiagnosticCodePrefix::PT],
            DiagnosticOrigin::Flake8Quotes => vec![DiagnosticCodePrefix::Q],
            DiagnosticOrigin::Flake8Return => vec![DiagnosticCodePrefix::RET],
            DiagnosticOrigin::Flake8Simplify => vec![DiagnosticCodePrefix::SIM],
            DiagnosticOrigin::Flake8TidyImports => vec![DiagnosticCodePrefix::TID],
            DiagnosticOrigin::Flake8UnusedArguments => vec![DiagnosticCodePrefix::ARG],
            DiagnosticOrigin::Isort => vec![DiagnosticCodePrefix::I],
            DiagnosticOrigin::McCabe => vec![DiagnosticCodePrefix::C90],
            DiagnosticOrigin::PEP8Naming => vec![DiagnosticCodePrefix::N],
            DiagnosticOrigin::PandasVet => vec![DiagnosticCodePrefix::PD],
            DiagnosticOrigin::Pycodestyle => vec![DiagnosticCodePrefix::E, DiagnosticCodePrefix::W],
            DiagnosticOrigin::Pydocstyle => vec![DiagnosticCodePrefix::D],
            DiagnosticOrigin::Pyflakes => vec![DiagnosticCodePrefix::F],
            DiagnosticOrigin::PygrepHooks => vec![DiagnosticCodePrefix::PGH],
            DiagnosticOrigin::Pylint => vec![
                DiagnosticCodePrefix::PLC,
                DiagnosticCodePrefix::PLE,
                DiagnosticCodePrefix::PLR,
                DiagnosticCodePrefix::PLW,
            ],
            DiagnosticOrigin::Pyupgrade => vec![DiagnosticCodePrefix::UP],
            DiagnosticOrigin::Flake8Pie => vec![DiagnosticCodePrefix::PIE],
            DiagnosticOrigin::Ruff => vec![DiagnosticCodePrefix::RUF],
        }
    }

    pub fn url(&self) -> Option<(&'static str, &'static Platform)> {
        match self {
            DiagnosticOrigin::Eradicate => {
                Some(("https://pypi.org/project/eradicate/2.1.0/", &Platform::PyPI))
            }
            DiagnosticOrigin::Flake82020 => Some((
                "https://pypi.org/project/flake8-2020/1.7.0/",
                &Platform::PyPI,
            )),
            DiagnosticOrigin::Flake8Annotations => Some((
                "https://pypi.org/project/flake8-annotations/2.9.1/",
                &Platform::PyPI,
            )),
            DiagnosticOrigin::Flake8Bandit => Some((
                "https://pypi.org/project/flake8-bandit/4.1.1/",
                &Platform::PyPI,
            )),
            DiagnosticOrigin::Flake8BlindExcept => Some((
                "https://pypi.org/project/flake8-blind-except/0.2.1/",
                &Platform::PyPI,
            )),
            DiagnosticOrigin::Flake8BooleanTrap => Some((
                "https://pypi.org/project/flake8-boolean-trap/0.1.0/",
                &Platform::PyPI,
            )),
            DiagnosticOrigin::Flake8Bugbear => Some((
                "https://pypi.org/project/flake8-bugbear/22.10.27/",
                &Platform::PyPI,
            )),
            DiagnosticOrigin::Flake8Builtins => Some((
                "https://pypi.org/project/flake8-builtins/2.0.1/",
                &Platform::PyPI,
            )),
            DiagnosticOrigin::Flake8Comprehensions => Some((
                "https://pypi.org/project/flake8-comprehensions/3.10.1/",
                &Platform::PyPI,
            )),
            DiagnosticOrigin::Flake8Debugger => Some((
                "https://pypi.org/project/flake8-debugger/4.1.2/",
                &Platform::PyPI,
            )),
            DiagnosticOrigin::Flake8ErrMsg => Some((
                "https://pypi.org/project/flake8-errmsg/0.4.0/",
                &Platform::PyPI,
            )),
            DiagnosticOrigin::Flake8ImplicitStrConcat => Some((
                "https://pypi.org/project/flake8-implicit-str-concat/0.3.0/",
                &Platform::PyPI,
            )),
            DiagnosticOrigin::Flake8ImportConventions => None,
            DiagnosticOrigin::Flake8Print => Some((
                "https://pypi.org/project/flake8-print/5.0.0/",
                &Platform::PyPI,
            )),
            DiagnosticOrigin::Flake8PytestStyle => Some((
                "https://pypi.org/project/flake8-pytest-style/1.6.0/",
                &Platform::PyPI,
            )),
            DiagnosticOrigin::Flake8Quotes => Some((
                "https://pypi.org/project/flake8-quotes/3.3.1/",
                &Platform::PyPI,
            )),
            DiagnosticOrigin::Flake8Return => Some((
                "https://pypi.org/project/flake8-return/1.2.0/",
                &Platform::PyPI,
            )),
            DiagnosticOrigin::Flake8Simplify => Some((
                "https://pypi.org/project/flake8-simplify/0.19.3/",
                &Platform::PyPI,
            )),
            DiagnosticOrigin::Flake8TidyImports => Some((
                "https://pypi.org/project/flake8-tidy-imports/4.8.0/",
                &Platform::PyPI,
            )),
            DiagnosticOrigin::Flake8UnusedArguments => Some((
                "https://pypi.org/project/flake8-unused-arguments/0.0.12/",
                &Platform::PyPI,
            )),
            DiagnosticOrigin::Flake8Datetimez => Some((
                "https://pypi.org/project/flake8-datetimez/20.10.0/",
                &Platform::PyPI,
            )),
            DiagnosticOrigin::Isort => {
                Some(("https://pypi.org/project/isort/5.10.1/", &Platform::PyPI))
            }
            DiagnosticOrigin::McCabe => {
                Some(("https://pypi.org/project/mccabe/0.7.0/", &Platform::PyPI))
            }
            DiagnosticOrigin::PandasVet => Some((
                "https://pypi.org/project/pandas-vet/0.2.3/",
                &Platform::PyPI,
            )),
            DiagnosticOrigin::PEP8Naming => Some((
                "https://pypi.org/project/pep8-naming/0.13.2/",
                &Platform::PyPI,
            )),
            DiagnosticOrigin::Pycodestyle => Some((
                "https://pypi.org/project/pycodestyle/2.9.1/",
                &Platform::PyPI,
            )),
            DiagnosticOrigin::Pydocstyle => Some((
                "https://pypi.org/project/pydocstyle/6.1.1/",
                &Platform::PyPI,
            )),
            DiagnosticOrigin::Pyflakes => {
                Some(("https://pypi.org/project/pyflakes/2.5.0/", &Platform::PyPI))
            }
            DiagnosticOrigin::Pylint => {
                Some(("https://pypi.org/project/pylint/2.15.7/", &Platform::PyPI))
            }
            DiagnosticOrigin::PygrepHooks => Some((
                "https://github.com/pre-commit/pygrep-hooks",
                &Platform::GitHub,
            )),
            DiagnosticOrigin::Pyupgrade => {
                Some(("https://pypi.org/project/pyupgrade/3.2.0/", &Platform::PyPI))
            }
            DiagnosticOrigin::Flake8Pie => Some((
                "https://pypi.org/project/flake8-pie/0.16.0/",
                &Platform::PyPI,
            )),
            DiagnosticOrigin::Ruff => None,
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
pub enum LintSource {
    AST,
    FileSystem,
    Lines,
    Tokens,
    Imports,
    NoQA,
}

impl DiagnosticCode {
    /// The source for the diagnostic (either the AST, the filesystem, or the
    /// physical lines).
    pub fn lint_source(&self) -> &'static LintSource {
        match self {
            DiagnosticCode::RUF100 => &LintSource::NoQA,
            DiagnosticCode::E501
            | DiagnosticCode::W292
            | DiagnosticCode::UP009
            | DiagnosticCode::PGH003
            | DiagnosticCode::PGH004 => &LintSource::Lines,
            DiagnosticCode::ERA001
            | DiagnosticCode::ISC001
            | DiagnosticCode::ISC002
            | DiagnosticCode::Q000
            | DiagnosticCode::Q001
            | DiagnosticCode::Q002
            | DiagnosticCode::Q003
            | DiagnosticCode::W605
            | DiagnosticCode::RUF001
            | DiagnosticCode::RUF002
            | DiagnosticCode::RUF003 => &LintSource::Tokens,
            DiagnosticCode::E902 => &LintSource::FileSystem,
            DiagnosticCode::I001 => &LintSource::Imports,
            _ => &LintSource::AST,
        }
    }

    pub fn category(&self) -> DiagnosticOrigin {
        #[allow(clippy::match_same_arms)]
        match self {
            // flake8-builtins
            DiagnosticCode::A001 => DiagnosticOrigin::Flake8Builtins,
            DiagnosticCode::A002 => DiagnosticOrigin::Flake8Builtins,
            DiagnosticCode::A003 => DiagnosticOrigin::Flake8Builtins,
            // flake8-annotations
            DiagnosticCode::ANN001 => DiagnosticOrigin::Flake8Annotations,
            DiagnosticCode::ANN002 => DiagnosticOrigin::Flake8Annotations,
            DiagnosticCode::ANN003 => DiagnosticOrigin::Flake8Annotations,
            DiagnosticCode::ANN101 => DiagnosticOrigin::Flake8Annotations,
            DiagnosticCode::ANN102 => DiagnosticOrigin::Flake8Annotations,
            DiagnosticCode::ANN201 => DiagnosticOrigin::Flake8Annotations,
            DiagnosticCode::ANN202 => DiagnosticOrigin::Flake8Annotations,
            DiagnosticCode::ANN204 => DiagnosticOrigin::Flake8Annotations,
            DiagnosticCode::ANN205 => DiagnosticOrigin::Flake8Annotations,
            DiagnosticCode::ANN206 => DiagnosticOrigin::Flake8Annotations,
            DiagnosticCode::ANN401 => DiagnosticOrigin::Flake8Annotations,
            // flake8-unused-arguments
            DiagnosticCode::ARG001 => DiagnosticOrigin::Flake8UnusedArguments,
            DiagnosticCode::ARG002 => DiagnosticOrigin::Flake8UnusedArguments,
            DiagnosticCode::ARG003 => DiagnosticOrigin::Flake8UnusedArguments,
            DiagnosticCode::ARG004 => DiagnosticOrigin::Flake8UnusedArguments,
            DiagnosticCode::ARG005 => DiagnosticOrigin::Flake8UnusedArguments,
            // flake8-bugbear
            DiagnosticCode::B002 => DiagnosticOrigin::Flake8Bugbear,
            DiagnosticCode::B003 => DiagnosticOrigin::Flake8Bugbear,
            DiagnosticCode::B004 => DiagnosticOrigin::Flake8Bugbear,
            DiagnosticCode::B005 => DiagnosticOrigin::Flake8Bugbear,
            DiagnosticCode::B006 => DiagnosticOrigin::Flake8Bugbear,
            DiagnosticCode::B007 => DiagnosticOrigin::Flake8Bugbear,
            DiagnosticCode::B008 => DiagnosticOrigin::Flake8Bugbear,
            DiagnosticCode::B009 => DiagnosticOrigin::Flake8Bugbear,
            DiagnosticCode::B010 => DiagnosticOrigin::Flake8Bugbear,
            DiagnosticCode::B011 => DiagnosticOrigin::Flake8Bugbear,
            DiagnosticCode::B012 => DiagnosticOrigin::Flake8Bugbear,
            DiagnosticCode::B013 => DiagnosticOrigin::Flake8Bugbear,
            DiagnosticCode::B014 => DiagnosticOrigin::Flake8Bugbear,
            DiagnosticCode::B015 => DiagnosticOrigin::Flake8Bugbear,
            DiagnosticCode::B016 => DiagnosticOrigin::Flake8Bugbear,
            DiagnosticCode::B017 => DiagnosticOrigin::Flake8Bugbear,
            DiagnosticCode::B018 => DiagnosticOrigin::Flake8Bugbear,
            DiagnosticCode::B019 => DiagnosticOrigin::Flake8Bugbear,
            DiagnosticCode::B020 => DiagnosticOrigin::Flake8Bugbear,
            DiagnosticCode::B021 => DiagnosticOrigin::Flake8Bugbear,
            DiagnosticCode::B022 => DiagnosticOrigin::Flake8Bugbear,
            DiagnosticCode::B023 => DiagnosticOrigin::Flake8Bugbear,
            DiagnosticCode::B024 => DiagnosticOrigin::Flake8Bugbear,
            DiagnosticCode::B025 => DiagnosticOrigin::Flake8Bugbear,
            DiagnosticCode::B026 => DiagnosticOrigin::Flake8Bugbear,
            DiagnosticCode::B027 => DiagnosticOrigin::Flake8Bugbear,
            DiagnosticCode::B904 => DiagnosticOrigin::Flake8Bugbear,
            DiagnosticCode::B905 => DiagnosticOrigin::Flake8Bugbear,
            // flake8-blind-except
            DiagnosticCode::BLE001 => DiagnosticOrigin::Flake8BlindExcept,
            // flake8-comprehensions
            DiagnosticCode::C400 => DiagnosticOrigin::Flake8Comprehensions,
            DiagnosticCode::C401 => DiagnosticOrigin::Flake8Comprehensions,
            DiagnosticCode::C402 => DiagnosticOrigin::Flake8Comprehensions,
            DiagnosticCode::C403 => DiagnosticOrigin::Flake8Comprehensions,
            DiagnosticCode::C404 => DiagnosticOrigin::Flake8Comprehensions,
            DiagnosticCode::C405 => DiagnosticOrigin::Flake8Comprehensions,
            DiagnosticCode::C406 => DiagnosticOrigin::Flake8Comprehensions,
            DiagnosticCode::C408 => DiagnosticOrigin::Flake8Comprehensions,
            DiagnosticCode::C409 => DiagnosticOrigin::Flake8Comprehensions,
            DiagnosticCode::C410 => DiagnosticOrigin::Flake8Comprehensions,
            DiagnosticCode::C411 => DiagnosticOrigin::Flake8Comprehensions,
            DiagnosticCode::C413 => DiagnosticOrigin::Flake8Comprehensions,
            DiagnosticCode::C414 => DiagnosticOrigin::Flake8Comprehensions,
            DiagnosticCode::C415 => DiagnosticOrigin::Flake8Comprehensions,
            DiagnosticCode::C416 => DiagnosticOrigin::Flake8Comprehensions,
            DiagnosticCode::C417 => DiagnosticOrigin::Flake8Comprehensions,
            // mccabe
            DiagnosticCode::C901 => DiagnosticOrigin::McCabe,
            // pydocstyle
            DiagnosticCode::D100 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D101 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D102 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D103 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D104 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D105 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D106 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D107 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D200 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D201 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D202 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D203 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D204 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D205 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D206 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D207 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D208 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D209 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D210 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D211 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D212 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D213 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D214 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D215 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D300 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D301 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D400 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D402 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D403 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D404 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D405 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D406 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D407 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D408 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D409 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D410 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D411 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D412 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D413 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D414 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D415 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D416 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D417 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D418 => DiagnosticOrigin::Pydocstyle,
            DiagnosticCode::D419 => DiagnosticOrigin::Pydocstyle,
            // flake8-datetimez
            DiagnosticCode::DTZ001 => DiagnosticOrigin::Flake8Datetimez,
            DiagnosticCode::DTZ002 => DiagnosticOrigin::Flake8Datetimez,
            DiagnosticCode::DTZ003 => DiagnosticOrigin::Flake8Datetimez,
            DiagnosticCode::DTZ004 => DiagnosticOrigin::Flake8Datetimez,
            DiagnosticCode::DTZ005 => DiagnosticOrigin::Flake8Datetimez,
            DiagnosticCode::DTZ006 => DiagnosticOrigin::Flake8Datetimez,
            DiagnosticCode::DTZ007 => DiagnosticOrigin::Flake8Datetimez,
            DiagnosticCode::DTZ011 => DiagnosticOrigin::Flake8Datetimez,
            DiagnosticCode::DTZ012 => DiagnosticOrigin::Flake8Datetimez,
            // pycodestyle (errors)
            DiagnosticCode::E401 => DiagnosticOrigin::Pycodestyle,
            DiagnosticCode::E402 => DiagnosticOrigin::Pycodestyle,
            DiagnosticCode::E501 => DiagnosticOrigin::Pycodestyle,
            DiagnosticCode::E711 => DiagnosticOrigin::Pycodestyle,
            DiagnosticCode::E712 => DiagnosticOrigin::Pycodestyle,
            DiagnosticCode::E713 => DiagnosticOrigin::Pycodestyle,
            DiagnosticCode::E714 => DiagnosticOrigin::Pycodestyle,
            DiagnosticCode::E721 => DiagnosticOrigin::Pycodestyle,
            DiagnosticCode::E722 => DiagnosticOrigin::Pycodestyle,
            DiagnosticCode::E731 => DiagnosticOrigin::Pycodestyle,
            DiagnosticCode::E741 => DiagnosticOrigin::Pycodestyle,
            DiagnosticCode::E742 => DiagnosticOrigin::Pycodestyle,
            DiagnosticCode::E743 => DiagnosticOrigin::Pycodestyle,
            DiagnosticCode::E902 => DiagnosticOrigin::Pycodestyle,
            DiagnosticCode::E999 => DiagnosticOrigin::Pycodestyle,
            // flake8-errmsg
            DiagnosticCode::EM101 => DiagnosticOrigin::Flake8ErrMsg,
            DiagnosticCode::EM102 => DiagnosticOrigin::Flake8ErrMsg,
            DiagnosticCode::EM103 => DiagnosticOrigin::Flake8ErrMsg,
            // eradicate
            DiagnosticCode::ERA001 => DiagnosticOrigin::Eradicate,
            // pyflakes
            DiagnosticCode::F401 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F402 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F403 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F404 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F405 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F406 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F407 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F501 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F502 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F503 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F504 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F505 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F506 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F507 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F508 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F509 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F521 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F522 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F523 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F524 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F525 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F541 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F601 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F602 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F621 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F622 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F631 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F632 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F633 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F634 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F701 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F702 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F704 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F706 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F707 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F722 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F811 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F821 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F822 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F823 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F841 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F842 => DiagnosticOrigin::Pyflakes,
            DiagnosticCode::F901 => DiagnosticOrigin::Pyflakes,
            // flake8-boolean-trap
            DiagnosticCode::FBT001 => DiagnosticOrigin::Flake8BooleanTrap,
            DiagnosticCode::FBT002 => DiagnosticOrigin::Flake8BooleanTrap,
            DiagnosticCode::FBT003 => DiagnosticOrigin::Flake8BooleanTrap,
            // isort
            DiagnosticCode::I001 => DiagnosticOrigin::Isort,
            // flake8-import-conventions
            DiagnosticCode::ICN001 => DiagnosticOrigin::Flake8ImportConventions,
            // flake8-implicit-str-concat
            DiagnosticCode::ISC001 => DiagnosticOrigin::Flake8ImplicitStrConcat,
            DiagnosticCode::ISC002 => DiagnosticOrigin::Flake8ImplicitStrConcat,
            DiagnosticCode::ISC003 => DiagnosticOrigin::Flake8ImplicitStrConcat,
            // pep8-naming
            DiagnosticCode::N801 => DiagnosticOrigin::PEP8Naming,
            DiagnosticCode::N802 => DiagnosticOrigin::PEP8Naming,
            DiagnosticCode::N803 => DiagnosticOrigin::PEP8Naming,
            DiagnosticCode::N804 => DiagnosticOrigin::PEP8Naming,
            DiagnosticCode::N805 => DiagnosticOrigin::PEP8Naming,
            DiagnosticCode::N806 => DiagnosticOrigin::PEP8Naming,
            DiagnosticCode::N807 => DiagnosticOrigin::PEP8Naming,
            DiagnosticCode::N811 => DiagnosticOrigin::PEP8Naming,
            DiagnosticCode::N812 => DiagnosticOrigin::PEP8Naming,
            DiagnosticCode::N813 => DiagnosticOrigin::PEP8Naming,
            DiagnosticCode::N814 => DiagnosticOrigin::PEP8Naming,
            DiagnosticCode::N815 => DiagnosticOrigin::PEP8Naming,
            DiagnosticCode::N816 => DiagnosticOrigin::PEP8Naming,
            DiagnosticCode::N817 => DiagnosticOrigin::PEP8Naming,
            DiagnosticCode::N818 => DiagnosticOrigin::PEP8Naming,
            // pandas-vet
            DiagnosticCode::PD002 => DiagnosticOrigin::PandasVet,
            DiagnosticCode::PD003 => DiagnosticOrigin::PandasVet,
            DiagnosticCode::PD004 => DiagnosticOrigin::PandasVet,
            DiagnosticCode::PD007 => DiagnosticOrigin::PandasVet,
            DiagnosticCode::PD008 => DiagnosticOrigin::PandasVet,
            DiagnosticCode::PD009 => DiagnosticOrigin::PandasVet,
            DiagnosticCode::PD010 => DiagnosticOrigin::PandasVet,
            DiagnosticCode::PD011 => DiagnosticOrigin::PandasVet,
            DiagnosticCode::PD012 => DiagnosticOrigin::PandasVet,
            DiagnosticCode::PD013 => DiagnosticOrigin::PandasVet,
            DiagnosticCode::PD015 => DiagnosticOrigin::PandasVet,
            DiagnosticCode::PD901 => DiagnosticOrigin::PandasVet,
            // pygrep-hooks
            DiagnosticCode::PGH001 => DiagnosticOrigin::PygrepHooks,
            DiagnosticCode::PGH002 => DiagnosticOrigin::PygrepHooks,
            DiagnosticCode::PGH003 => DiagnosticOrigin::PygrepHooks,
            DiagnosticCode::PGH004 => DiagnosticOrigin::PygrepHooks,
            // pylint
            DiagnosticCode::PLC0414 => DiagnosticOrigin::Pylint,
            DiagnosticCode::PLC2201 => DiagnosticOrigin::Pylint,
            DiagnosticCode::PLC3002 => DiagnosticOrigin::Pylint,
            DiagnosticCode::PLE0117 => DiagnosticOrigin::Pylint,
            DiagnosticCode::PLE0118 => DiagnosticOrigin::Pylint,
            DiagnosticCode::PLE1142 => DiagnosticOrigin::Pylint,
            DiagnosticCode::PLR0206 => DiagnosticOrigin::Pylint,
            DiagnosticCode::PLR0402 => DiagnosticOrigin::Pylint,
            DiagnosticCode::PLR1701 => DiagnosticOrigin::Pylint,
            DiagnosticCode::PLR1722 => DiagnosticOrigin::Pylint,
            DiagnosticCode::PLW0120 => DiagnosticOrigin::Pylint,
            DiagnosticCode::PLW0602 => DiagnosticOrigin::Pylint,
            // flake8-pytest-style
            DiagnosticCode::PT001 => DiagnosticOrigin::Flake8PytestStyle,
            DiagnosticCode::PT002 => DiagnosticOrigin::Flake8PytestStyle,
            DiagnosticCode::PT003 => DiagnosticOrigin::Flake8PytestStyle,
            DiagnosticCode::PT004 => DiagnosticOrigin::Flake8PytestStyle,
            DiagnosticCode::PT005 => DiagnosticOrigin::Flake8PytestStyle,
            DiagnosticCode::PT006 => DiagnosticOrigin::Flake8PytestStyle,
            DiagnosticCode::PT007 => DiagnosticOrigin::Flake8PytestStyle,
            DiagnosticCode::PT008 => DiagnosticOrigin::Flake8PytestStyle,
            DiagnosticCode::PT009 => DiagnosticOrigin::Flake8PytestStyle,
            DiagnosticCode::PT010 => DiagnosticOrigin::Flake8PytestStyle,
            DiagnosticCode::PT011 => DiagnosticOrigin::Flake8PytestStyle,
            DiagnosticCode::PT012 => DiagnosticOrigin::Flake8PytestStyle,
            DiagnosticCode::PT013 => DiagnosticOrigin::Flake8PytestStyle,
            DiagnosticCode::PT015 => DiagnosticOrigin::Flake8PytestStyle,
            DiagnosticCode::PT016 => DiagnosticOrigin::Flake8PytestStyle,
            DiagnosticCode::PT017 => DiagnosticOrigin::Flake8PytestStyle,
            DiagnosticCode::PT018 => DiagnosticOrigin::Flake8PytestStyle,
            DiagnosticCode::PT019 => DiagnosticOrigin::Flake8PytestStyle,
            DiagnosticCode::PT020 => DiagnosticOrigin::Flake8PytestStyle,
            DiagnosticCode::PT021 => DiagnosticOrigin::Flake8PytestStyle,
            DiagnosticCode::PT022 => DiagnosticOrigin::Flake8PytestStyle,
            DiagnosticCode::PT023 => DiagnosticOrigin::Flake8PytestStyle,
            DiagnosticCode::PT024 => DiagnosticOrigin::Flake8PytestStyle,
            DiagnosticCode::PT025 => DiagnosticOrigin::Flake8PytestStyle,
            DiagnosticCode::PT026 => DiagnosticOrigin::Flake8PytestStyle,
            // flake8-quotes
            DiagnosticCode::Q000 => DiagnosticOrigin::Flake8Quotes,
            DiagnosticCode::Q001 => DiagnosticOrigin::Flake8Quotes,
            DiagnosticCode::Q002 => DiagnosticOrigin::Flake8Quotes,
            DiagnosticCode::Q003 => DiagnosticOrigin::Flake8Quotes,
            // flake8-return
            DiagnosticCode::RET501 => DiagnosticOrigin::Flake8Return,
            DiagnosticCode::RET502 => DiagnosticOrigin::Flake8Return,
            DiagnosticCode::RET503 => DiagnosticOrigin::Flake8Return,
            DiagnosticCode::RET504 => DiagnosticOrigin::Flake8Return,
            DiagnosticCode::RET505 => DiagnosticOrigin::Flake8Return,
            DiagnosticCode::RET506 => DiagnosticOrigin::Flake8Return,
            DiagnosticCode::RET507 => DiagnosticOrigin::Flake8Return,
            DiagnosticCode::RET508 => DiagnosticOrigin::Flake8Return,
            // flake8-bandit
            DiagnosticCode::S101 => DiagnosticOrigin::Flake8Bandit,
            DiagnosticCode::S102 => DiagnosticOrigin::Flake8Bandit,
            DiagnosticCode::S103 => DiagnosticOrigin::Flake8Bandit,
            DiagnosticCode::S104 => DiagnosticOrigin::Flake8Bandit,
            DiagnosticCode::S105 => DiagnosticOrigin::Flake8Bandit,
            DiagnosticCode::S106 => DiagnosticOrigin::Flake8Bandit,
            DiagnosticCode::S107 => DiagnosticOrigin::Flake8Bandit,
            DiagnosticCode::S108 => DiagnosticOrigin::Flake8Bandit,
            DiagnosticCode::S113 => DiagnosticOrigin::Flake8Bandit,
            DiagnosticCode::S324 => DiagnosticOrigin::Flake8Bandit,
            DiagnosticCode::S501 => DiagnosticOrigin::Flake8Bandit,
            DiagnosticCode::S506 => DiagnosticOrigin::Flake8Bandit,
            // flake8-simplify
            DiagnosticCode::SIM103 => DiagnosticOrigin::Flake8Simplify,
            DiagnosticCode::SIM101 => DiagnosticOrigin::Flake8Simplify,
            DiagnosticCode::SIM102 => DiagnosticOrigin::Flake8Simplify,
            DiagnosticCode::SIM105 => DiagnosticOrigin::Flake8Simplify,
            DiagnosticCode::SIM107 => DiagnosticOrigin::Flake8Simplify,
            DiagnosticCode::SIM108 => DiagnosticOrigin::Flake8Simplify,
            DiagnosticCode::SIM109 => DiagnosticOrigin::Flake8Simplify,
            DiagnosticCode::SIM110 => DiagnosticOrigin::Flake8Simplify,
            DiagnosticCode::SIM111 => DiagnosticOrigin::Flake8Simplify,
            DiagnosticCode::SIM117 => DiagnosticOrigin::Flake8Simplify,
            DiagnosticCode::SIM118 => DiagnosticOrigin::Flake8Simplify,
            DiagnosticCode::SIM201 => DiagnosticOrigin::Flake8Simplify,
            DiagnosticCode::SIM202 => DiagnosticOrigin::Flake8Simplify,
            DiagnosticCode::SIM208 => DiagnosticOrigin::Flake8Simplify,
            DiagnosticCode::SIM210 => DiagnosticOrigin::Flake8Simplify,
            DiagnosticCode::SIM211 => DiagnosticOrigin::Flake8Simplify,
            DiagnosticCode::SIM212 => DiagnosticOrigin::Flake8Simplify,
            DiagnosticCode::SIM220 => DiagnosticOrigin::Flake8Simplify,
            DiagnosticCode::SIM221 => DiagnosticOrigin::Flake8Simplify,
            DiagnosticCode::SIM222 => DiagnosticOrigin::Flake8Simplify,
            DiagnosticCode::SIM223 => DiagnosticOrigin::Flake8Simplify,
            DiagnosticCode::SIM300 => DiagnosticOrigin::Flake8Simplify,
            // flake8-debugger
            DiagnosticCode::T100 => DiagnosticOrigin::Flake8Debugger,
            // flake8-print
            DiagnosticCode::T201 => DiagnosticOrigin::Flake8Print,
            DiagnosticCode::T203 => DiagnosticOrigin::Flake8Print,
            // flake8-tidy-imports
            DiagnosticCode::TID251 => DiagnosticOrigin::Flake8TidyImports,
            DiagnosticCode::TID252 => DiagnosticOrigin::Flake8TidyImports,
            // pyupgrade
            DiagnosticCode::UP001 => DiagnosticOrigin::Pyupgrade,
            DiagnosticCode::UP003 => DiagnosticOrigin::Pyupgrade,
            DiagnosticCode::UP004 => DiagnosticOrigin::Pyupgrade,
            DiagnosticCode::UP005 => DiagnosticOrigin::Pyupgrade,
            DiagnosticCode::UP006 => DiagnosticOrigin::Pyupgrade,
            DiagnosticCode::UP007 => DiagnosticOrigin::Pyupgrade,
            DiagnosticCode::UP008 => DiagnosticOrigin::Pyupgrade,
            DiagnosticCode::UP009 => DiagnosticOrigin::Pyupgrade,
            DiagnosticCode::UP010 => DiagnosticOrigin::Pyupgrade,
            DiagnosticCode::UP011 => DiagnosticOrigin::Pyupgrade,
            DiagnosticCode::UP012 => DiagnosticOrigin::Pyupgrade,
            DiagnosticCode::UP013 => DiagnosticOrigin::Pyupgrade,
            DiagnosticCode::UP014 => DiagnosticOrigin::Pyupgrade,
            DiagnosticCode::UP015 => DiagnosticOrigin::Pyupgrade,
            DiagnosticCode::UP016 => DiagnosticOrigin::Pyupgrade,
            DiagnosticCode::UP017 => DiagnosticOrigin::Pyupgrade,
            DiagnosticCode::UP018 => DiagnosticOrigin::Pyupgrade,
            DiagnosticCode::UP019 => DiagnosticOrigin::Pyupgrade,
            DiagnosticCode::UP020 => DiagnosticOrigin::Pyupgrade,
            DiagnosticCode::UP021 => DiagnosticOrigin::Pyupgrade,
            DiagnosticCode::UP022 => DiagnosticOrigin::Pyupgrade,
            DiagnosticCode::UP023 => DiagnosticOrigin::Pyupgrade,
            DiagnosticCode::UP024 => DiagnosticOrigin::Pyupgrade,
            DiagnosticCode::UP025 => DiagnosticOrigin::Pyupgrade,
            DiagnosticCode::UP026 => DiagnosticOrigin::Pyupgrade,
            DiagnosticCode::UP027 => DiagnosticOrigin::Pyupgrade,
            DiagnosticCode::UP028 => DiagnosticOrigin::Pyupgrade,
            DiagnosticCode::UP029 => DiagnosticOrigin::Pyupgrade,
            // pycodestyle (warnings)
            DiagnosticCode::W292 => DiagnosticOrigin::Pycodestyle,
            DiagnosticCode::W605 => DiagnosticOrigin::Pycodestyle,
            // flake8-2020
            DiagnosticCode::YTT101 => DiagnosticOrigin::Flake82020,
            DiagnosticCode::YTT102 => DiagnosticOrigin::Flake82020,
            DiagnosticCode::YTT103 => DiagnosticOrigin::Flake82020,
            DiagnosticCode::YTT201 => DiagnosticOrigin::Flake82020,
            DiagnosticCode::YTT202 => DiagnosticOrigin::Flake82020,
            DiagnosticCode::YTT203 => DiagnosticOrigin::Flake82020,
            DiagnosticCode::YTT204 => DiagnosticOrigin::Flake82020,
            DiagnosticCode::YTT301 => DiagnosticOrigin::Flake82020,
            DiagnosticCode::YTT302 => DiagnosticOrigin::Flake82020,
            DiagnosticCode::YTT303 => DiagnosticOrigin::Flake82020,
            // flake8-pie
            DiagnosticCode::PIE790 => DiagnosticOrigin::Flake8Pie,
            DiagnosticCode::PIE794 => DiagnosticOrigin::Flake8Pie,
            DiagnosticCode::PIE807 => DiagnosticOrigin::Flake8Pie,
            // Ruff
            DiagnosticCode::RUF001 => DiagnosticOrigin::Ruff,
            DiagnosticCode::RUF002 => DiagnosticOrigin::Ruff,
            DiagnosticCode::RUF003 => DiagnosticOrigin::Ruff,
            DiagnosticCode::RUF004 => DiagnosticOrigin::Ruff,
            DiagnosticCode::RUF100 => DiagnosticOrigin::Ruff,
        }
    }
}

impl DiagnosticKind {
    /// The summary text for the diagnostic. Typically a truncated form of the
    /// body text.
    pub fn summary(&self) -> String {
        match self {
            DiagnosticKind::UnaryPrefixIncrement(..) => {
                "Python does not support the unary prefix increment".to_string()
            }
            DiagnosticKind::UnusedLoopControlVariable(violations::UnusedLoopControlVariable(
                name,
            )) => {
                format!("Loop control variable `{name}` not used within the loop body")
            }
            DiagnosticKind::NoAssertRaisesException(..) => {
                "`assertRaises(Exception)` should be considered evil".to_string()
            }
            DiagnosticKind::StarArgUnpackingAfterKeywordArg(..) => {
                "Star-arg unpacking after a keyword argument is strongly discouraged".to_string()
            }

            // flake8-datetimez
            DiagnosticKind::CallDatetimeToday(..) => {
                "The use of `datetime.datetime.today()` is not allowed".to_string()
            }
            DiagnosticKind::CallDatetimeUtcnow(..) => {
                "The use of `datetime.datetime.utcnow()` is not allowed".to_string()
            }
            DiagnosticKind::CallDatetimeUtcfromtimestamp(..) => {
                "The use of `datetime.datetime.utcfromtimestamp()` is not allowed".to_string()
            }
            DiagnosticKind::CallDateToday(..) => {
                "The use of `datetime.date.today()` is not allowed.".to_string()
            }
            DiagnosticKind::CallDateFromtimestamp(..) => {
                "The use of `datetime.date.fromtimestamp()` is not allowed".to_string()
            }
            _ => self.body(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub location: Location,
    pub end_location: Location,
    pub fix: Option<Fix>,
    pub parent: Option<Location>,
}

impl Diagnostic {
    pub fn new<K: Into<DiagnosticKind>>(kind: K, range: Range) -> Self {
        Self {
            kind: kind.into(),
            location: range.location,
            end_location: range.end_location,
            fix: None,
            parent: None,
        }
    }

    pub fn amend(&mut self, fix: Fix) -> &mut Self {
        self.fix = Some(fix);
        self
    }

    pub fn parent(&mut self, parent: Location) -> &mut Self {
        self.parent = Some(parent);
        self
    }
}

/// Pairs of checks that shouldn't be enabled together.
pub const INCOMPATIBLE_CODES: &[(DiagnosticCode, DiagnosticCode, &str)] = &[(
    DiagnosticCode::D203,
    DiagnosticCode::D211,
    "`D203` (OneBlankLineBeforeClass) and `D211` (NoBlankLinesBeforeClass) are incompatible. \
     Consider adding `D203` to `ignore`.",
)];

/// A hash map from deprecated to latest `DiagnosticCode`.
pub static CODE_REDIRECTS: Lazy<FxHashMap<&'static str, DiagnosticCode>> = Lazy::new(|| {
    FxHashMap::from_iter([
        // TODO(charlie): Remove by 2023-01-01.
        ("U001", DiagnosticCode::UP001),
        ("U003", DiagnosticCode::UP003),
        ("U004", DiagnosticCode::UP004),
        ("U005", DiagnosticCode::UP005),
        ("U006", DiagnosticCode::UP006),
        ("U007", DiagnosticCode::UP007),
        ("U008", DiagnosticCode::UP008),
        ("U009", DiagnosticCode::UP009),
        ("U010", DiagnosticCode::UP010),
        ("U011", DiagnosticCode::UP011),
        ("U012", DiagnosticCode::UP012),
        ("U013", DiagnosticCode::UP013),
        ("U014", DiagnosticCode::UP014),
        ("U015", DiagnosticCode::UP015),
        ("U016", DiagnosticCode::UP016),
        ("U017", DiagnosticCode::UP017),
        ("U019", DiagnosticCode::UP019),
        // TODO(charlie): Remove by 2023-02-01.
        ("I252", DiagnosticCode::TID252),
        ("M001", DiagnosticCode::RUF100),
        // TODO(charlie): Remove by 2023-02-01.
        ("PDV002", DiagnosticCode::PD002),
        ("PDV003", DiagnosticCode::PD003),
        ("PDV004", DiagnosticCode::PD004),
        ("PDV007", DiagnosticCode::PD007),
        ("PDV008", DiagnosticCode::PD008),
        ("PDV009", DiagnosticCode::PD009),
        ("PDV010", DiagnosticCode::PD010),
        ("PDV011", DiagnosticCode::PD011),
        ("PDV012", DiagnosticCode::PD012),
        ("PDV013", DiagnosticCode::PD013),
        ("PDV015", DiagnosticCode::PD015),
        ("PDV901", DiagnosticCode::PD901),
        // TODO(charlie): Remove by 2023-02-01.
        ("R501", DiagnosticCode::RET501),
        ("R502", DiagnosticCode::RET502),
        ("R503", DiagnosticCode::RET503),
        ("R504", DiagnosticCode::RET504),
        ("R505", DiagnosticCode::RET505),
        ("R506", DiagnosticCode::RET506),
        ("R507", DiagnosticCode::RET507),
        ("R508", DiagnosticCode::RET508),
        // TODO(charlie): Remove by 2023-02-01.
        ("IC001", DiagnosticCode::ICN001),
        ("IC002", DiagnosticCode::ICN001),
        ("IC003", DiagnosticCode::ICN001),
        ("IC004", DiagnosticCode::ICN001),
    ])
});

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use strum::IntoEnumIterator;

    use crate::registry::DiagnosticCode;

    #[test]
    fn check_code_serialization() {
        for check_code in DiagnosticCode::iter() {
            assert!(
                DiagnosticCode::from_str(check_code.as_ref()).is_ok(),
                "{check_code:?} could not be round-trip serialized."
            );
        }
    }

    #[test]
    fn fixable_codes() {
        for check_code in DiagnosticCode::iter() {
            let kind = check_code.kind();
            if kind.fixable() {
                assert!(
                    kind.commit().is_some(),
                    "{check_code:?} is fixable but has no commit message."
                );
            }
        }
    }
}
