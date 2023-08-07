use lambda_calculus::{
    parse,
    parser::ParseError,
    term::Notation::{Classic, DeBruijn},
};

#[test]
fn parse_debruijn_and_classic() -> Result<(), ParseError> {
    for (dbr, cla) in [
        ("12", "a b"),
        ("λλ21", "λs. λz. s z"),
        (
            "(λλλλ42(321))(λλ1)(λλ21)9A",
            "(λm.λn.λs.λz. m s (n s z)) (λs.λz. z) (λs.λz. s z) s z",
        ),
    ] {
        let term_dbr = parse(dbr, DeBruijn)?;
        let term_cla = parse(cla, Classic)?;
        assert_eq!(term_dbr, term_cla);
    }
    Ok(())
}
