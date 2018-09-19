#[macro_use]
extern crate rustler;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate tantivy;

use rustler::resource::ResourceArc;
use rustler::{thread, Encoder, Env, Error, NifResult, Term};

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Mutex;

use tantivy::collector::TopCollector;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::tokenizer::NgramTokenizer;
use tantivy::Index;

mod atoms {
    rustler_atoms! {
        atom ok;
    }
}

struct Tantivy {
    index: Index,
    query_parser: QueryParser,
}

struct TantivyResource(Mutex<Tantivy>);

rustler_export_nifs! {
    "Elixir.Tantivy",
    [
        ("build_index_from_file", 1, build_index_from_file),
        ("query", 2, query),
    ],
    Some(load)
}

fn load(env: Env, _info: Term) -> bool {
    resource_struct_init!(TantivyResource, env);
    true
}

fn build_index_from_file<'a>(caller: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let path: String = try!(args[0].decode());

    thread::spawn::<thread::ThreadSpawner, _>(caller, move |env| {
        let file_handle = File::open(path).unwrap();

        let mut schema_builder = SchemaBuilder::default();

        let word_indexing = TextFieldIndexing::default()
            .set_tokenizer("ngram3")
            .set_index_option(IndexRecordOption::WithFreqsAndPositions);

        let word_options = TextOptions::default()
            .set_indexing_options(word_indexing)
            .set_stored();

        schema_builder.add_text_field("word", word_options);
        let schema = schema_builder.build();

        let index = Index::create_in_ram(schema.clone());

        index
            .tokenizers()
            .register("ngram3", NgramTokenizer::new(3, 3, false));

        let mut index_writer = index.writer(10_000_000).unwrap();
        let schema_word = schema.get_field("word").unwrap();

        for word in BufReader::new(&file_handle).lines() {
            index_writer.add_document(doc!(schema_word => word.unwrap().to_lowercase()));
        }

        index_writer.commit().unwrap();
        index.load_searchers().unwrap();

        let query_parser = QueryParser::for_index(&index, vec![schema_word]);

        let resource = ResourceArc::new(TantivyResource(Mutex::new(Tantivy {
            index,
            query_parser,
        })));

        (atoms::ok(), resource).encode(env)
    });

    Ok(atoms::ok().encode(caller))
}

fn query<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<TantivyResource> = args[0].decode()?;
    let query: String = args[1].decode()?;

    let tantivy = match resource.0.try_lock() {
        Ok(tantivy) => tantivy,
        Err(_) => return Err(Error::BadArg),
    };

    let query = tantivy.query_parser.parse_query(&query).unwrap();
    let mut top_collector = TopCollector::with_limit(5);

    let searcher = tantivy.index.searcher();

    searcher.search(&*query, &mut top_collector).unwrap();

    let doc_addresses = top_collector.docs();

    let words: Vec<Term<'a>> = doc_addresses
        .into_iter()
        .map(|doc_addr| {
            searcher
                .doc(doc_addr)
                .unwrap()
                .get_first(Field(0))
                .unwrap()
                .text()
                .unwrap()
                .encode(env)
        }).collect();

    Ok(words.encode(env))
}
