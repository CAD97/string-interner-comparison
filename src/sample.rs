use argh::FromArgs;
use parity_wordlist::WORDS;
use parse_display::{Display, FromStr};

#[derive(FromArgs)]
/// Run sample code
#[argh(subcommand, name = "sample")]
pub struct Sample {
    #[argh(option)]
    /// which library to use
    lib: Lib,
}

#[derive(Display, FromStr)]
#[display(style = "snake_case")]
enum Lib {
    Std,
    InternerBucket,
    InternerString,
    Lasso,
    Lalrpop,
    Intaglio,
    IntaglioDyn,
    Cargo,
    StrenaNew,
    StrenaWithCapacity,
}

impl Sample {
    pub fn run(self) {
        match self.lib {
            Lib::Std => self.std_collect_words(),
            Lib::InternerBucket => self.interner_bucket_collect_words(),
            Lib::InternerString => self.interner_string_collect_words(),
            Lib::Lasso => self.lasso_collect_words(),
            Lib::Lalrpop => self.lalrpop_collect_words(),
            Lib::Intaglio => self.intaglio_collect_words(),
            Lib::IntaglioDyn => self.intaglio_dyn_collect_words(),
            Lib::Cargo => self.cargo_collect_words(),
            Lib::StrenaNew => self.strena_new_collect_words(),
            Lib::StrenaWithCapacity => self.strena_with_capacity_collect_words(),
        }
    }

    fn std_collect_words(&self) {
        crate::ALLOCATOR.set_active(true);
        let mut words: Vec<String> = Vec::new();
        crate::ALLOCATOR.mark_point();
        for &word in WORDS {
            words.push(word.into());
            crate::ALLOCATOR.mark_point();
        }
        crate::ALLOCATOR.set_active(false);
        println!("Loaded {} words", words.len());
    }

    fn interner_bucket_collect_words(&self) {
        crate::ALLOCATOR.set_active(true);
        let mut words = string_interner::StringInterner::default();
        crate::ALLOCATOR.mark_point();
        for &word in WORDS {
            words.get_or_intern(word);
            crate::ALLOCATOR.mark_point();
        }
        crate::ALLOCATOR.set_active(false);
        println!("Loaded {} words", words.len());
    }

    fn interner_string_collect_words(&self) {
        use string_interner::{StringInterner, DefaultSymbol, backend::StringBackend};
        crate::ALLOCATOR.set_active(true);
        let mut words = <StringInterner<DefaultSymbol, StringBackend<DefaultSymbol>>>::new();
        crate::ALLOCATOR.mark_point();
        for &word in WORDS {
            words.get_or_intern(word);
            crate::ALLOCATOR.mark_point();
        }
        crate::ALLOCATOR.set_active(false);
        println!("Loaded {} words", words.len());
    }

    fn lasso_collect_words(&self) {
        crate::ALLOCATOR.set_active(true);
        let mut words: lasso::Rodeo = lasso::Rodeo::new();
        crate::ALLOCATOR.mark_point();
        for &word in WORDS {
            words.get_or_intern(word);
            crate::ALLOCATOR.mark_point();
        }
        crate::ALLOCATOR.set_active(false);
        println!("Loaded {} words", words.len());
    }

    fn lalrpop_collect_words(&self) {
        // NB: lalrpop_intern is an exclusively global interner
        crate::ALLOCATOR.set_active(true);
        crate::ALLOCATOR.mark_point();
        for &word in WORDS {
            lalrpop_intern::intern(word);
            crate::ALLOCATOR.mark_point();
        }
        crate::ALLOCATOR.set_active(false);
        println!("Loaded {} words", WORDS.len());
    }

    fn intaglio_collect_words(&self) {
        crate::ALLOCATOR.set_active(true);
        let mut words = intaglio::SymbolTable::new();
        crate::ALLOCATOR.mark_point();
        for &word in WORDS {
            words.intern(word).unwrap();
            crate::ALLOCATOR.mark_point();
        }
        crate::ALLOCATOR.set_active(false);
        println!("Loaded {} words", words.len());
    }

    fn intaglio_dyn_collect_words<'a>(&'a self) {
        crate::ALLOCATOR.set_active(true);
        let mut words = intaglio::SymbolTable::with_capacity(0);
        crate::ALLOCATOR.mark_point();
        for &word in WORDS {
            words.intern(String::from(word)).unwrap();
            crate::ALLOCATOR.mark_point();
        }
        crate::ALLOCATOR.set_active(false);
        println!("Loaded {} words", words.len());
    }

    fn cargo_collect_words(&self) {
        crate::ALLOCATOR.set_active(true);
        crate::ALLOCATOR.mark_point();
        for &word in WORDS {
            cargo::core::InternedString::new(word);
            crate::ALLOCATOR.mark_point();
        }
        crate::ALLOCATOR.set_active(false);
        println!("Loaded {} words", WORDS.len());
    }

    fn strena_new_collect_words(&self) {
        crate::ALLOCATOR.set_active(true);
        let mut words = strena::Interner::new();
        crate::ALLOCATOR.mark_point();
        for &word in WORDS {
            words.get_or_insert(word);
            crate::ALLOCATOR.mark_point();
        }
        crate::ALLOCATOR.set_active(false);
        println!("Loaded {} words", words.len());
    }

    fn strena_with_capacity_collect_words(&self) {
        crate::ALLOCATOR.set_active(true);
        let mut words = strena::Interner::with_capacity(strena::Capacity {
            symbols: WORDS.len(),
            bytes: WORDS.len() * 5, // google says average word length is 4.7
        });
        crate::ALLOCATOR.mark_point();
        for &word in WORDS {
            words.get_or_insert(word);
            crate::ALLOCATOR.mark_point();
        }
        crate::ALLOCATOR.set_active(false);
        println!("Loaded {} words", words.len());
    }
}
