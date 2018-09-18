#[macro_use]
extern crate rustler;
#[macro_use]
extern crate lazy_static;
extern crate fst;
extern crate fst_levenshtein;

use fst::{IntoStreamer, Set, SetBuilder};
use fst_levenshtein::Levenshtein;
use rustler::resource::ResourceArc;
use rustler::{thread, Encoder, Env, Error, NifResult, Term};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Mutex;

mod atoms {
    rustler_atoms! {
        atom ok;
    }
}

struct SetResource(Mutex<Set>);

rustler_export_nifs! {
    "Elixir.FST",
    [
        ("build_set_from_file", 1, build_set_from_file),
        ("query", 3, query)
    ],
    Some(load)
}

fn load(env: Env, _info: Term) -> bool {
    resource_struct_init!(SetResource, env);
    true
}

fn build_set_from_file<'a>(caller: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let path: String = try!(args[0].decode());

    thread::spawn::<thread::ThreadSpawner, _>(caller, move |env| {
        let mut file_handle = File::open(path).unwrap();
        let mut set_builder = SetBuilder::memory();
        for word in BufReader::new(&file_handle).lines() {
            set_builder.insert(word.unwrap().to_lowercase()).unwrap();
        }
        let fst_bytes = set_builder.into_inner().unwrap();
        let set = Set::from_bytes(fst_bytes).unwrap();

        let resource = ResourceArc::new(SetResource(Mutex::new(set)));

        (atoms::ok(), resource).encode(env)
    });

    Ok(atoms::ok().encode(caller))
}

fn query<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<SetResource> = args[0].decode()?;
    let query: String = try!(args[1].decode());
    let distance: u32 = try!(args[2].decode());

    let set = match resource.0.try_lock() {
        Ok(guard) => guard,
        Err(_) => return Err(Error::BadArg),
    };

    let lev = Levenshtein::new(&query, distance).unwrap();

    let results = set.search(lev).into_stream().into_strs().unwrap();

    Ok(results.encode(env))
}
