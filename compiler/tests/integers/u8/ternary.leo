function main(s: bool, a: u8, b: u8, c: u8) {
    let r = if s ? a : b;

    console.assert(r == c);
}