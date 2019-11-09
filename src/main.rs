use std::fs;
use std::str;
use std::io::{Read,Write};
use std::env;

use serde_json::{Result, Value};

struct PwFile {
    rootkey : Vec<u8>,
    clientkey : Vec<u8>, 
    pwfile : String,
    command : String,
    args: Vec<String>,
    
}

impl PwFile {
    fn new() -> PwFile {
        let rootkey = fs::read("/mnt/pw/root.key").unwrap();
        let clientkey = fs::read("/var/lib/misc/pwclient.key").unwrap();
        let pwfile  = String::from("/mnt/pw/pword");
        let args: Vec<String> = env::args().collect();
        let command = args[1].clone();
        PwFile{ rootkey, clientkey, pwfile, command, args }
    }

    fn run(&self) {
       match self.command.as_ref() {
           "get" => self.get(),
           "dump" => self.dump(),
           "load" => self.load(),
           _ => println!("unknown command"),
       }
    }

    fn get(&self) {
        let key = self.args[2].clone();
        let field = self.args[3].clone();
        let pwfile  = fs::read(&self.pwfile).unwrap();
        let raw = self.crypt(&pwfile);
        let jsonstr = str::from_utf8(&raw).unwrap();
        let v: Value = serde_json::from_str(jsonstr).unwrap();

        let result = String::from(v[key][field].as_str().unwrap());
        print!("{}", result);
    }

    fn dump(&self) {
        let pwfile  = fs::read(&self.pwfile).unwrap();
        let output = self.crypt(&pwfile);
        PwFile::write_stdout(&output);
    }

    fn load(&self) {
        let infile = PwFile::read_stdin();
        let encrypted = self.crypt(&infile);
        PwFile::write_stdout(&encrypted);
    }
    
    fn crypt(&self, pwfile: &Vec<u8>) -> Vec<u8> {
        let mut result = Vec::new();
        for i in 0..pwfile.len() {
            result.push(self.rootkey[i] ^ self.clientkey[i] ^ pwfile[i]); 
        }
        return result;
    }

    fn write_stdout(output: &Vec<u8>) {
        let stdout = std::io::stdout();
        let mut out = stdout.lock();
        out.write(&output).unwrap();
    }

    fn read_stdin() -> Vec<u8> {
        let mut infile = Vec::new();
        let stdin = std::io::stdin();
        let mut fin = stdin.lock();
        let cnt = fin.read_to_end(&mut infile).unwrap();
        infile
    }
}

fn main() {
    let pwfile = PwFile::new();
    pwfile.run();
}



/* 
 *
 * pw clip amazon.co.uk user
 * pw get password amazon.co.uk
 * pw get all amazon.co.uk
 * pw get disk home
 * pw dump > /mnt/ram/myfile
 * pw load < /mnt/ram/myfile
 
 pw <action> [<lookupkey> <attribute>] 

 clip - copy specified attribute value to clipboard
 get - output to stdout
 dump - output full file decrypted to stdout
 load - input file from stdin and encrypt

*/
