pub mod checks;
mod helpers;
pub mod plugins;
pub mod settings;

#[cfg(test)]
mod tests {
    use std::convert::AsRef;
    use std::path::Path;

    use anyhow::Result;
    use test_case::test_case;

    use crate::linter::test_path;
    use crate::registry::DiagnosticCode;
    use crate::settings;

    #[test_case(DiagnosticCode::N801, Path::new("N801.py"); "N801")]
    #[test_case(DiagnosticCode::N802, Path::new("N802.py"); "N802")]
    #[test_case(DiagnosticCode::N803, Path::new("N803.py"); "N803")]
    #[test_case(DiagnosticCode::N804, Path::new("N804.py"); "N804")]
    #[test_case(DiagnosticCode::N805, Path::new("N805.py"); "N805")]
    #[test_case(DiagnosticCode::N806, Path::new("N806.py"); "N806")]
    #[test_case(DiagnosticCode::N807, Path::new("N807.py"); "N807")]
    #[test_case(DiagnosticCode::N811, Path::new("N811.py"); "N811")]
    #[test_case(DiagnosticCode::N812, Path::new("N812.py"); "N812")]
    #[test_case(DiagnosticCode::N813, Path::new("N813.py"); "N813")]
    #[test_case(DiagnosticCode::N814, Path::new("N814.py"); "N814")]
    #[test_case(DiagnosticCode::N815, Path::new("N815.py"); "N815")]
    #[test_case(DiagnosticCode::N816, Path::new("N816.py"); "N816")]
    #[test_case(DiagnosticCode::N817, Path::new("N817.py"); "N817")]
    #[test_case(DiagnosticCode::N818, Path::new("N818.py"); "N818")]
    fn checks(check_code: DiagnosticCode, path: &Path) -> Result<()> {
        let snapshot = format!("{}_{}", check_code.as_ref(), path.to_string_lossy());
        let checks = test_path(
            Path::new("./resources/test/fixtures/pep8_naming")
                .join(path)
                .as_path(),
            &settings::Settings::for_rule(check_code),
        )?;
        insta::assert_yaml_snapshot!(snapshot, checks);
        Ok(())
    }
}
