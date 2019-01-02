use array_tool::vec::{Intersect, Union};
use std::collections::HashSet;

fn main() {
    //let hello = split_tokens_lower("Hello, world!");
    //let rerro = split_tokens_lower("Hello, porld!");
    let hello = String::from("Hello, world!");
    let rerro = String::from("Hello, porld!");

    //let hello = split_tokens_lower("Ceasers Palace");
    //let rerro = split_tokens_lower("Caesars Palace");
    //let hello = String::from("Ceasers Palace");
    //let rerro = String::from("Caesars Palace");

    //let hello = split_tokens_lower("abcde");
    //let rerro = split_tokens_lower("abdcde");
    let sng = SimpleNgram {
        s1: &hello,
        s2: &rerro,
        n: 2,
        tsg: TwoSeqGram {
            xs: HashSet::new(),
            ys: HashSet::new(),
        },
    };
    println!("s1: {:?}", sng.s1);
    println!("s2: {:?}", sng.s2);

    let sim = sng.similarity();
    println!("score: {}", sim);

    println!("simpler ngrams...");
    let ng = ngrams(hello.clone(), rerro.clone(), 2);
    println!("simpler ngrams Gramset xs: {:?}", ng.xs);
    println!("simpler ngrams Gramset ys: {:?}", ng.ys);

    let score = jaccard_coeff(ng);
    println!("simpler ngrams score: {}", score);

    println!("simpler ngramsv...");
    let ngv = ngramsv(&hello, &rerro, 2);
    println!("simpler ngramsv GramSetv s1: {:?}", ngv.s1);
    println!("simpler ngramsv GramSetv s2: {:?}", ngv.s2);
    println!(
        "simpler ngramsv GramSetv intersect: {:?}, {}",
        ngv.intersect, ngv.intersect_len
    );
    println!(
        "simpler ngramsv GramSetv union: {:?}, {}",
        ngv.union, ngv.union_len
    );

    println!("simpler ngramsv GramSetv union: {:?}", ngv.union);
    println!("qgram q1: {:?}", ngv.qgram.q1);
    println!("qgram q2: {:?}", ngv.qgram.q2);

    let jc = ngv.jaccard_similarity();
    println!("ngramsv jaccard_similarity: {}", jc);
    let cs = ngv.cosine_similarity();
    println!("ngramsv cosine_similarity: {}", cs);

    let jd = ngv.jaccard_distance();
    println!("ngramsv jaccard_distance: {}", jd);
    let cd = ngv.cosine_distance();
    println!("ngramsv cosine_distance: {}", cd);
}

///Ngram is continuous sequence of n-items from a given sequence. The distance is the relative number of items between these two sequences.
///
/// References:
///
///	[N-Gram Similarity and Distance](https://webdocs.cs.ualberta.ca/~kondrak/papers/spire05.pdf)
///	[Wikipedia n-gram](https://en.wikipedia.org/wiki/N-gram)
///	[WolframAlpha n-gram](http://m.wolframalpha.com/input/?i=n-grams+%22n-gram+example+of+n-grams+in+wolfram+alpha%22&x=0&y=0)

struct TextSplitter;
#[allow(dead_code)]
impl TextSplitter {
    fn word_match(c: char) -> bool {
        match c {
            ' ' | ',' | '.' | '!' | '?' | ';' | '\'' | '"' | ':' | '\t' | '\n' | '(' | ')'
            | '-' => true,
            _ => false,
        }
    }
    fn word_to_tokens_match(c: char) -> bool {
        match c {
            ' ' | ',' | '.' | '(' | ')' | '-' => true,
            _ => false,
        }
    }
}
/// split_words takes borrowed str and splits on word_match
/// filtering out empty slots
#[allow(dead_code)]
fn split_words(t: &str) -> Vec<&str> {
    t.split(TextSplitter::word_match)
        .filter(|s| !s.is_empty())
        .collect()
}
/// split_words takes borrowed str and splits all
/// filtering out empty slots
#[allow(dead_code)]
fn split_tokens(t: &str) -> Vec<&str> {
    t.split("").filter(|s| !s.is_empty()).collect()
}
/// split_tokens_lower takes borrowed str
/// and uses custom splitter, collects, concats,
/// then lowercase each str, returning new String
#[allow(dead_code)]
fn split_tokens_lower(t: &str) -> String {
    // TODO make this &str
    t.split(TextSplitter::word_to_tokens_match)
        .collect::<Vec<&str>>()
        .concat()
        .split("")
        .map(|s| s.to_lowercase())
        .filter(|s| !s.is_empty()) //needed a split pads
        .collect::<Vec<String>>()
        .concat()
}

#[derive(Eq, PartialEq, Debug)]
struct TwoSeqGram<'a> {
    xs: HashSet<&'a str>,
    ys: HashSet<&'a str>,
}

struct SimpleNgram<'a> {
    tsg: TwoSeqGram<'a>,
    s1: &'a str,
    s2: &'a str,
    n: usize,
}

impl<'a> SimpleNgram<'a> {
    fn build(mut self) -> SimpleNgram<'a> {
        let mut tsg = TwoSeqGram {
            xs: HashSet::with_capacity(self.s1.len() as usize),
            ys: HashSet::with_capacity(self.s1.len() as usize),
        };

        for i in 0..((self.s1.len() - self.n) + 1) {
            tsg.xs.insert(&self.s1[i..(i + self.n)]);
        }

        for i in 0..((self.s2.len() - self.n) + 1) {
            tsg.ys.insert(&self.s2[i..(i + self.n)]);
        }
        println!("{:?}", tsg.xs);
        println!("{:?}", tsg.ys);
        self.tsg = tsg;
        self
    }

    ///jaccard_coefficient calculates similarity of two sets as intersection divided by union.
    fn jaccard_coefficient(&self) -> f64 {
        //psuedo: tsg.xs.intersection(tsg.ys).size / tsg.xs.union(tgs.ys).size
        let intersect: HashSet<_> = self.tsg.xs.intersection(&self.tsg.ys).collect();
        let union: HashSet<_> = self.tsg.xs.union(&self.tsg.ys).collect();
        (intersect.len() as f64) / (union.len() as f64)
    }

    /// similarity builds the two sequences grams and returns score; higher is more similar.
    pub fn similarity(self) -> f64 {
        self.build().jaccard_coefficient()
    }
}

#[derive(Eq, PartialEq, Debug)]
struct GramSet {
    xs: HashSet<String>,
    ys: HashSet<String>,
}

fn ngrams(s1: String, s2: String, n: usize) -> GramSet {
    let mut tsg = GramSet {
        xs: HashSet::with_capacity(s1.len() as usize),
        ys: HashSet::with_capacity(s1.len() as usize),
    };

    for i in 0..((s1.len() - n) + 1) {
        tsg.xs.insert(s1[i..(i + n)].to_string());
    }
    for i in 0..((s2.len() - n) + 1) {
        tsg.ys.insert(s2[i..(i + n)].to_string());
    }

    println!("{:?}", tsg.xs);
    println!("{:?}", tsg.ys);
    tsg
}

///jaccard_coeff: calculates jaccard coefficient similarity of two sets as intersection divided by union.
fn jaccard_coeff(tsg: GramSet) -> f64 {
    //psuedo: tsg.xs.intersection(tsg.ys).size / tsg.xs.union(tgs.ys).size
    let intersect: HashSet<_> = tsg.xs.intersection(&tsg.ys).collect();
    let union: HashSet<_> = tsg.xs.union(&tsg.ys).collect();
    (intersect.len() as f64) / (union.len() as f64)
}

struct Qgram {
    q1: Vec<usize>,
    q2: Vec<usize>,
}
struct GramSetv {
    s1: Vec<String>,
    s2: Vec<String>,
    s1_len: f64,
    s2_len: f64,
    intersect: Vec<String>,
    union: Vec<String>,
    intersect_len: f64,
    union_len: f64,
    qgram: Qgram,
}

fn ngramsv(string1: &str, string2: &str, n: usize) -> GramSetv {
    let mut tsg = GramSetv {
        s1: Vec::with_capacity(string1.len() as usize),
        s2: Vec::with_capacity(string2.len() as usize),
        s1_len: 0.0,
        s2_len: 0.0,
        intersect: Vec::new(),
        union: Vec::new(),
        intersect_len: 0.0,
        union_len: 0.0,
        qgram: Qgram {
            q1: Vec::new(),
            q2: Vec::new(),
        },
    };

    for i in 0..((string1.len() - n) + 1) {
        tsg.s1.push(string1[i..(i + n)].to_string());
    }
    for i in 0..((string2.len() - n) + 1) {
        tsg.s2.push(string2[i..(i + n)].to_string());
    }

    tsg.s1_len = tsg.s1.len() as f64;
    tsg.s2_len = tsg.s2.len() as f64;
    tsg.intersect = tsg.s1.intersect(tsg.s2.clone());
    tsg.union = tsg.s1.union(tsg.s2.clone());
    tsg.intersect_len = tsg.intersect.len() as f64;
    tsg.union_len = tsg.union.len() as f64;

    let cap = tsg.union_len as usize;
    tsg.qgram.q1 = Vec::with_capacity(cap);
    tsg.qgram.q2 = Vec::with_capacity(cap);
    for c in &tsg.union {
        if tsg.s1.contains(c) {
            tsg.qgram.q1.push(1)
        } else {
            tsg.qgram.q1.push(0)
        }
        if tsg.s2.contains(c) {
            tsg.qgram.q2.push(1)
        } else {
            tsg.qgram.q2.push(0)
        }
    }

    tsg
}

impl GramSetv {
    /// jaccard_distance: 1 - jaccard_similarity. higher score is less similar.
    fn jaccard_distance(&self) -> f64 {
        1.0 - GramSetv::jaccard_similarity(&self)
    }
    /// cosine_distance: 1 - cosine_similarity. higher socre is less similar.
    pub fn cosine_distance(&self) -> f64 {
        1.0 - GramSetv::cosine_similarity(&self)
    }

    /// jaccard_similarity: calculates jaccard coefficient, the similarity
    /// of two sets as intersection divided by union: J(X,Y) = |X∩Y| / |X∪Y|.
    /// higher score is more similar
    pub fn jaccard_similarity(&self) -> f64 {
        (&self.intersect_len / &self.union_len)
    }
    /// cosine_similarity:
    /// higher score is more similar
    pub fn cosine_similarity(&self) -> f64 {
        let s1s1_mul: f64 = self
            .qgram
            .q1
            .iter()
            .zip(self.qgram.q1.iter())
            .map(|(x, y)| (x * y) as f64)
            .sum();
        let s1s2_mul: f64 = self
            .qgram
            .q1
            .iter()
            .zip(self.qgram.q2.iter())
            .map(|(x, y)| (x * y) as f64)
            .sum();
        let s2s2_mul: f64 = self
            .qgram
            .q2
            .iter()
            .zip(self.qgram.q2.iter())
            .map(|(x, y)| (x * y) as f64)
            .sum();
        s1s2_mul / (s1s1_mul.sqrt() * s2s2_mul.sqrt())
    }
}

/*
fn qgram(&self) -> Qgram {
    let cap = self.union_len as usize;
    let mut qgram = Qgram {
        q1: Vec::with_capacity(cap),
        q2: Vec::with_capacity(cap),
    };

    for c in &self.union {
        if self.s1.contains(c) {
            qgram.q1.push(1)
        } else {
            qgram.q1.push(0)
        }
        if self.s2.contains(c) {
            qgram.q2.push(1)
        } else {
            qgram.q2.push(0)
        }
    }
    qgram
}
*/
