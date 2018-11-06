extern crate reqwest;


mod util;

use std::io::{BufReader, BufRead, Write};
use self::util::uppercase_first;
use std::collections::BTreeSet;
use std::fs::{self, File};
use std::path::Path;
use std::mem;


fn main() {
    let root_dir = Path::new(".");

    let adjectives = words_first_adjectives().into_iter().chain(words_second_adjectives().into_iter()).map(uppercase_first).collect::<BTreeSet<_>>();
    let mut nouns = words_nouns();
    for n in &mut nouns {
        let mut t = String::new();
        mem::swap(&mut t, n);
        *n = uppercase_first(t);
    }
    let adverbs = words_first_adverbs().into_iter().chain(words_second_adverbs().into_iter()).map(uppercase_first).collect::<BTreeSet<_>>();

    let rust_root = root_dir.join("rust");
    fs::create_dir_all(&rust_root).unwrap();
    words_rust(&adjectives, &nouns, &adverbs, &rust_root);
}


fn words_rust<'w, Adj, N, Adv>(adjectives: Adj, nouns: N, adverbs: Adv, out_dir: &Path)
    where Adj: IntoIterator<Item = &'w String>,
          N: IntoIterator<Item = &'w String>,
          Adv: IntoIterator<Item = &'w String>
{
    let dest_path = out_dir.join("words.rs");
    let mut f = File::create(&dest_path).unwrap();

    f.write_all("/// A set of upper-case-first adjectives for random string gen.\n".as_bytes()).unwrap();
    f.write_all("pub static ADJECTIVES: &[&str] = &[\n".as_bytes()).unwrap();
    for adj in adjectives {
        f.write_all("   \"".as_bytes()).unwrap();
        f.write_all(adj.as_bytes()).unwrap();
        f.write_all("\",\n".as_bytes()).unwrap();
    }
    f.write_all("];\n".as_bytes()).unwrap();
    f.write_all("\n".as_bytes()).unwrap();
    f.write_all("/// A set of upper-case-first nouns for random string gen.\n".as_bytes()).unwrap();
    f.write_all("pub static NOUNS: &[&str] = &[\n".as_bytes()).unwrap();
    for noun in nouns {
        f.write_all("   \"".as_bytes()).unwrap();
        f.write_all(noun.as_bytes()).unwrap();
        f.write_all("\",\n".as_bytes()).unwrap();
    }
    f.write_all("];\n".as_bytes()).unwrap();
    f.write_all("\n".as_bytes()).unwrap();
    f.write_all("/// A set of upper-case-first adverbs for random string gen.\n".as_bytes()).unwrap();
    f.write_all("pub static ADVERBS: &[&str] = &[\n".as_bytes()).unwrap();
    for adv in adverbs {
        f.write_all("   \"".as_bytes()).unwrap();
        f.write_all(adv.as_bytes()).unwrap();
        f.write_all("\",\n".as_bytes()).unwrap();
    }
    f.write_all("];\n".as_bytes()).unwrap();
}

fn words_first_adjectives() -> Vec<String> {
    let mut currently = false;
    let mut coll = vec![];

    for l in BufReader::new(reqwest::Client::builder()
            .gzip(true)
            .build()
            .unwrap()
            .get("http://enchantedlearning.com/wordlist/adjectives.shtml")
            .send()
            .unwrap())
        .lines() {
        let l = l.unwrap();
        for l in l.split("\r") {
            let l = l.to_lowercase();

            if l == "</td></tr></table>" {
                currently = false;
            }

            if currently && !l.is_empty() {
                let l = l.replace("<br>", "");
                if !l.contains('<') && l.len() > 1 {
                    coll.push(l);
                }
            }

            if l.contains("<font size=+0>a</font>") {
                currently = true;
                continue;
            }
        }
    }

    coll
}

fn words_second_adjectives() -> Vec<String> {
    words_talkenglish("http://www.talkenglish.com/vocabulary/top-1500-nouns.aspx")
}

fn words_nouns() -> Vec<String> {
    BufReader::new(reqwest::Client::builder()
            .gzip(true)
            .build()
            .unwrap()
            .get("http://www.desiquintans.com/downloads/nounlist/nounlist.txt")
            .send()
            .unwrap())
        .lines()
        .map(Result::unwrap)
        .filter(|l| l.len() != 1)
        .collect()
}

fn words_first_adverbs() -> Vec<String> {
    words_talkenglish("http://www.talkenglish.com/vocabulary/top-250-adverbs.aspx")
}

fn words_second_adverbs() -> Vec<String> {
    let mut currently = false;
    let mut coll = vec![];

    for l in BufReader::new(reqwest::Client::builder()
            .gzip(true)
            .build()
            .unwrap()
            .get("https://www.espressoenglish.net/100-common-english-adverbs/")
            .send()
            .unwrap())
        .lines() {
        let l = l.unwrap();
        for l in l.split("\r") {
            let l = l.to_lowercase();

            if l.contains("100.") {
                currently = false;
            }

            if currently && !l.contains("div>") {
                let l = l.replace("<br />", "").replace("<p>", "").replace("</p>", "").replace("/>", "");
                coll.push(l.rsplitn(2, " ").next().unwrap().trim().to_string());
            }

            if l.contains("<p>1.") {
                currently = true;
                continue;
            }
        }
    }

    coll
}

fn words_talkenglish(url: &str) -> Vec<String> {
    BufReader::new(reqwest::Client::builder()
            .gzip(true)
            .build()
            .unwrap()
            .get(url)
            .send()
            .unwrap())
        .lines()
        .map(Result::unwrap)
        .filter(|l| l.contains(r#"<a href="/how-to-use/"#))
        .filter_map(|l| l.replace("</a>", "").replace('>', "\n").split("\n").skip(1).next().map(|s| s.to_string()))
        .skip(1)
        .filter(|l| l.len() != 1)
        .collect()
}
