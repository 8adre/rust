// ignore-tidy-linelength
#![feature(min_const_generics)]
#![crate_name = "foo"]

// @has foo/type.CellIndex.html '//pre[@class="rust typedef"]' 'type CellIndex<const D: usize> = [i64; D];'
pub type CellIndex<const D: usize> = [i64; D];
