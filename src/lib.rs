#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

mod trie;

use trie::Trie;

use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::str;

use itertools::Itertools;
use unidecode::unidecode;

const NBR_LETTER: usize = 26;

pub struct Paranagram {
    path_data: String,
    sacamot: Vec<String>,
    max_len: usize,
}

impl Paranagram {
    pub fn new(path_data: &str) -> io::Result<Self> {
        // Open and read the data file
        let path = Path::new(path_data);
        let mut file = File::open(&path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        let mut max_len = 0;

        // Parse the content of the data file to create an vec of all word
        let mut sacamot: Vec<String> = buffer
            .split("\n")
            .filter_map(|s| {
                if s.len() > max_len {
                    max_len = s.len()
                }
                let s = s.trim_end().to_owned();
                if s.len() != 0 {
                    Some(s)
                } else {
                    None
                }

            })
            .collect();

        // Return our Paranagram
        Ok(Self {
            path_data: path_data.to_owned(),
            sacamot,
            max_len,
        })
    }

    fn existing_anagrams(&self, sentence: &str) -> Vec<&String> {
        let trie = Trie::new_with_iter_and_maximun_deep(sentence.chars(), self.max_len);
        self.sacamot.iter().flat_map(|word| {
            if trie.existing(word.chars()) {
                Some(word)
            } else {
                None
            }
        }).collect::<Vec<&String>>()
    }

}

impl fmt::Debug for Paranagram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // We don't print the field "sacamot" because it's too large an uninteresting
        f.debug_struct("Paranagram")
            .field("path_data", &self.path_data)
            .field("max_len", &self.max_len)
            .finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn init() {
        let paranagram = Paranagram::new("data/word.txt");
        println!("{:?}", paranagram);
    }

    #[test]
    fn find_all_anagram_of_a_word() {
        let word = "parisien";
        let paranagram = Paranagram::new("data/word.txt").unwrap();
        let anagrams = paranagram.existing_anagrams(word);
        println!("{:?}", anagrams);
    }
}
// Well Tries would make it easier to check if the word exists. So if you put the whole dictionary in a trie:

// http://en.wikipedia.org/wiki/Trie

// then you can afterward take your word and do simple backtracking by taking a char and recursively checking if we can "walk" down the Trie with any combiniation of the rest of the chars (adding one char at a time). When all chars are used in a recursion branch and there was a valid path in the Trie, then the word exists.

// The Trie helps because its a nice stopping condition: We can check if the part of a string, e.g "Anag" is a valid path in the trie, if not we can break that perticular recursion branch. This means we don't have to check every single permutation of the characters.

// In pseudo-code

// checkAllChars(currentPositionInTrie, currentlyUsedChars, restOfWord)
//        if (restOfWord == 0)
//        {
//             AddWord(currentlyUsedChar)
//        }
//        else
//        {
//            foreach (char in restOfWord)
//            {
//                nextPositionInTrie = Trie.Walk(currentPositionInTrie, char)
//                if (nextPositionInTrie != Positions.NOT_POSSIBLE)
//                {
//                    checkAllChars(nextPositionInTrie, currentlyUsedChars.With(char), restOfWord.Without(char))
//                }
//            }
//        }

// Obviously you need a nice Trie datastructure which allows you to progressively "walk" down the tree and check at each node if there is a path with the given char to any next node...
