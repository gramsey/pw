# pw
A key small key management command line utitlity that can be used to make a usbkey, and password store. 

## Use case
This is intended to be used with a usb stick or similar device to securely store passwords or similar secrets for use with home devices that you own. 
You will need to create a few shell scripts to make it useful/convenient to use. Instructions are only for linux for this early version.

## Design 

There is a client.key stored on the device (say) your PC, the encyrpted password file and the root.key is stored on (say) a usb stick. 
When you want to access your passwords, you plug in and mount the usb stick. Call the software to retrieve the password, then unmount/remove the usbkey stick

This has the following features
1) If you loose your usbkey, anyone who finds it will not be able to decrypt and discover any of your passwords. 
2) If the usbkey is not plugged in and mounted on the device, there is no way to obtain the passwords from the PC.
3) will be able to access the same passwords accross mulitple devices (by copying the pwclient.key to that device). 

## Encryption
Instead of using GPG, AES or other 'fancy' encryption algorthms, it just uses XOR as follows :-

(client.key byte) XOR (root.key byte) XOR (unencrypted byte) -> (encrypted byte). 

to decrypt is the same 

(client.key byte) XOR (root.key byte) XOR (encrypted byte) -> (deencrypted byte). 

It gets away with this simple mechanism because of the specific nature of our use-case. You arn't trying to communicate information with someone else remotely, and the content to be encrypted is small, and rarely changes. Turns out in this situation, the humble XOR actually works very well indeed.
(I would however stongly suggest your own homework before applying a similar algorythm in other programs, there are many reasons why XOR is not the right choice for other situations). 




## build
This is a rust application, so build it with cargo like any other.
cargo build --release 
the program will be put in targets/release/ folder


## Setup
First you need to mount a usbstick on /mnt/pw 
```
mount /mnt/pw /dev/sdd1
mkdir /mnt/pw/bin
cp <where you built it>/targets/release/pw /mnt/pw/bin/
```
create two 1 meg random files for keys (size can vary but should be at least as big as your password file)
```
dd if=/dev/urandom of=/mnt/pw/root.key bs=1M count=1
dd if=/dev/urandom of=/var/lib/misc/pwclient.key bs=1M count=1
```

Then create your password file as a valid json file (i would create this file on a ram disk) 
```JSON
{
    "mywebsite" : { "username" : "bob", "password" : "SuperSecret123" },
    "google" : { "username" : "bob@gmail.com", "password" : "ILikeBunnyRabbits" } 
}
```

to encrypt your file and load it (if you saved it in /mnt/ram/mypasswords) :- 
```
/mnt/bin/pw load < /mnt/ram/mypasswords > /mnt/pw/pword
```

## usage 
to retrieve the "password" for "mywebsite" :-
```
/mnt/pw/bin/pw get mywebsite password 
```

to decrypt and view whole password file 
```
/mnt/pw/bin/pw dump
```










