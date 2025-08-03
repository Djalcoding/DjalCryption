#[allow(dead_code)]
pub mod encrypting {
    use std::fs;
    pub struct Query {
        path: String,
        pub contents: String,
        encrypted: bool,
    }

    impl Query {
        pub fn get_atbash(&self) -> String {
            let mut output = String::new();
            for character in self.contents.chars() {
                if character.is_ascii_alphabetic() {
                    if character.is_lowercase() {
                        output.push((b'z' - character as u8 + b'a') as char)
                    } else {
                        output.push((b'Z' - character as u8 + b'A') as char)
                    }
                } else {
                    output.push(character)
                };
            }
            output
        }

        fn encrpyt_ceasar(&self, key: u8) -> String {
            let mut out: String = String::new();
            for character in self.contents.chars() {
                let mut short_key = key;
                let mut ch = character;
                while short_key > 0 {
                    if ch as u8 == 255 {
                        ch = 0 as char;
                    } else {
                        ch = (ch as u8 + 1) as char;
                    }
                    short_key -= 1;
                }
                out.push(ch);
            }
            out
        }

        fn decrypt_ceasar(&self, key: u8) -> String {
            let mut out: String = String::new();
            for character in self.contents.chars() {
                let mut short_key = key;
                let mut ch = character;
                while short_key > 0 {
                    if ch as u8 == 0 {
                        ch = 255 as char;
                    } else {
                        ch = (ch as u8 - 1) as char;
                    }
                    short_key -= 1;
                }
                out.push(ch);
            }
            out
        }

        pub fn get_ceasar(&self, key: u8) -> String {
            if self.encrypted {
                self.decrypt_ceasar(key)
            } else {
                self.encrpyt_ceasar(key)
            }
        }

        pub fn get_encrpyted(&self, key: u8) -> String {
            Query::from_simple(self.get_ceasar(key), true).get_atbash()
        }

        pub fn get_decrypted(&self, key: u8) -> String {
            Query::from_simple(self.get_atbash(), true).get_ceasar(key)
        }

        pub fn from(path: String, encrypted: bool) -> Query {
            let file_content: Result<String, std::io::Error> = fs::read_to_string(&path);
            let contents = match file_content {
                Ok(text) => text,
                Err(e) => {
                    eprint!("Error : {e}");
                    panic!();
                }
            };
            Query {
                path,
                contents,
                encrypted,
            }
        }

        pub fn from_simple(contents: String, encrypted: bool) -> Query {
            Query {
                path: String::new(),
                contents,
                encrypted,
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::encrypting::Query;
    #[test]
    fn atbash_encrpyt() {
        let q = Query::from(
            String::from("/home/bert/Projects/RustProjects/encrypting/src/test_files/decrypted.txt"),
            false,
        );
        let out = q.get_atbash();

        assert_eq!(out, String::from("Svool, Dliow ! 1234567890\n"))
    }

    #[test]
    fn atbash_decrypt() {
        let q = Query::from(
            String::from("/home/bert/Projects/RustProjects/encrypting/src/test_files/encrpyted.txt"),
            true,
        );
        let out = q.get_atbash();

        assert_eq!(out, String::from("Hello, World ! 1234567890\n"))
    }

    #[test]
    fn encrypt() {
        let q = Query::from(
            String::from(
                "/home/bert/Projects/RustProjects/encrypting/src/test_files/long_decrypted.txt",
            ),
            false,
        );
        let e = Query::from_simple(q.get_encrpyted(142), false);
        assert_eq!(e.get_decrypted(142), q.contents);
    }
}
