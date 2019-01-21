use array_tool::vec::{Intersect, Union};
//use nalgebra::DVector;
//use nalgebra::DMatrix;
use ndarray::Array1;
use std::collections::HashSet;

fn main() {
    //let hello = split_tokens_lower("Hello, world!");
    //let rerro = split_tokens_lower("Hello, porld!");
    //let hello = String::from("Hello, world!");
    //let rerro = String::from("Hello, porld!");

    //let hello = split_tokens_lower("Ceasers Palace");
    //let rerro = split_tokens_lower("Caesars Palace");
    //let hello = String::from("Ceasers Palace");
    //let rerro = String::from("Caesars Palace");

    let hello = split_tokens_lower("abcde");
    let rerro = split_tokens_lower("abdcde");
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
    let ngv = build(&hello, &rerro, 2);
    println!("simpler ngramsv GramSetv s1: {:?}", ngv.s1);
    println!("simpler ngramsv GramSetv s2: {:?}", ngv.s2);
    println!("qgram q1: {:?}", ngv.qgram.q1);

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

    println!("");
    println!("");
    println!("ngramnsa...");
    let ngv = ngramsna(&hello, &rerro, 1);
    println!("simpler ngramsna GramSetv s1: {:?}", ngv.s1);
    println!("simpler ngramsna GramSetv s2: {:?}", ngv.s2);
    println!("qgram q1: {:?}", ngv.qgram.q1);

    println!(
        "ngramsna GramSetNA intersect: {:?}, {}",
        ngv.intersect, ngv.intersect_len
    );
    println!(
        "ngramsna GramSetv union: {:?}, {}",
        ngv.union, ngv.union_len
    );

    println!("simpler ngramsna GramSetNA union: {:?}", ngv.union);
    println!("qgram q1: {}", ngv.qgram.q1);
    println!("qgram q2: {}", ngv.qgram.q2);

    let jc = ngv.jaccard_similarity();
    println!("ngramsna jaccard_similarity: {}", jc);
    let cs = ngv.cosine_similarity();
    println!("ngramsna cosine_similarity: {}", cs);

    let jd = ngv.jaccard_distance();
    println!("ngramsna jaccard_distance: {}", jd);
    let cd = ngv.cosine_distance();
    println!("ngramsna cosine_distance: {}", cd);

    let jwd = ngv.jaro_winkler_distance(1.0, 2.0, 0.1);
    println!("ngramsna jaro_winkler_distance: {}", jwd);

    let jjwd = jaro_winkler_distance(&hello, &rerro, 0.1);
    println!("(str,str) jaro_winkler_distance: {}", jjwd);

    let jjjwd = jaro_winkler_similarity(&hello, &rerro, 0.1);
    println!("(str,str) jaro_winkler_similarity: {}", jjjwd);
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

pub struct NGram {
    // TODO: roll in nalgebra or ndarry instead of using vectors
    pub s1: Vec<String>,
    // TODO: roll in nalgebra or ndarry instead of using vectors
    pub s2: Vec<String>,
    pub s1_len: f64,
    pub s2_len: f64,
    // TODO: roll in nalgebra or ndarry instead of using vectors
    pub intersect: Vec<String>,
    // TODO: roll in nalgebra or ndarry instead of using vectors
    pub union: Vec<String>,
    pub intersect_len: f64,
    pub union_len: f64,
    pub qgram: Qgram,
}
pub struct Qgram {
    pub q1: Vec<usize>,
    pub q2: Vec<usize>,
}

pub fn build(string1: &str, string2: &str, n: usize) -> NGram {
    let mut tsg = NGram {
        // TODO: roll in nalgebra or ndarry instead of using vectors
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

    // TODO: roll in nalgebra or ndarry instead of using vectors
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

impl NGram {
    /// jaccard_distance: 1 - jaccard_similarity. higher score is less similar.
    pub fn jaccard_distance(&self) -> f64 {
        1.0 - NGram::jaccard_similarity(&self)
    }
    /// cosine_distance: 1 - cosine_similarity. higher socre is less similar.
    pub fn cosine_distance(&self) -> f64 {
        1.0 - NGram::cosine_similarity(&self)
    }

    /// jaccard_similarity: calculates jaccard coefficient, the similarity
    /// of two sets as intersection divided by union: J(X,Y) = |X∩Y| / |X∪Y|.
    /// higher score is more similar
    pub fn jaccard_similarity(&self) -> f64 {
        (&self.intersect_len / &self.union_len)
    }
    /// cosine_similarity:
    /// higher score is more similar
    ///
    // TODO: roll in nalgebra or ndarry instead of using vectors
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

///jaccard_coeff: calculates jaccard coefficient similarity of two sets as intersection divided by union.
fn jaccard_coeff(tsg: GramSet) -> f64 {
    //psuedo: tsg.xs.intersection(tsg.ys).size / tsg.xs.union(tgs.ys).size
    let intersect: HashSet<_> = tsg.xs.intersection(&tsg.ys).collect();
    let union: HashSet<_> = tsg.xs.union(&tsg.ys).collect();
    (intersect.len() as f64) / (union.len() as f64)
}
//type QgramVec = DMatrix<f64>;
type QgramVec = Array1<f64>;

struct QgramNA {
    //pub q1: DMatrix<f64>,
    //pub q2: DMatrix<f64>,
    q1: QgramVec,
    q2: QgramVec,
}
struct GramSetNA {
    N: usize,
    string1: String,
    string2: String,
    s1: Vec<String>,
    s2: Vec<String>,
    sv1_len: f64,
    sv2_len: f64,
    intersect: Vec<String>,
    union: Vec<String>,
    intersect_len: f64,
    union_len: f64,
    qgram: QgramNA,
}

fn ngramsna(string1: &str, string2: &str, n: usize) -> GramSetNA {
    let mut sa1 = Vec::with_capacity(string1.len() as usize);
    let mut sa2 = Vec::with_capacity(string2.len() as usize);

    for i in 0..((string1.len() - n) + 1) {
        sa1.push(string1[i..(i + n)].to_string());
    }
    for i in 0..((string2.len() - n) + 1) {
        sa2.push(string2[i..(i + n)].to_string());
    }

    let sa1_len = sa1.len() as f64;
    let sa2_len = sa2.len() as f64;
    let intersect = sa1.intersect(sa2.clone());
    let union = sa1.union(sa2.clone());
    let int_len = intersect.len() as f64;
    let un_len = union.len() as f64;

    let mut qv1: Vec<f64> = Vec::with_capacity(un_len as usize);
    let mut qv2: Vec<f64> = Vec::with_capacity(un_len as usize);
    for c in &union {
        if sa1.contains(c) {
            qv1.push(1.0);
        } else {
            qv1.push(0.0);
        }

        if sa2.contains(c) {
            qv2.push(1.0);
        } else {
            qv2.push(0.0);
        }
    }

    GramSetNA {
        N: n,
        string1: string1.to_string(),
        string2: string2.to_string(),
        s1: sa1,
        s2: sa2,
        sv1_len: sa1_len,
        sv2_len: sa2_len,
        intersect: intersect,
        union: union,
        intersect_len: int_len,
        union_len: un_len,
        qgram: QgramNA {
            //q1: DMatrix::from_vec(1, qv1.len(), qv1),
            //q2: DMatrix::from_vec(1, qv2.len(), qv2),
            q1: QgramVec::from_vec(qv1),
            q2: QgramVec::from_vec(qv2),
        },
    }
}

fn jaro_winkler_func(a: &f64, b: &f64, m: &f64, t: f64) -> f64 {
    1.0 - (1.0 / 3.0) * (m / a + m / b + (m - t) / m)
}

impl GramSetNA {
    pub fn jaro_winkler_distance(&self, t: f64, l: f64, p: f64) -> f64 {
        if self.N == (1 as usize) {
            return jaro_winkler_func(&self.sv1_len, &self.sv2_len, &self.union_len, t)
                * (1.0 - l * p);
        }
        let new = ngramsna(&self.string1, &self.string2, 1);
        jaro_winkler_func(&new.sv1_len, &new.sv2_len, &new.union_len, t) * (1.0 - l * p)
    }
    /// jaccard_distance: 1 - jaccard_similarity. higher score is less similar.
    fn jaccard_distance(&self) -> f64 {
        1.0 - GramSetNA::jaccard_similarity(&self)
    }
    /// cosine_distance: 1 - cosine_similarity. higher socre is less similar.
    pub fn cosine_distance(&self) -> f64 {
        1.0 - GramSetNA::cosine_similarity(&self)
    }

    /// jaccard_similarity: calculates jaccard coefficient, the similarity
    /// of two sets as intersection divided by union: J(X,Y) = |X∩Y| / |X∪Y|.
    /// higher score is more similar
    pub fn jaccard_similarity(&self) -> f64 {
        (&self.intersect_len / &self.union_len)
    }

    /// cosine_similarity:
    /// higher score is more similar
    /// cos_sim(a,b) = sum(a .* b) / (sqrt(sum(a .* a)) * sqrt(sum(b .* b)))
    pub fn cosine_similarity(&self) -> f64 {
        let a = &self.qgram.q1;
        let b = &self.qgram.q2;
        // with ndarray
        (a * b).sum() / (((a * a).sum()).sqrt() * ((b * b).sum()).sqrt())
        /*  WITH nalgebra...
        a.component_mul(b).iter().sum::<f64>()
            / ((a.component_mul(a).iter().sum::<f64>()).sqrt()
                * (b.component_mul(b).iter().sum::<f64>()).sqrt())
                */
    }
    pub fn jaro_winkler_similarity(&self, t: f64, l: f64, p: f64) -> f64 {
        1.0 - GramSetNA::jaro_winkler_distance(&self, t, l, p)
    }
}

use std::cmp::{max, min};
// jaro_func(A,B.m,t,l,p) = f(A,B,m,t) * (1 - l*p)
/// a is length of first string
/// b is length of second string
/// m is matching chars, the number of shared symbols
/// t is number of needed transpositions fo shared symbols
/// l is the length of common prefix, the number of symbols at beginning before first mismatch (max is 4)
/// p is the prefix scale, ranges between [0,0.25], gives more favorable ratings to strings that match from the beginning for a set prefix length l.
fn jaro_func(a: f64, b: f64, m: f64, t: f64) -> f64 {
    //(1.0 / 3.0) * (m / a + m / b + (m - t) / m)
    (1.0 / 3.0) * (m / a + m / b + (m - t) / m)
}

/// jaro_distance: higher score is less similar.
/// m is matching chars, the number of shared symbols
/// t is number of needed transpositions fo shared symbols
/// l is the length of common prefix, the number of symbols at beginning before first mismatch (max is 4)
/// p is the prefix scale, ranges between [0,0.25], gives more favorable ratings to strings that match from the beginning for a set prefix length l.
pub fn jaro(s1: &str, s2: &str) -> f64 {
    if s1 == s2 {
        return 1.0;
    }
    let s1_char_count = s1.chars().count();
    let s2_char_count = s2.chars().count();
    if s1_char_count == 0 || s2_char_count == 0 || (s1_char_count == 1 && s2_char_count == 1) {
        return 0.0;
    }
    let (mut t, mut m) = (0.0, 0.0);
    let mut s2_index: usize = 0;
    let mut sa2: Vec<u8> = Vec::with_capacity(s2_char_count as usize);

    for _i in 0..s2_char_count {
        sa2.push(0)
    }
    let window = (max(s1_char_count, s2_char_count) / 2) - 1;
    for (i, a) in s1.chars().enumerate() {
        let min_limit = if i > window { max(0, i - window) } else { 0 };
        let max_limit = min(s2_char_count - 1, i + window);
        if min_limit > max_limit {
            continue;
        }

        for (j, b) in s2.chars().enumerate() {
            if min_limit <= j && j <= max_limit && a == b && sa2[j] == 0 {
                sa2[j] = 1;
                m += 1.0;
                if j < s2_index {
                    t += 1.0;
                }
                s2_index = j;
                break;
            }
        }
    }

    if m == 0.0 {
        return 0.0;
    }
    jaro_func(s1_char_count as f64, s2_char_count as f64, m, t)
}

pub fn jaro_winkler_distance(s1: &str, s2: &str, mut p: f64) -> f64 {
    let mut l = s1
        .chars()
        .zip(s2.chars())
        .take_while(|&(s1_char, s2_char)| s1_char == s2_char)
        .count() as f64;
    if l > 4.0 {
        l = 4.0;
    };
    if p > 0.25 {
        p = 0.25
    }

    (1.0 - jaro(s1, s2)) * (1.0 - l * p)
}

pub fn jaro_winkler_similarity(s1: &str, s2: &str, p: f64) -> f64 {
    1.0 - jaro_winkler_distance(s1, s2, p)
}
