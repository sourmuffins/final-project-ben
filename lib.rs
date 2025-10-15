#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, Symbol, Vec};

const LIBRARY_KEY: Symbol = symbol_short!("LIBRARY");

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Book {
    pub title: Symbol,
    pub author: Symbol,
    pub year: u32,
}

#[contracttype]
#[derive(Clone)]
pub struct Library {
    pub books: Vec<Book>,
}

#[contract]
pub struct LibraryContract;

#[contractimpl]
impl LibraryContract {
    /// Initialize an empty library
    pub fn initialize(env: Env) {
        let lib = Library {
            books: Vec::new(&env),
        };
        env.storage().persistent().set(&LIBRARY_KEY, &lib);
    }

    /// Add a new book
    pub fn add_book(env: Env, title: Symbol, author: Symbol, year: u32) {
        let mut lib = Self::get_library(&env);
        let book = Book { title, author, year };
        lib.books.push_back(book);
        env.storage().persistent().set(&LIBRARY_KEY, &lib);
    }

    /// Remove a book by title
    pub fn remove_book(env: Env, title: Symbol) {
        let mut lib = Self::get_library(&env);
        if let Some(index) = lib.books.iter().position(|b| b.title == title) {
            lib.books.remove(index as u32);
            env.storage().persistent().set(&LIBRARY_KEY, &lib);
        }
    }

    /// Find a book
    pub fn find_book(env: Env, title: Symbol) -> Option<Book> {
        let lib = Self::get_library(&env);
        lib.books
            .into_iter()
            .find(|b| b.title == title)
    }

    /// List all books
    pub fn list_books(env: Env) -> Vec<Book> {
        let lib = Self::get_library(&env);
        lib.books
    }

    /// Count books
    pub fn count_books(env: Env) -> u32 {
        let lib = Self::get_library(&env);
        lib.books.len() as u32
    }

    // Helper to load library or create a new one
    fn get_library(env: &Env) -> Library {
        env.storage()
            .persistent()
            .get::<Symbol, Library>(&LIBRARY_KEY)
            .unwrap_or_else(|| Library {
                books: Vec::new(env),
            })
    }
}
