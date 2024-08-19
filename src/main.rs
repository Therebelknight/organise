use std::collections::BTreeMap;

use clap::{Arg, ArgAction, Command};

fn main() {
    let m = Command::new("org")
        .author("Stephan")
        .version("0.1.0")
        .about("Organises file in alphabetical order")
        .arg(
            Arg::new("alphabetical")
                .short('a')
                .long("alpha")
                .action(ArgAction::SetTrue)
                .value_name("ORDER")
                .help("organise in alphabetical order"),
        )
        .arg(
            Arg::new("reverse")
                .short('r')
                .long("reverse")
                .action(ArgAction::SetTrue)
                .value_name("ORDER")
                .help("organise in reverse alphabetical order"),
        )
        .arg(
            Arg::new("length")
                .short('l')
                .long("length")
                .action(ArgAction::SetTrue)
                .value_name("ORDER")
                .help("organise from longest to shortest in alphabetical order"),
        )
        .arg(
            Arg::new("by3")
                .short('t')
                .long("by-third")
                .action(ArgAction::SetTrue)
                .value_name("ORDER")
                .help("organise by the third letter of each word"),
        )
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .num_args(0..)
                .required(true)
                .action(ArgAction::Append)
                .value_name("FILE")
                .help("the file you want to organise"),
        )
        .get_matches();
    let mut vals: Vec<_> = m.get_many::<String>("file").unwrap().collect();
    let a = m.ids().next().unwrap().as_str();
    if a == "alphabetical" {
        vals.sort();
        for i in vals {
            println!("{}", i)
        }
    } else if a == "reverse" {
        vals.sort();
        vals.reverse();
        for u in vals {
            println!("{}", u)
        }
    } else if a == "length" {
        let mut b2: BTreeMap<usize, String> = BTreeMap::new();
        for x in vals {
            if b2.contains_key(&x.len()) {
                let a = format!("{}\n{}", b2.get(&x.len()).unwrap(), x);
                b2.insert(x.len(), a);
            } else {
                b2.insert(x.len(), x.to_string());
            }
        }
        for x in b2 {
            if x.1.contains("\n") {
                let mut n: Vec<_> = x.1.split_terminator("\n").collect();
                n.sort();
                for o in n {
                    println!("{}", o.trim());
                }
            } else {
                println!("{}", x.1);
            }
        }
    } else if a == "by3" {
        let mut b: BTreeMap<&str, String> = BTreeMap::new();
        for l in vals {
            if b.contains_key(&l[2..3]) {
                let f = format!("{}\n{}", b.get(&l[2..3]).unwrap(), l);
                b.insert(&l[2..3], f);
            } else {
                b.insert(&l[2..3], l.to_string());
            }
        }
        for l in b {
            if l.1.contains("\n") {
                let mut q: Vec<_> = l.1.split_terminator("\n").collect();
                q.sort();
                for k in q {
                    println!("{}", k.trim());
                }
            } else {
                println!("{}", l.1);
            }
        }
    } else if a == "file" {
        panic!("File cannot be the first argument")
    }
}
