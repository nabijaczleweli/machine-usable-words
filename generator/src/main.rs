extern crate reqwest;
#[macro_use]
extern crate clap;


mod options;
mod util;

use self::util::{PolyWrite, uppercase_first};
use std::io::{BufReader, BufRead, Write};
use std::collections::BTreeSet;
use self::options::Options;
use std::fs::{self, File};
use std::path::Path;


fn main() {
    let opts = Options::parse();

    let adjectives = words_first_adjectives().into_iter().chain(words_second_adjectives().into_iter()).map(uppercase_first).collect::<BTreeSet<_>>();
    let nouns = words_first_nouns().into_iter().chain(words_second_nouns().into_iter()).map(uppercase_first).collect::<BTreeSet<_>>();
    let adverbs = words_first_adverbs()
        .into_iter()
        .chain(words_second_adverbs().into_iter())
        .chain(words_third_adverbs().into_iter())
        .map(uppercase_first)
        .collect::<BTreeSet<_>>();

    let rust_root = opts.output_dir.1.join("rust");
    fs::create_dir_all(&rust_root).unwrap();
    words_rust(&adjectives, &nouns, &adverbs, &rust_root);

    let raw_root = opts.output_dir.1.join("raw");
    fs::create_dir_all(&raw_root).unwrap();
    words_raw(&adjectives, &nouns, &adverbs, &raw_root);
}


fn words_rust<'w, Adj, N, Adv>(adjectives: Adj, nouns: N, adverbs: Adv, out_dir: &Path)
    where Adj: IntoIterator<Item = &'w String>,
          N: IntoIterator<Item = &'w String>,
          Adv: IntoIterator<Item = &'w String>
{
    let mut words_f = File::create(out_dir.join("words.rs")).unwrap();

    {
        let adjectives_f = File::create(out_dir.join("adjectives.rs")).unwrap();
        let mut files = PolyWrite(&mut words_f, adjectives_f);

        files.write_all("/// A set of upper-case-first adjectives for random string gen.\n".as_bytes()).unwrap();
        files.write_all("pub static ADJECTIVES: &[&str] = &[\n".as_bytes()).unwrap();
        for adj in adjectives {
            files.write_all("   \"".as_bytes()).unwrap();
            files.write_all(adj.as_bytes()).unwrap();
            files.write_all("\",\n".as_bytes()).unwrap();
        }
        files.write_all("];\n".as_bytes()).unwrap();
    }
    words_f.write_all("\n".as_bytes()).unwrap();

    {
        let nouns_f = File::create(out_dir.join("nouns.rs")).unwrap();
        let mut files = PolyWrite(&mut words_f, nouns_f);

        files.write_all("/// A set of upper-case-first nouns for random string gen.\n".as_bytes()).unwrap();
        files.write_all("pub static NOUNS: &[&str] = &[\n".as_bytes()).unwrap();
        for noun in nouns {
            files.write_all("   \"".as_bytes()).unwrap();
            files.write_all(noun.as_bytes()).unwrap();
            files.write_all("\",\n".as_bytes()).unwrap();
        }
        files.write_all("];\n".as_bytes()).unwrap();
    }
    words_f.write_all("\n".as_bytes()).unwrap();

    {
        let adverbs_f = File::create(out_dir.join("adverbs.rs")).unwrap();
        let mut files = PolyWrite(&mut words_f, adverbs_f);

        files.write_all("/// A set of upper-case-first adverbs for random string gen.\n".as_bytes()).unwrap();
        files.write_all("pub static ADVERBS: &[&str] = &[\n".as_bytes()).unwrap();
        for adv in adverbs {
            files.write_all("   \"".as_bytes()).unwrap();
            files.write_all(adv.as_bytes()).unwrap();
            files.write_all("\",\n".as_bytes()).unwrap();
        }
        files.write_all("];\n".as_bytes()).unwrap();
    }
}

fn words_raw<'w, Adj, N, Adv>(adjectives: Adj, nouns: N, adverbs: Adv, out_dir: &Path)
    where Adj: IntoIterator<Item = &'w String>,
          N: IntoIterator<Item = &'w String>,
          Adv: IntoIterator<Item = &'w String>
{
    let mut words_f = File::create(out_dir.join("words")).unwrap();

    {
        let adjectives_f = File::create(out_dir.join("adjectives")).unwrap();
        let mut files = PolyWrite(&mut words_f, adjectives_f);

        for adj in adjectives {
            files.write_all(adj.as_bytes()).unwrap();
            files.write_all("\n".as_bytes()).unwrap();
        }
    }
    words_f.write_all("\n".as_bytes()).unwrap();

    {
        let nouns_f = File::create(out_dir.join("nouns")).unwrap();
        let mut files = PolyWrite(&mut words_f, nouns_f);

        for noun in nouns {
            files.write_all(noun.as_bytes()).unwrap();
            files.write_all("\n".as_bytes()).unwrap();
        }
    }
    words_f.write_all("\n".as_bytes()).unwrap();

    {
        let adverbs_f = File::create(out_dir.join("adverbs")).unwrap();
        let mut files = PolyWrite(&mut words_f, adverbs_f);

        for adv in adverbs {
            files.write_all(adv.as_bytes()).unwrap();
            files.write_all("\n".as_bytes()).unwrap();
        }
    }
}


fn words_first_adjectives() -> Vec<String> {
    words_enchantedlearning("http://enchantedlearning.com/wordlist/adjectives.shtml")
}

fn words_second_adjectives() -> Vec<String> {
    words_talkenglish("http://www.talkenglish.com/vocabulary/top-1500-nouns.aspx")
}

fn words_first_nouns() -> Vec<String> {
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

fn words_second_nouns() -> Vec<String> {
    words_enchantedlearning("http://enchantedlearning.com/wordlist/nounandverb.shtml")
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

fn words_third_adverbs() -> Vec<String> {
    words_enchantedlearning("https://www.enchantedlearning.com/wordlist/adjectives.shtml")
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

fn words_enchantedlearning(url: &str) -> Vec<String> {
    let mut currently = false;
    let mut coll = vec![];

    for l in BufReader::new(reqwest::Client::builder()
            .gzip(true)
            .build()
            .unwrap()
            .get(url)
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
