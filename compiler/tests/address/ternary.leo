function main(s: bool, c: address) {
    let a = address(aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8sta57j8);
    let b = address(aleo18qgam03qe483tdrcc3fkqwpp38ehff4a2xma6lu7hams6lfpgcpq3dq05r);

    let r = if s? a: b;

    console.assert(r == c);
}