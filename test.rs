#![cfg(test)]

use super::*;
use soroban_sdk::{Env, Symbol};

fn s(env: &Env, text: &str) -> Symbol {
    Symbol::new(env, text)
}

#[test]
fn test_initialize() {
    let env = Env::default();
    LibraryContract::initialize(env.clone());
    assert_eq!(LibraryContract::count_books(env.clone()), 0);
}

#[test]
fn test_add_and_list() {
    let env = Env::default();
    LibraryContract::initialize(env.clone());
    LibraryContract::add_book(env.clone(), s(&env, "1984"), s(&env, "Orwell"), 1949);

    let books = LibraryContract::list_books(env.clone());
    assert_eq!(books.len(), 1);
    let b = books.get(0).unwrap();
    assert_eq!(b.title, s(&env, "1984"));
    assert_eq!(b.author, s(&env, "Orwell"));
    assert_eq!(b.year, 1949);
}

#[test]
fn test_find_book() {
    let env = Env::default();
    LibraryContract::initialize(env.clone());
    LibraryContract::add_book(env.clone(), s(&env, "Brave"), s(&env, "Huxley"), 1932);

    let found = LibraryContract::find_book(env.clone(), s(&env, "Brave"));
    assert!(found.is_some());
    let book = found.unwrap();
    assert_eq!(book.author, s(&env, "Huxley"));
}

#[test]
fn test_remove_book() {
    let env = Env::default();
    LibraryContract::initialize(env.clone());
    LibraryContract::add_book(env.clone(), s(&env, "1984"), s(&env, "Orwell"), 1949);
    assert_eq!(LibraryContract::count_books(env.clone()), 1);

    LibraryContract::remove_book(env.clone(), s(&env, "1984"));
    assert_eq!(LibraryContract::count_books(env.clone()), 0);
}

#[test]
fn test_duplicate_titles() {
    let env = Env::default();
    LibraryContract::initialize(env.clone());
    LibraryContract::add_book(env.clone(), s(&env, "A"), s(&env, "X"), 2000);
    LibraryContract::add_book(env.clone(), s(&env, "A"), s(&env, "Y"), 2001);

    assert_eq!(LibraryContract::count_books(env.clone()), 2);
    LibraryContract::remove_book(env.clone(), s(&env, "A"));
    assert_eq!(LibraryContract::count_books(env.clone()), 1);
}
